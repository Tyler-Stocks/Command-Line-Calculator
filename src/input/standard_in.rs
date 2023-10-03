use crate::console_utility::clear::clear_console;
use crate::input::keyboard::wait_for_enter;

pub fn get_input(message: &str) -> String {
    let mut user_input_result: Result<usize, std::io::Error>;
    let mut user_input:        String = String::new();

    loop {
        clear_console();
        println!("{}", message);

        user_input_result =
            std::io::stdin().read_line(&mut user_input);

        match user_input_result {
            Ok(_value) => return user_input.trim().to_lowercase().to_string(),
            Err(error) => {
                clear_console();
                println!("Failed to read input due to {}, please try again", error);
            }
        }
    }
}


pub fn get_i128(message: &str) -> i128 {
    let mut user_input: String;

    loop {
        user_input = get_input(message);
    
        match user_input.trim().parse() {
            Ok(value) => {
                clear_console();
                return value;
            },
            Err(_error) => {
                println!("Failed to convert input into i128.");
                wait_for_enter();
                clear_console();
            },
        }
    }
}