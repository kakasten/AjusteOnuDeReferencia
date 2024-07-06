use std::process::Command;
use log::{info, error};

pub fn ping(ip: &str) -> Result<bool, String> {
    info!("Executando comando ping para o IP: {}", ip);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "ping", "-n", "2", ip])
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?
    } else if cfg!(target_os = "linux") {
        Command::new("ping")
            .args(&["-c", "2", ip])
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?
    } else {
        let error_msg = "Sistema operacional não suportado!";
        error!("{}", error_msg);
        return Err(error_msg.to_string());
    };

    let output_str = String::from_utf8_lossy(&output.stdout);

    let ping_success = if cfg!(target_os = "windows") {
        output_str.contains("TTL=")
    } else if cfg!(target_os = "linux") {
        output_str.contains("ttl=")
    } else {
        false
    };

    if output.status.success() && ping_success {
        info!("Dispositivo está pingando com sucesso!");
        Ok(true)
    } else {
        let error_msg = "Dispositivo não está pingando!";
        error!("{}", error_msg);
        Err(error_msg.to_string())
    }
}
