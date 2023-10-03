use clearscreen::ClearScreen;

use crate::enums::operation_types::OperationType;
use crate::input::keyboard::wait_for_enter;

pub fn get_input(message: &str) -> String {
    let mut user_input_result: Result<usize, std::io::Error>;
    let mut user_input: String = String::new();

    loop {
        ClearScreen::default()
            .clear()
            .expect("Failed to clear screen");
        println!("{}", message);

        user_input_result = std::io::stdin().read_line(&mut user_input);

        match user_input_result {
            Ok(_value) => return user_input.trim().to_lowercase(),
            Err(error) => {
                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");
                println!("Failed to read input due to {}, please try again", error);
            }
        }
    }
}

pub fn get_f64(message: &str) -> f64 {
    let mut user_input: String;

    loop {
        user_input = get_input(message);

        match user_input.trim().parse() {
            Ok(value) => {
                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");
                return value;
            }
            Err(_error) => {
                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");
                println!("Failed to convert input into f64.");
                wait_for_enter();
                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");
            }
        }
    }
}

pub fn get_operation_type() -> OperationType {
    loop {
        let operation_type =
            get_input("Please enter the type of operation your would like to perform.");

        match operation_type.as_str() {
            "arithmatic" => return OperationType::ARITHMATIC,
            "trigonometric" => return OperationType::TRIGONOMETRIC,
            "statistical" => return OperationType::STATISTICAL,
            _ => (),
        }
    }
}
