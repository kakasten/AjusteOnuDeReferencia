#[path = "../functions/read_int_from_keyboard.rs"]
mod read_int_from_keyboard;
#[path = "../functions/shutdown.rs"]
mod shutdown;

use log::info;
use read_int_from_keyboard::read_int_from_keyboard;

pub struct ConfigArgs<'a> {
    config_mode: &'a str,
}

impl<'a> ConfigArgs<'a> {
    pub fn new(args: &[String]) -> Self {
        let config_mode = if args.get(1).map_or(true, |arg| arg.is_empty()) {
            "pc_config"
        } else if args[1] == "vm_config" {
            "vm_config"
        } else {
            "pc_config"
        };

        ConfigArgs { config_mode }
    }

    pub fn get_config_mode(&self) -> &str {
        self.config_mode
    }

    pub fn finalize_code(&self) {
        println!("Digite 1 para poder ajustar outra ONU!");
        println!("Clique enter para fechar o programa e desligar a VM!");
        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        match input.trim().parse::<i32>() {
            Ok(n) => {
                if n == 1 {
                    info!("Reiniciando codigo!");
                    println!("Recomeçando codigo!");
                    std::thread::sleep(std::time::Duration::from_secs(3));
                } else {
                    info!("Codigo finalizado!");
                    info!("Desligando VM!");
                    shutdown::shutdown();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
            Err(_e) => {
                info!("Codigo finalizado!");
                info!("Desligando VM!");
                shutdown::shutdown();
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }

    pub fn cell_selection(&self) {
        if self.get_config_mode() == "vm_config" {
            println!("Digite qual pruduto esta ajustando!");
            println!("1 - ONT-01");
            println!("2 - ONT-02");
            println!("3 - ONT-03");
            println!("4 - ONU-01");
            println!("5 - ONU-02");
            println!("6 - ONU-03");

            info!("Usuário selecionando produto");
            info!("Esperando entrada do usuário...");

            loop {
                let mut selected_product: i32 =
                    read_int_from_keyboard("Digite somente os numeros acima, exemplo '2'!");
                match selected_product {
                    1 => {info!("Célula sendo ajustada ONT-01"); break;},
                    2 => {info!("Célula sendo ajustada ONT-02"); break;},
                    3 => {info!("Célula sendo ajustada ONT-03"); break;},
                    4 => {info!("Célula sendo ajustada ONU-01"); break;},
                    5 => {info!("Célula sendo ajustada ONU-02"); break;},
                    6 => {info!("Célula sendo ajustada ONU-03"); break;},
                    _ => {
                        println!("Valor digitado inválido, digite um número que aparece acima!")
                    }
                }
            }
        }
    }
}
