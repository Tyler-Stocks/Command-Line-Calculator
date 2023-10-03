use clearscreen::ClearScreen;

use crate::application::operate::{perform_arithmatic_operation, perform_trig_operation};
use crate::enums::operation_types::OperationType;
use crate::input::standard_in::{get_f64, get_operation_type};

pub fn main_loop() {
    let mut first_number: f64;
    let mut second_number: f64;
    let mut operation_type: OperationType;

    loop {
        operation_type = get_operation_type();

        match operation_type {
            OperationType::ARITHMATIC => {
                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");

                first_number = get_f64("Please enter your first number:");

                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");

                second_number = get_f64("Please enter your second number:");

                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");

                perform_arithmatic_operation(first_number, second_number);
            }
            OperationType::TRIGONOMETRIC => {
                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");

                first_number = get_f64("Please enter your number:");

                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");

                perform_trig_operation(first_number);
            }
            OperationType::STATISTICAL => todo!(),
        }
        ClearScreen::default()
            .clear()
            .expect("Failed to clear screen");
    }
}
