use std::process::exit;

use crate::enums::{
    arithmatic_operations::ArithmaticOperation,
    operation_types::OperationType
};
use crate::input::keyboard::wait_for_enter;
use crate::console_utility::clear::clear_console;


/// Gets a user string input from standard in.
///
/// Note, the function will continue to loop until it is successfull.
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
                if user_input.trim().to_lowercase() == "q" { exit(0); }
                return user_input.trim().to_lowercase();
            },
            Err(error) => {
                clear_console();
                println!("Failed to read input due to {}, please try again", error);
            }
        }
    }
}


/// Gets a list of inputs, and returns them as a vector of Strings.
///
/// Prompts the user to enter f when they are finishde.
///
/// This function will never panic, and will always return.
pub fn get_inputs(message: &str) -> Vec<String> {
    let mut user_input:     String;
    let mut user_input_vec: Vec<String> = vec![];


    loop {
        user_input = get_input(message);

        if user_input == "f" { break; }

        user_input_vec.push(user_input)
    }

    return user_input_vec;

}


/// Gets an f64 from standard in.
///
/// Note, this function simply gets a string input, then parses it into an f64.
///
/// No magic here, sorry :)
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

/// Gets a list of f64's from standard in.
///
/// Note this function simply calls get_inputs() and converts them into a string.
///
/// No magic here, sorry :)
pub fn get_f64s(message: &str) -> Vec<f64> {
    let inputs_as_string:       Vec<String> = get_inputs(message);
    let mut inputs_as_f64:      Vec<f64>    = Vec::new();
    let mut temp_convert_value: f64;

    for number in inputs_as_string {
        temp_convert_value = number.parse().expect("Failed to parse");
        inputs_as_f64.push(temp_convert_value);
    }

    inputs_as_f64
}


/// Gets an operation type from standard in.
///
/// Note, this function simply gets a string niput, then parses it into an OperationType
///
/// No magic here, sorry :)
pub fn get_operation_type() -> OperationType {
    let error_message: &str =
    r#"Error, unknown operation type provided. (Perhaps you misspelled it?)
The supported operation types are as follows:
1. Arithmatic
2. Trigonometric
3. Statistical"#;

    loop {
        match get_input("Please enter the type of operation.").as_str() {
            "arithmatic"    => return OperationType::ARITHMATIC,
            "trigonometric" => return OperationType::TRIGONOMETRIC,
            "statistical"   => return OperationType::STATISTICAL,
            "q"             => exit(0),
            _ => {
                println!("{}", error_message);
                wait_for_enter();
            },
        }
    }
}


/// Gets an arithmatic operation from standard in.
///
/// Returns an ArithmaticOperation.
pub fn get_arithmatic_operation() -> ArithmaticOperation {
    let error_message: &str =
    r#"Error, unknown operation provided. (Perhaps you misspelled it?)
The supported operations are as follows:
1. Addition
2. Subtraction
3. Multiplication
4. Division
5. Exponent
6. Root

To quit enter q
To go back enter b
"#;

    loop {
        match get_input("Please enter the operation you would like to perform").as_str() {
            "addition"       => return ArithmaticOperation::ADDITION,
            "subtraction"    => return ArithmaticOperation::SUBTRACTION,
            "multiplication" => return ArithmaticOperation::MULTIPLICATION,
            "division"       => return ArithmaticOperation::DIVISION,
            "exponent"       => return ArithmaticOperation::EXPONENT,
            "root"           => return ArithmaticOperation::ROOT,
            _                => {
                println!("{}", error_message);
                wait_for_enter();
            }
        }
    }
}