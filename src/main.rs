pub mod macros;
pub mod input;
pub mod application;
pub mod enums;
pub mod console_utility;

use crate::application::{
   start::start,
   main_loop::main_loop
};


/// Ex-Girlfriend keeps calling my phone, but the Bitch can't hurt me so I'm not worried - Juice Wrld
fn main() {
   start();
   main_loop();
}
