pub mod macros;
pub mod input;
pub mod application;
pub mod enums;
pub mod console_utility;

use crate::application::main_loop::main_loop;
use crate::application::start::start;

fn main() {
   start();
   main_loop();
}
