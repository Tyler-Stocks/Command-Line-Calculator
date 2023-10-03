use crate::evalulate;

use crate::input::standard_in::get_input;
use crate::input::keyboard::wait_for_enter;
use crate::console_utility::clear::clear_console;

pub fn perform_operation(first_number: i128, second_number: i128) {
    let mut operation: String;
    loop {
        operation = get_input("Please enter your operation");
        clear_console();

        match operation.as_str() {
            "addition" => {
                evalulate!(first_number + second_number);
                wait_for_enter();
                break;
            },
            "subtraction" => {
                evalulate!(first_number - second_number);
                wait_for_enter();
                break;
            },
            "multiplication" => {
                evalulate!(first_number * second_number);
                break;
            },
            "division" => {
                evalulate!(first_number / second_number);
                wait_for_enter();
                break;
            },
            "power" => {
                evalulate!(first_number ** second_number);
                wait_for_enter();
                break;
            },
            "root" => {
                evalulate!(first_number root second_number);
                wait_for_enter();
                break;
            },
            _ => {
                println!("Operation {}, not supported.", operation);
            }
        }
        clear_console();
    }
}