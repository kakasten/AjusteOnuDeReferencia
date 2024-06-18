use std::process::{Command, Output};
use std::io::{self, Result};
use log::{info, error};

pub fn ping(ip: &String) -> Result<Output> {
    info!("Executando comando ping para o IP: {}", ip);
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "ping", "-n", "2", ip])
            .output()?
    } else {
        Command::new("ping")
            .args(&["-c", "2", ip])
            .output()?
    };

    let output_str = String::from_utf8_lossy(&output.stdout);

    if output.status.success() && output_str.contains("TTL=") {
        info!("Dispositivo está pingando com sucesso!");
        Ok(output)
    } else {
        let error_msg = "Dispositivo não está pingando!";
        error!("{}", error_msg);
        Err(io::Error::new(io::ErrorKind::Other, error_msg))
    }
}