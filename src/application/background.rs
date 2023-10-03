use std::{thread, process};

use crate::input::keyboard::check_for_character;

pub fn spawn_termination_thread() {
  thread::spawn(||{
    match check_for_character('q') {
        true  => process::exit(0),
        false => ()
    }
  });
}