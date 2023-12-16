use clap::{Parser, Subcommand};

use crate::{Contract, Interpreter};

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
pub struct Assemble {}

impl Assemble {
    fn run(&self) {}
}

/// Turn bytecode into assembly code.
#[derive(Parser, Debug)]
pub struct Disassemble {}

impl Disassemble {
    fn run(&self) {}
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
        Commands::Assemble(command) => Ok(command.run()),
        Commands::Disassemble(command) => Ok(command.run()),
        Commands::Run(command) => Ok(command.run()),
    }
}
