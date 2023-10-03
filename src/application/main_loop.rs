use crate::application::operate::{perform_arithmatic_operation, perform_trig_operation};
use crate::enums::operation_types::OperationType;
use crate::input::standard_in::{get_f64, get_operation_type};

pub fn main_loop() {
    let mut first_number:  f64;
    let mut second_number: f64;

    loop {
        match get_operation_type() {
            OperationType::ARITHMATIC => {
                first_number  = get_f64("Please enter your first number:");
                second_number = get_f64("Please enter your second number:");
                perform_arithmatic_operation(first_number, second_number);
            }
            OperationType::TRIGONOMETRIC => {
                first_number = get_f64("Please enter your number:");
                perform_trig_operation(first_number);
            }
            OperationType::STATISTICAL => todo!(),
        }
    }
}
