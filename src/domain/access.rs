#[path = "../domain/constants.rs"] mod constants;
#[path = "../functions/ssh_connection.rs"] mod ssh_connection;

use core::time;
use std::io::{Read, Write};
use std::thread::sleep;
use std::time::Instant;
use constants::TIME_OUT;
use ssh2::Session;
use ssh_connection::ssh_open_channel;

pub struct Access {
    pub channel: Option<ssh2::Channel>,
}

impl Access {
    pub fn execute_command_read_line(&mut self, command: &str, mut session: &mut Session) -> String {
        if let Some(ref mut channel) = self.channel {
            sleep(time::Duration::from_secs(3));
            if let Err(e) = channel.write_all(command.as_bytes()) {
                eprintln!("Falha ao enviar comando: {}", e);
                return String::new();
            }
            if let Err(e) = channel.write_all(b"\n") {
                eprintln!("Falha ao enviar nova linha: {}", e);
                return String::new();
            }

            let mut output: String = String::new();
            let mut buffer: [u8; 1024] = [0; 1024];
            let start_time = Instant::now();

            loop {
                if start_time.elapsed() >= time::Duration::from_secs(5) {
                    break;
                }

                match channel.read(&mut buffer) {
                    Ok(0) => {
                        println!("EOF ou canal fechado");
                        channel.send_eof();
                        channel.wait_close();
                        self.channel =  ssh_open_channel(&mut session);
                        break;
                    }
                    Ok(n) => {
                        output.push_str(&String::from_utf8_lossy(&buffer[..n]));
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(TIME_OUT);
                    }
                    Err(e) => {
                        eprintln!("Falha ao ler saída do comando: {}", e);
                        break;
                    }
                }
            }
            return output;
        } 
        String::new()
    }

    pub fn execute_command(&mut self, command: &str){
        if let Some(ref mut channel) = self.channel {
            if let Err(e) = channel.write_all(command.as_bytes()) {
                eprintln!("Falha ao enviar comando: {}", e);
            }
            if let Err(e) = channel.write_all(b"\n") {
                eprintln!("Falha ao enviar nova linha: {}", e);
            }
        }
    }

    pub fn execute_commands(&mut self, commands: Vec<&str>, session: &mut Session) -> Vec<String> {
        let mut results: Vec<String> = Vec::new();
        
        if let Some(ref mut channel) = self.channel {
            let _ = channel.shell();
            for command in commands {
                if command  == "i2c bosa_calibrate get dev 0x51 reg 0xa8 count 1"{
                    let output = self.execute_command_read_line(command, session);
                    results.push(output);
                } else {
                    self.execute_command(command);
                    sleep(time::Duration::from_secs(1));
                }
            }
        }
        results
    }
}
