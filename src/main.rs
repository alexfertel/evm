use std::process;

fn main() {
    if let Err(e) = smol_evm_rs::cli::run() {
        eprintln!("Error: {e:?}");
        process::exit(1);
    }
}
