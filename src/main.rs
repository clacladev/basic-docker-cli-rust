use anyhow::{Context, Result};
use std::{fs, os::unix, path::Path, process::Stdio};
use tempfile::TempDir;

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let command = &args[3];
    let command_args = &args[4..];

    // println!("Args: {:?}", args);
    // Args: ["/tmp/codecrafters-docker-target/release/docker-starter-rust", "run", "ubuntu:latest", "/usr/local/bin/docker-explorer", "ls", "src"]

    setup_new_sandbox()?;

    let output = std::process::Command::new(command)
        .args(command_args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;

    if output.status.success() {
        let std_out = std::str::from_utf8(&output.stdout)?;
        print!("{std_out}");
        let std_err = std::str::from_utf8(&output.stderr)?;
        eprint!("{std_err}");
    } else {
        std::process::exit(output.status.code().unwrap_or(1));
    }

    Ok(())
}

fn setup_new_sandbox() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;

    // Crate the sandbox directory
    fs::create_dir_all(temp_dir.path().join("usr/local/bin/"))?;
    fs::create_dir_all(temp_dir.path().join("dev/null"))?;

    // Copy binary to the sandbox
    let source_path = Path::new("/usr/local/bin/docker-explorer");
    let dest_path = temp_dir.path().join("usr/local/bin/docker-explorer");
    fs::copy(source_path, &dest_path)?;

    // Chroot into the sandbox
    unix::fs::chroot(temp_dir.path())?;
    std::env::set_current_dir("/")?;

    Ok(())
}
