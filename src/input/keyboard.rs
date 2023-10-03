use console::Term;

use crate::console_utility::clear::clear_console;

pub fn wait_for_character(character_to_check: char) {
  loop {
    let keyboard_stroke = Term::stdout()
                                  .read_char()
                                  .unwrap();

    match keyboard_stroke {
      x if x == character_to_check => break,
      _                                  => (),
    }
  }
}


pub fn wait_for_enter() {
  loop {

    let keyboard_stroke: char = Term::stdout()
                                  .read_char()
                                  .unwrap();

    if keyboard_stroke == '\n' {
      clear_console();
      break;
    }
  }
}

pub fn check_for_character(char_to_check: char) -> bool {
  let mut keyboard_input_character: char;
  loop {
    keyboard_input_character = Term::stdout()
                                 .read_char()
                                 .unwrap();

    if keyboard_input_character == char_to_check {
      return true;
    }
  }
}