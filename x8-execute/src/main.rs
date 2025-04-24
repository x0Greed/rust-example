use std::process::{Command, Stdio};

fn exec(cmd: &str) -> bool {
    let status = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match status {
        Ok(status) => status.success(),
        Err(_) => false,
    }
}

fn main() {
    let mkdir;

    if exec("mkdir test") {
        mkdir = 1;
        println!("mkdir: {}", mkdir);
    } else {
        mkdir = 0;
        println!("mkdir: {}", mkdir);
    }
}
