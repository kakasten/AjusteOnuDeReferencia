use std::{io::Read, net::TcpStream, sync::mpsc::channel};
use ssh2::{Channel, Session};

const PORT:u16 = 22;

pub fn ssh_connection( ip: String, user: &String, password: &String, mut session: Session) {
    match TcpStream::connect ((ip, PORT)) {
        Ok (_tcp) => {
            session.handshake().expect("Falha no handshake");
            session.userauth_password(&user, &password).expect("Falha na autenticação");

            if session.authenticated() {
                ssh_open_channel(session);
            } else {
                eprintln!("Falha na autenticação");
            }
        },
        Err( e ) => eprintln!("Falha ao conectar ao servidor ssh: {}", e),
    }
}

pub fn ssh_open_channel( session: Session) -> Channel{
    let mut channel = session.channel_session().expect("Falha ao abrir canal");
    channel
}

pub fn ssh_closed ( mut channel: Channel) {
    channel.send_eof().expect("Falha ao enviar EOF");
    channel.wait_close().expect("Falha ao fechar o canal");
}

pub fn ssh_command (mut channel: Channel, command: &String) {
    channel.exec(&command).expect("Falha ao executar o comando");
    let mut s = String::new();
    channel.read_to_string(&mut s).expect("Falha ao ler saida do comando");
    println!("{}", s);
}