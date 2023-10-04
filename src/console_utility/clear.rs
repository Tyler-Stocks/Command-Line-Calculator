use clearscreen::{ClearScreen, Error};

use crate::input::keyboard::wait_for_enter;

/// Wrapper around
///
/// clearscreen::ClearScreen::default.clear().expect("Failed to clear the terminal")
pub fn clear_console() -> () {
    let mut clear_screen_result: Result<(), Error>;
    let mut count: u8 = 0;

    loop {
        if count == 3 {break;}
        clear_screen_result = ClearScreen::default().clear();

        match clear_screen_result {
            Ok(_value)    => return,
            Err(error) => {
                println!("Failed to clear the terminal, {}", error);
                wait_for_enter()
            }
        }

        count += 1;
    }
}
