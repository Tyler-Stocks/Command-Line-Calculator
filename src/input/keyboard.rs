use std::io::Error;

use console::Term;

use crate::console_utility::clear::clear_console;

/// Waits for a specified character to be inputed, then returns.
///
/// Note that special characters cannot be detected.
///
/// For example, 'ctrl' could not be detected, but 'a' could be.
pub fn wait_for_character(character_to_check: char) {
  let mut character_read_result: Result<char, Error>;
  
  loop {
    character_read_result = Term::stdout().read_char();

    match character_read_result {
      Ok(value) => {
        if character_to_check == value {
          clear_console();
          return;
        }
      },
      Err(error) => {
        println!("Failed to read character from standard out due to {}.", error);
      }
    }
  }
}

/// Waits for enter key to be pressed, then continues.
pub fn wait_for_enter() {
  let mut character_read_result: Result<char, Error>;

  loop {
    character_read_result = Term::stdout().read_char();

    match character_read_result {
      Ok(value) => {
        if value == '\n' {
          clear_console();
          return;
        }
      },
      Err(_error) => ()
    }
  }
}

/// Returns a bool as to whether or not a character was pressed.
pub fn check_for_character(char_to_check: char) -> bool {
  let mut character_read_result: Result<char, Error>;

  loop {
    character_read_result = Term::stdout().read_char();

    match character_read_result {
      Ok(value) => {
        if value == char_to_check {
          clear_console();
          return true;
        }
      },
      Err(error) => {
        println!("Failed to read character from standard in due to {}.", error)
      }
    }
  }
}
