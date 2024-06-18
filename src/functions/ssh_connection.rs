#[path = "../domain/constants.rs"]
mod constants;

use std::net::TcpStream;
use ssh2::{Channel, Session};
use log::{info, error};

pub fn ssh_connection(ip: &str, user: &str, password: &str, session: &mut Session) -> Result<(), ssh2::Error> {
    info!("Estabelecendo conexão SSH para o IP {}...", ip);

    match TcpStream::connect((ip, constants::PORT)) {
        Ok(tcp) => {
            session.set_tcp_stream(tcp);
            session.handshake()?;
            session.userauth_password(user, password)?;
            info!("Conexão SSH estabelecida com sucesso para o IP {}", ip);
            Ok(())
        },
        Err(e) => {
            eprintln!("Falha ao conectar ao servidor ssh: {}", e);
            error!("Falha ao conectar ao servidor SSH: {}", e);
            Err(ssh2::Error::from_errno(ssh2::ErrorCode::Session(-1)))
        },
    }
}

pub fn ssh_open_channel(session: &mut Session) -> Option<Channel> {
    session.set_blocking(false);
    session.channel_session().ok()
}

