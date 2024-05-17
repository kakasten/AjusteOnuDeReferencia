// ping command

use std::process::{Command, Output};
use std::io::{self, Result};

pub fn ping(ip: &String) -> Result<Output> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "ping", "-n", "2", ip])
            .output()?
    } else {
        Command::new("ping")
            .args(&["-c", "2", ip])
            .output()?
    };

    if output.status.success() {
        Ok(output)
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "Comando ping falhou"))
    }
}
