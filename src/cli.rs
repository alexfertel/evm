use alloy_primitives::Bytes;
use clap::{Parser, Subcommand};
use clap_stdin::FileOrStdin;

use crate::{
    instructions::opcode::{opcode, OPCODE_NAMES, PUSH0},
    Contract, Interpreter,
};

/// This is the entry point to the executable.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
pub(crate) struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(name = "assemble")]
    Assemble(Assemble),
    #[command(name = "disassemble")]
    Disassemble(Disassemble),
    #[command(name = "run")]
    Run(Run),
}

/// Turn assembly code into bytecode.
#[derive(Parser, Debug)]
pub struct Assemble {
    /// The file to assemble or stdin.
    file: FileOrStdin,
}

impl Assemble {
    fn run(&self) -> eyre::Result<()> {
        // TODO: Avoid cloning.
        let contents = self.file.clone().contents()?;
        let mut bytes = Vec::with_capacity(contents.lines().count());
        for l in contents.lines() {
            let instruction = l.split_whitespace().collect::<Vec<_>>();
            bytes.push(opcode(instruction[0]));
            if instruction.len() > 1 {
                let data: Bytes = instruction[1].parse().expect("should be a hex number");
                for byte in data {
                    bytes.push(byte);
                }
            }
        }

        for byte in bytes {
            print!("{byte:0>2x}");
        }
        println!();

        Ok(())
    }
}

/// Turn bytecode into assembly code.
#[derive(Parser, Debug)]
pub struct Disassemble {
    /// The hex string representing the bytecode disassemble.
    code: String,
    /// Prefix instructions with their byte offset.
    ///
    /// 0004: PUSH1 0x01
    #[arg(long, default_value = "false")]
    verbose: bool,
}

impl Disassemble {
    fn run(&self) {
        let bytecode: Bytes = self.code.parse().unwrap();
        let mut i = 0;
        let mut instructions = Vec::with_capacity(bytecode.len());
        while i < bytecode.len() {
            let opcode = bytecode[i];
            if let Some(name) = OPCODE_NAMES[opcode as usize] {
                if name.starts_with("PUSH") {
                    let data_length = (opcode - PUSH0) as usize;
                    let bytes = bytecode.slice(i + 1..i + 1 + data_length);
                    instructions.push((i, format!("{name} {bytes}")));
                    i += data_length;
                } else {
                    instructions.push((i, name.to_owned()));
                }
            } else {
                instructions.push((i, "UNKNOWN".to_owned()));
            };
            i += 1;
        }

        for (byte, instruction) in instructions {
            if self.verbose {
                println!("{:0>4}: {}", byte, instruction);
            } else {
                println!("{}", instruction);
            }
        }
    }
}

/// Execute bytecode.
#[derive(Parser, Debug)]
pub struct Run {
    /// The hex string representing the bytecode to run.
    #[arg(long)]
    code: String,
    /// The hex string representing the calldata passed as input.
    #[arg(long)]
    calldata: String,
}

impl Run {
    fn run(&self) {
        let bytecode = self.code.parse().unwrap();
        let input = self.calldata.parse().unwrap_or_default();
        let contract = Box::new(Contract::new(
            alloy_primitives::Address::ZERO,
            bytecode,
            input,
        ));
        let interpreter = Interpreter::new(contract);
        match interpreter.execute() {
            Ok(bytes) => println!("{bytes}"),
            Err(e) => eprintln!("{e}"),
        }
    }
}

pub fn run() -> eyre::Result<()> {
    let config = Cli::parse();
    match config.command {
        Commands::Assemble(command) => command.run(),
        Commands::Disassemble(command) => Ok(command.run()),
        Commands::Run(command) => Ok(command.run()),
    }
}
