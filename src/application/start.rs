use console::Term;

use crate::input::keyboard::wait_for_enter;
use crate::console_utility::clear::clear_console;


/// Prints the start message to standard out.
pub fn display_start_message() -> () {
    clear_console();
    println!("Press s to start the calculator.");
    println!("Press h for help.");
    println!("Press q to quit");
}


/// Prints the help message to standard out.
pub fn display_help_message() -> () {
    clear_console();
    println!(r#"Hello, this is a calculator that supports the following operations:
1. Addition        7.  Sin
2. Subtraction     8.  Cos
3. Multiplication  9.  Tan
4. Divsion         10. Asin
5. Exponent        11. Acos
6. Root            12. Atan
To continue after an operation or an error, press enter."#);
}


/// Starting logic, before the application loop.
pub fn start() {
    loop {
        display_start_message();

        match Term::stdout().read_char().unwrap() {
            's' => break,
            'h' => {
                display_help_message();
                wait_for_enter();
            },
            _ => (),
        }
    }
}
