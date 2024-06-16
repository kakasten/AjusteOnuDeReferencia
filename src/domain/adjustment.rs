#[path = "../functions/clear_terminal.rs"] mod clear_termianl;
#[path = "../functions/read_float_from_keyboard.rs"] mod read_float_from_keyboard;
#[path = "../functions/ping.rs"] mod ping;
mod access;
mod constants;

use crate::access::Access;
use clear_termianl::clear_terminal;
use read_float_from_keyboard::read_float_from_keyboard;
use ssh2::Session;

pub struct Adjustment {
    pub ip: String,
    pub channel_conf: Access,
    pub session: Session,
}

impl Adjustment {
    pub fn reboot(&mut self) {
        let commands: Vec<&str> = vec!["rtkbosa -b /etc/config/rtkbosa_k.bin", "reboot"];
        self.channel_conf.execute_commands(commands, &mut self.session);
    }

    pub fn get_value_register(&mut self) -> i32 {
        let commands: Vec<&str> = vec![
            "diag",
            "i2c bosa_calibrate get dev 0x51 reg 0xa8 count 1",
            "exit",
        ];
        let result_vec: Vec<String> = self.channel_conf.execute_commands(commands, &mut self.session);

        let hex_str = result_vec[0]
            .trim()
            .replace(" " , "")
            .replace("RTK.0>", "")
            .replace("i2cbosa_calibrategetdev0x51reg0xa8count1", "")
            .replace("\n", "");
    
        let hex_string = match i32::from_str_radix(&hex_str, 16) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("Falha ao converter número hexadecimal para inteiro: {}", e);
                -1
            }
        };
        hex_string
    }

    pub fn set_value_register(&mut self, value: i32) {
        let command = format!(
            "i2c bosa_calibrate set dev 0x51 reg 0xa8 count 1 data {:X}",
            value
        );
        let commands: Vec<&str> = vec!["diag", &command, "exit"];
        self.channel_conf.execute_commands(commands, &mut self.session);
    }

    pub fn check_value(&mut self) {
        let mut measured_value: f64 = read_float_from_keyboard();
        let mut  register_value: i32 = self.get_value_register();
        loop {
            if measured_value > constants::Max_REGISTER_VALUE {
                loop {
                    if register_value == -1 {
                        eprint!("Erro ao resgatar valor de register_value");    
                    }else if measured_value > constants::Max_REGISTER_VALUE {
                        register_value -= 4;
                        self.set_value_register(register_value);
                    } else if measured_value < constants::MIN_REGISTER_VALUE {
                        register_value += 1;
                        self.set_value_register(register_value);
                    } else {
                        break;
                    }
                    measured_value = read_float_from_keyboard();
                }
            } else if measured_value < constants::MIN_REGISTER_VALUE {
                loop {
                    if register_value == -1 {
                        eprint!("Erro ao resgatar valor de register_value");    
                    }else if measured_value > constants::Max_REGISTER_VALUE {
                        register_value -= 1;
                        self.set_value_register(register_value);
                    } else if measured_value < constants::MIN_REGISTER_VALUE {
                        register_value += 4;
                        self.set_value_register(register_value);
                    } else {
                        break;
                    }
                    measured_value = read_float_from_keyboard();
                }
            } else {
                break;
            }
        }
    }

    pub fn finalize_adjustment(&mut self) {
        clear_terminal();
        println!("Valor medido está na faixa!");
        println!("Aguarde reiniciar a onu!");
        println!("Reiniciando");
        self.reboot();
        std::thread::sleep(core::time::Duration::from_secs(5)); 
        loop {
            match ping::ping(&constants::IP.to_string()) {
                Ok(_output) => {
                    println!("Desconecte a Onu!");
                    std::thread::sleep(core::time::Duration::from_secs(10));
                    clear_terminal();
                    break;
                }
                Err(_e) => {
                    eprintln!("Aguarde...");
                    std::thread::sleep(constants::TIME_OUT);
                }
            }
        }
    }
}
