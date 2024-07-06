#[path = "domain/access.rs"]
mod access;
#[path = "domain/adjustment.rs"]
mod adjustment;
#[path = "domain/user.rs"]
mod user;
#[path = "domain/constants.rs"]
mod constants;
#[path = "functions/is_ethernet_connected.rs"]
mod is_ethernet_connected;
#[path = "functions/ping.rs"]
mod ping;
#[path = "functions/ssh_connection.rs"]
mod ssh_connection;
#[path = "functions/clear_terminal.rs"]
mod clear_terminal;
#[path = "functions/shutdown.rs"]
mod shutdown;

use access::Access;
use adjustment::Adjustment;
use is_ethernet_connected::is_ethernet_connected;
use log::{error, info};
use ping::ping;
use ssh2::Session;
use ssh_connection::{ssh_connection, ssh_open_channel};
use std::env;
use user::User;

fn main() {
    let mut config_path = env::current_exe().unwrap();
    config_path.pop();
    config_path.push("config/log4rs.yaml");

    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

    'main: loop {
        let mut session = Session::new().expect("Falha ao criar sessão");
        let user: User = User {
            ip: constants::IP.to_string(),
            user_name: constants::USER_NAME.to_string(),
            password: constants::PASSWORD.to_string(),
        };

        info!("Iniciando programa...");
        println!("Por favor plugue a ONU de referencia!");

        'start: loop {
            match is_ethernet_connected() {
                Ok(true) => {
                    info!("Ethernet conectada");
                    println!("Tentando pingar onu de referencia");
                    match ping(&user.ip) {
                        Ok(_output) => {
                            info!("Ping bem-sucedido para a ONU de referência");
                            clear_terminal::clear_terminal();
                            match ssh_connection(
                                &user.ip,
                                &user.user_name,
                                &user.password,
                                &mut session,
                            ) {
                                Ok(_) => {
                                    info!("Conexão SSH estabelecida com sucesso");
                                    loop {
                                        let mut channel_access: Access = Access {
                                            channel: ssh_open_channel(&mut session),
                                        };
                                        if let Some(ref mut _channel) = channel_access.channel {
                                            info!("Canal SSH aberto com sucesso");
                                            let mut adjustment = Adjustment {
                                                ip: constants::IP.to_string(),
                                                channel_conf: channel_access,
                                                session: session,
                                            };
                                            loop {
                                                adjustment.check_value();
                                                adjustment.finalize_adjustment();
                                                break 'start;
                                            }
                                        }
                                    }
                                }
                                Err(e) => error!("Erro na conexão SSH: {:?}", e),
                            }
                        }
                        Err(e) => {
                            error!("Erro ao pingar o dispositivo: {}", e);
                        }
                    }
                }
                Ok(false) => println!("Por favor plugue a ONU de referencia!"),
                Err(e) => error!("Erro ao verificar a conexão Ethernet: {}", e),
            }
        }
        println!("Digite 1 para poder ajustar outra ONU!");
        println!("Clique enter para fechar o programa e desligar a VM!");
        let mut x = String::new();
        std::io::stdin().read_line(&mut x).expect("Tudo ok");
        if !(x == "1") {
            info!("Codigo finalizado!");
            info!("Desligando a VM");
            shutdown::shutdown();
        }
    }
}
