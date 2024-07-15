mod clear_terminal;
use log::{info, error};

pub fn read_int_from_keyboard(example_message : &str) -> i32 {
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        match input.replace(" ", "").trim().parse::<i32>() {
            Ok(value) => {
                info!("Entrada do usuário: {}", value);
                return value;
            }
            Err(_) => {
                clear_terminal::clear_terminal();
                error!("Entrada inválida do usuário: {}", input.trim());
                println!("{}", example_message);
            }
        }
    }
}
