use console::Term;

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

    if keyboard_stroke == '\n' { break }
  }
}