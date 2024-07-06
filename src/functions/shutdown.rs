use std::process::Command;
use log::{info, error};


pub fn shutdown () {
    info!("Executando comando para desligar VM");

    if cfg!(target_os = "windows") {
        Command::new("shutdown")
        .arg("/s")
        .arg("/t")
        .arg("0")
        .status()
        .unwrap();
    } else if cfg!(target_os = "linux") {
        Command::new("shutdown")
        .arg("now")
        .status()
        .unwrap();
    } else {
        let error_msg = "Sistema operacional não suportado!";
        println!("{}", error_msg);
        error!("{}", error_msg);
    }
}