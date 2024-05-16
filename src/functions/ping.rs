// ping command

use std::process::{Command, Output};

pub fn ping( ip: &String) -> Output{
    let output: Output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "ping", "-n", "2", ip])
            .output()
            .expect("Falha ao executar o comando")
    } else {
        Command::new("ping")
            .args(&["-c", "2", ip])
            .output()
            .expect("Falha ao executar o comando")
    };
    outputs
}

