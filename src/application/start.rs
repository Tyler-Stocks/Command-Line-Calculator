use console::Term;

use crate::input::keyboard::wait_for_enter;
use crate::console_utility::clear::clear_console;

pub fn start() {
    // Logic performed before the main application loop.

    clear_console();

    loop {

        println!("Press 's' to start the calculator.");
        println!("Press 'h' for help.");
        println!("Press 'q' to quit.");


        match Term::stdout().read_char().unwrap() {
            's' => break,
            'h' => {
                clear_console();
                println!(
                    r#"
Hello, this is a calculator that supports the following operations:
    1. Addition
    2. Subtraction
    3. Multiplication
    4. Divsion
    5. Exponent
    6. Root
To continue after an operation or an error, press enter.
Press Enter to continue."#
                );
                wait_for_enter();
            }
            'q' => std::process::exit(1),
            _ => (),
        }
    }
}
