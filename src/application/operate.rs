use std::process::exit;

use crate::{arithmatic, trig, stats};

use crate::input::{keyboard::wait_for_enter, standard_in::get_input};
use crate::enums::arithmatic_operations::ArithmaticOperation;

/// Gets the user to input an arithmatic operation.
/// Performs the operation on the two supplied numbers.
pub fn perform_arithmatic_operation(first_number: f64, second_number: f64, operation: ArithmaticOperation) {
    match operation {
        ArithmaticOperation::ADDITION       => {
            arithmatic!(first_number + second_number);
            wait_for_enter();
        },
        ArithmaticOperation::SUBTRACTION    => {
            arithmatic!(first_number - second_number);
            wait_for_enter();
        },
        ArithmaticOperation::MULTIPLICATION => {
            arithmatic!(first_number * second_number);
            wait_for_enter();
        },
        ArithmaticOperation::DIVISION       => {
            arithmatic!(first_number / second_number);
            wait_for_enter();
        },
        ArithmaticOperation::EXPONENT       => {
            arithmatic!(first_number ^ second_number);
            wait_for_enter();
        },
        ArithmaticOperation::ROOT           => {
            arithmatic!(first_number root second_number);
            wait_for_enter()
        }
    }
}


/// Gets the user to input a trigonometric operation.
/// Then performs the operation on the supplied number.
pub fn perform_trig_operation(first_number: f64) {
    loop {
        match get_input("Please enter your operation.").as_str() {
            "tan" => {
                trig!(tan first_number);
                wait_for_enter();
                break;
            },
            "sin" => {
                trig!(sin first_number);
                wait_for_enter();
                break;
            },
            "cos" => {
                trig!(cos first_number);
                wait_for_enter();
                break;
            },
            "atan" => {
                trig!(atan first_number);
                wait_for_enter();
                break;
            },
            "asin" => {
                trig!(asin first_number);
                wait_for_enter();
                break;
            },
            "acos" => {
                trig!(acos first_number);
                wait_for_enter();
                break;
            },
            "b" => exit(0),
            "q" => break,
            _ => {
                println!("Operation not supported, please input a valid operation.");
                wait_for_enter();
            },
        }
    }
}


/// Gets the user to input a statistical operation.
/// Then performs the opeation on the supplied list of numbers.
pub fn perform_statistical_operation(numbers: &Vec<f64>) {
    let error_message: &str =
    r#"The statistical operation that you provided is not currently supported.
       The following operations are currently supported:
       1. Average
       2. Min
       3. Max

       To quit please enter q the next time you are prompted to input.
       To go back please enter b the next time you are prompted to input.
    "#;

    loop {
        match get_input("Please input the operation you would like to perform").as_str() {
            "average" => {
                stats!(average numbers);
                wait_for_enter();
            },
            "min"     => {
                stats!(min numbers);
                wait_for_enter();
            },
            "max"     => {
                stats!(max numbers);
                wait_for_enter();
            },
            "b"       => break,
            "q"       => exit(0),
            _         => {
                println!("{}", error_message);
                wait_for_enter();
            },
        }
    }
}