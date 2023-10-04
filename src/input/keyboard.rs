use console::Term;

use crate::console_utility::clear::clear_console;

/// Waits for a specified character to be inputed, then returns.
///
/// Note that special characters cannot be detected.
///
/// For example, 'ctrl' could not be detected, but 'a' could be.
pub fn wait_for_character(character_to_check: char) {
  loop {
    if Term::stdout().read_char().unwrap() == character_to_check {
      clear_console();
      return;
    }
  }
}

/// Waits for enter key to be pressed, then continues.
pub fn wait_for_enter() {
  loop {
    if Term::stdout().read_char().unwrap() == '\n' {
      clear_console();
      return;
    }
  }
}

/// Returns a bool as to whether or not a character was pressed.
pub fn check_for_character(char_to_check: char) -> bool {
  loop {
    if Term::stdout().read_char().unwrap() == char_to_check {
      return true;
    }
  }
}
