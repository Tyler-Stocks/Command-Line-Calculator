use console::Term;

macro_rules! evalulate {
    ($first:tt + $second:tt) => {
        println!("{} + {} = {}", $first, $second, $first as f64 + $second as f64)
    };
    ($first:tt - $second:tt) => {
        println!("{} - {} = {}", $first, $second, $first as f64 - $second as f64)
    };
    ($first:tt * $second:tt) => {
        println!("{} * {} = {}", $first, $second, $first as f64 * $second as f64)
    };
    ($first:tt / $second:tt) => {
        println!("{} / {} = {}", $first, $second, $first as f64 / $second as f64)
    };
    ($base:tt ** $power:tt) => {
        println!("{} ^ {} = {}", $base, $power, i128::pow($base as i128, $power as u32))
    };
    ($nth:tt root $number:tt) => {
        println!("The {}th root of {} is {}", $nth, $number, i128::pow($nth, (1 / $number) as u32))
    };
}

fn clear_console() {
    print!("\x1B[2J\x1B[1;1H");
}


fn wait_for_character(character_to_check: char) {
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

fn wait_for_enter() {
    loop {
        let keyboard_stroke: char;

        keyboard_stroke = Term::stdout()
                            .read_char()
                            .unwrap();

        match keyboard_stroke {
            '\n' => break,
            _    => ()
        }
    }
}


fn main() {
    clear_console();
    println!("Hello, press 's' to start the calculator.");
    wait_for_character('s');
    clear_console();

    let mut first_number_str:        String = String::new();
    let mut first_number_str_result: Result<usize, std::io::Error>;
    let mut first_number:            i128;
    let mut first_number_result:     Result<i128, std::num::ParseIntError>;

    let mut second_number_str:        String = String::new();
    let mut second_number_str_result: Result<usize, std::io::Error>;
    let mut second_number:            i128;
    let mut second_number_result:     Result<i128, std::num::ParseIntError>;

    let mut operation:         String = String::new();
    let mut operation_result:  Result<usize, std::io::Error>;

    loop {
        clear_console();

        loop {
            println!("Please enter your first number.");
            first_number_str_result =
                std::io::stdin().read_line(&mut first_number_str);

            match first_number_str_result {
                Ok(_value) => {
                    first_number_result = first_number_str.trim().parse::<i128>();

                    match first_number_result {
                        Ok(value) => {
                            first_number = value;
                            clear_console();
                            break;
                        }
                        Err(_error) => {
                            println!("Failed to convert input into i128.");
                            wait_for_enter();
                            clear_console();
                        }
                    }
                }
                Err(error) => {
                    clear_console();
                    println!("Failed to read the number due to {}, please try again",error);
                    wait_for_enter();
                    clear_console()
                }
            }
        }

        clear_console();

        loop {
            println!("Please enter your second number.");
            second_number_str_result =
                std::io::stdin().read_line(&mut second_number_str);

            match second_number_str_result {
                Ok(_value) => {
                    second_number_result = second_number_str.trim().parse::<i128>();

                    match second_number_result {
                        Ok(value) => {
                            second_number = value;
                            clear_console();
                            break;
                        },
                        Err(_error) => {
                            println!("Failed to convert input into i128.");
                            wait_for_enter();
                            clear_console();
                        },
                    }
                }
                Err(error) => {
                    clear_console();
                    println!("Failed to read the number due to {}, please try again",error);
                    wait_for_enter();
                    clear_console()
                }
            }
        }

        clear_console();

        loop {
            println!("Please enter the operation your would like to perform.");
            operation_result = std::io::stdin().read_line(&mut operation);

            match operation_result {
                Ok(_value)  => (),
                Err(_error) => {
                    clear_console();
                    println!("Failed to read operation");
                    wait_for_enter();
                    clear_console();
                },
            }

            match operation.trim().to_lowercase().as_str() {
                "addition" => {
                    clear_console();
                    evalulate!(first_number + second_number);
                    wait_for_enter();
                    clear_console();
                    break;
                },
                "subtraction" => {
                    clear_console();
                    evalulate!(first_number - second_number);
                    wait_for_enter();
                    clear_console();
                    break;
                },
                "multiplication" => {
                    clear_console();
                    evalulate!(first_number * second_number);
                    wait_for_enter();
                    clear_console();
                    break;
                },
                "division" => {
                    clear_console();
                    evalulate!(first_number / second_number);
                    wait_for_enter();
                    clear_console();
                    break;
                },
                "power" => {
                    clear_console();
                    evalulate!(first_number ** second_number);
                    wait_for_enter();
                    clear_console();
                    break;
                },
                "root" => {
                    clear_console();
                    evalulate!(first_number root second_number);
                    wait_for_enter();
                    clear_console();
                    break;
                },
                _ => {
                    clear_console();
                    println!("Operation {}, not supported.", operation);
                    wait_for_enter();
                    clear_console();
                }

            }
        }
    }
}
