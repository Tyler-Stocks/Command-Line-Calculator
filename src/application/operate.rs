use crate::{arithmatic, trig};

use crate::input::{keyboard::wait_for_enter, standard_in::get_input};


/// Gets the user to input an arithmatic operation.
/// Performs the operation on the two supplied numbers.
pub fn perform_arithmatic_operation(first_number: f64, second_number: f64) -> () {
    loop {
        match get_input("Please enter your operation").as_str() {
            "addition" => {
                arithmatic!(first_number + second_number);
                wait_for_enter();
                break;
            },
            "subtraction" => {
                arithmatic!(first_number - second_number);
                wait_for_enter();
                break;
            },
            "multiplication" => {
                arithmatic!(first_number * second_number);
                break;
            },
            "division" => {
                arithmatic!(first_number / second_number);
                wait_for_enter();
                break;
            },
            "power" => {
                arithmatic!(first_number ^ second_number);
                wait_for_enter();
                break;
            },
            "root" => {
                arithmatic!(first_number root second_number);
                wait_for_enter();
                break;
            },
            _ => {
                println!("The operation you entered is not supported.");
                wait_for_enter();
            }
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
            _ => {
                println!("Operation not supported, please input a valid operation.");
                wait_for_enter();
            }
        }
    }
}
