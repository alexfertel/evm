use clap::{Parser, Subcommand};

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
pub struct Run {}

impl Run {
    fn run(&self) {}
}

pub fn run() -> eyre::Result<()> {
    let config = Cli::parse();
    match config.command {
        Commands::Assemble(command) => Ok(command.run()),
        Commands::Disassemble(command) => Ok(command.run()),
        Commands::Run(command) => Ok(command.run()),
    }
}
