use std::{
    env,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
};

pub fn get_binary_path() -> PathBuf {
    let root = env::current_exe()
        .unwrap()
        .parent()
        .expect("should be in the executable's directory")
        .to_path_buf();
    root.join("../evm")
}

/// Runs a command with the specified args.
pub fn cmd(binary_path: &PathBuf, command: &str, args: &[&str], stdin: Option<&str>) -> Output {
    let mut child = Command::new(binary_path)
        .arg(command)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn command");

    if let Some(input_data) = stdin {
        let stdin = child.stdin.as_mut().expect("Failed to open stdin");
        stdin
            .write_all(input_data.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output().expect("Failed to read stdout");

    if !output.stderr.is_empty() {
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }
    if output.status.success() {
        output
    } else {
        panic!("Command failed with exit code {}", output.status);
    }
}
