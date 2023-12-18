use std::process;

fn main() {
    if let Err(e) = evm::cli::run() {
        eprintln!("Error: {e:?}");
        process::exit(1);
    }
}
