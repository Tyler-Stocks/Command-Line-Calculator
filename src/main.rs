pub mod macros;
pub mod input;
pub mod application;
pub mod enums;
pub mod console_utility;

use crate::application::{background::*, start::*, main_loop::*};


fn main() {
   spawn_termination_thread();
   start();
   main_loop();
}
