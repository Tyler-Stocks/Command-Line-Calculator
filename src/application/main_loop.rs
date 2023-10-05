use crate::application::operate::{perform_arithmatic_operation, perform_trig_operation};
use crate::enums::arithmatic_operations::ArithmaticOperation;
use crate::enums::operation_types::OperationType;
use crate::input::standard_in::{get_f64, get_f64s, get_operation_type, get_arithmatic_operation};

use super::operate::perform_statistical_operation;


/// Main application loop, does not return, may panic.
pub fn main_loop() -> ! {
    let mut operation:     ArithmaticOperation;
    let mut first_number:  f64;
    let mut second_number: f64;
    let mut numbers:       Vec<f64>;

    loop {
        match get_operation_type() {
            OperationType::ARITHMATIC => {
                operation     = get_arithmatic_operation();
                first_number  = get_f64("Please enter your first number:");
                second_number = get_f64("Please enter your second number:");
                perform_arithmatic_operation(first_number, second_number, operation);
            }
            OperationType::TRIGONOMETRIC => {
                first_number = get_f64("Please enter your number:");
                perform_trig_operation(first_number);
            }
            OperationType::STATISTICAL => {
                numbers = get_f64s("Please enter your numbers, press f to finish.");
                perform_statistical_operation(&numbers);
            },
        }
    }
}
