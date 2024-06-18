use log::info;

pub fn clear_terminal() {
    #[cfg(windows)]
    {
        info!("Limpando terminal (Windows)");
        let _ = std::process::Command::new("cmd")
            .args(&["/c", "cls"])
            .status();
    }

    #[cfg(unix)]
    {
        info!("Limpando terminal (Unix/Linux)");
        let _ = std::process::Command::new("clear").status();
    }
}