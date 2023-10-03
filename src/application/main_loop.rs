use crate::application::operate::perform_operation;
use crate::console_utility::clear::clear_console;
use crate::input::standard_in::get_i128;


pub fn main_loop() {
    let mut first_number:     i128;
    let mut second_number:     i128;

    loop {
        clear_console();

        first_number = get_i128("Please enter your first number");

        clear_console();

        second_number = get_i128("Please enter your second number");

        clear_console();

        perform_operation(first_number, second_number);

        clear_console();
    }
}