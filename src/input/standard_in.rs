use crate::enums::operation_types::OperationType;
use crate::input::keyboard::wait_for_enter;
use crate::console_utility::clear::clear_console;

pub fn get_input(message: &str) -> String {
    let mut user_input_result: Result<usize, std::io::Error>;
    let mut user_input: String = String::new();

    clear_console();

    loop {
        println!("{}", message);

        user_input_result = std::io::stdin().read_line(&mut user_input);

        match user_input_result {
            Ok(_value) => {
                clear_console();
                return user_input.trim().to_lowercase();
            },
            Err(error) => {
                clear_console();
                println!("Failed to read input due to {}, please try again", error);
            }
        }
    }
}

pub fn get_f64(message: &str) -> f64 {
    loop {
        match get_input(message).parse() {
            Ok(value) => {
                return value;
            }
            Err(_error) => {
                println!("Failed to convert input into f64.");
                wait_for_enter();
            }
        }
    }
}

pub fn get_operation_type() -> OperationType {
    loop {
        match get_input("Please enter the type of operation.").as_str() {
            "arithmatic" =>    return OperationType::ARITHMATIC,
            "trigonometric" => return OperationType::TRIGONOMETRIC,
            "statistical" =>   return OperationType::STATISTICAL,
            _ => (),
        }
    }
}
