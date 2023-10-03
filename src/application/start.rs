use console::Term;
use clearscreen::ClearScreen;

use crate::input::keyboard::wait_for_enter;

pub fn start() {
    // Starting logic for the app.

    let mut keyboard_stroke: char;

    loop {
        ClearScreen::default()
            .clear()
            .expect("Failed to clear screen");
        println!("Press 's' to start the calculator.");
        println!("Press 'h' for help.");
        println!("Press 'q' to quit.");

        keyboard_stroke = Term::stdout().read_char().unwrap();

        match keyboard_stroke {
            's' => break,
            'h' => {
                ClearScreen::default()
                    .clear()
                    .expect("Failed to clear screen");
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
        ClearScreen::default()
            .clear()
            .expect("Failed to clear screen");
    }
}
