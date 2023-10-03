use crate::{arithmatic, trig};

use crate::input::standard_in::get_input;
use crate::input::keyboard::wait_for_enter;
use crate::console_utility::clear::clear_console;

pub fn perform_arithmatic_operation(first_number: f64, second_number: f64) {
    let mut operation: String;
    loop {
        operation = get_input("Please enter your operation");
        clear_console();

        match operation.as_str() {
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
                arithmatic!(first_number ** second_number);
                wait_for_enter();
                break;
            },
            "root" => {
                arithmatic!(first_number root second_number);
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

pub fn perform_trig_operation(first_number: f64) {
    let mut operation: String;

    loop {
        operation = get_input("Please enter your operation.");

        clear_console();
        match operation.as_str() {
            "tan"  => {
                trig!(tan first_number);
                wait_for_enter();
                break;
            },
            "sin"  => {
                trig!(sin first_number);
                wait_for_enter();
                break;
            },
            "cos"  => {
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
            _      => {

                println!("Operation not supported, please input a valid operation.");
                wait_for_enter();
            }
        }
        clear_console();
    }
}