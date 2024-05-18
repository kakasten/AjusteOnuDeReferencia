#[path = "../domain/constants.rs"]
mod constants;

use std::{io::Read, net::TcpStream};
use ssh2::{Channel, Session};

pub fn ssh_connection(ip: &str, user: &str, password: &str, session: &mut Session) {
    match TcpStream::connect((ip, constants::PORT)) {
        Ok(tcp) => {
            session.set_tcp_stream(tcp);
            session.handshake().expect("Falha no handshake");
            session.userauth_password(user, password).expect("Falha na autenticação");
        },
        Err(e) => eprintln!("Falha ao conectar ao servidor ssh: {}", e),
    }
}

pub fn ssh_open_channel(session: &mut Session) -> Channel {
    session.channel_session().expect("Falha ao abrir canal")
}

pub fn ssh_close(channel: &mut Channel) {
    channel.send_eof().expect("Falha ao enviar EOF");
    channel.wait_close().expect("Falha ao fechar o canal");
}

pub fn ssh_command(channel: &mut Channel, command: &str) {
    channel.exec(command).expect("Falha ao executar o comando");
    let mut s = String::new();
    channel.read_to_string(&mut s).expect("Falha ao ler saída do comando");
    println!("{}", s);
}