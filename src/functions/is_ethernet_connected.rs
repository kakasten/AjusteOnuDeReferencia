use std::process::Command;

pub fn is_ethernet_connected() -> Result<bool, String> {
    let result = if cfg!(target_os = "linux") {
        let output = Command::new("sh")
            .arg("-c")
            .arg("ip link show | grep -E 'state UP'") // Lista todas as interfaces e verifica se alguma está UP
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.contains("state UP"))
    } else if cfg!(target_os = "windows") {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg("Get-NetAdapter | Where-Object { $_.Status -eq 'Up' } | Measure-Object | ForEach-Object { $_.Count }")
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;
        
        let count = String::from_utf8_lossy(&output.stdout).trim().parse::<usize>().unwrap_or(0);
        Ok(count > 0)
    } else {
        Ok(false)
    };
    
    result
}