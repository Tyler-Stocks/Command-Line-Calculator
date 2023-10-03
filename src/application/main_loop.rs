use crate::application::operate::perform_arithmatic_operation;
use crate::console_utility::clear::clear_console;
use crate::input::standard_in::{get_i128, get_operation_type};
use crate::enums::operation_types::OperationType;

use super::operate::perform_trig_operation;


pub fn main_loop() {
    let mut first_number:   i128;
    let mut second_number:  i128;
    let mut operation_type: OperationType;

    loop {
        operation_type = get_operation_type();

        match operation_type {
            OperationType::ARITHMATIC    => {
                clear_console();

                first_number = get_i128("Please enter your first number:");

                clear_console();

                second_number = get_i128("Please enter your second number:");

                clear_console();

                perform_arithmatic_operation(first_number, second_number);
            },
            OperationType::TRIGONOMETRIC => {
                clear_console();

                first_number = get_i128("Please enter your number:");

                clear_console();

                perform_trig_operation(first_number);
            }
            OperationType::STATISTICAL   => todo!(),
        }
        clear_console();
    }
}