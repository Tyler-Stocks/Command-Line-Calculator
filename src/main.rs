pub mod macros;
pub mod input;
pub mod application;
pub mod enums;
pub mod console_utility;

use crate::application::{
   start::start,
   main_loop::main_loop
};


fn main() {
   start();
   main_loop();
}
