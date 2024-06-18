mod clear_terminal;
use log::{info, error};

pub fn read_float_from_keyboard() -> f64 {
    loop {
        info!("Esperando entrada do usuário...");
        println!("Digite o valor medido!");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        match input.replace(",", ".").trim().parse::<f64>() {
            Ok(value) => {
                info!("Entrada do usuário: {}", value);
                return value;
            }
            Err(_) => {
                clear_terminal::clear_terminal();
                error!("Entrada inválida do usuário: {}", input.trim());
                println!("Entrada inválida! Por favor, insira um número decimal exemplo: 2.15");
            }
        }
    }
}
