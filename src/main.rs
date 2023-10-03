pub mod macros;
pub mod console_utility;
pub mod input;
pub mod application;
pub mod enums;

use crate::application::main_loop::main_loop;
use crate::application::start::start;

fn main() {
   start();
   main_loop();
}
