// eqr - Encode text into svg/png/jpg/terminal-format QR codes with optional logo

mod args;
mod error;
mod run;
mod utils;

use args::{Args, Parser};

use crate::run::run;

/// Main function: calls run function and prints errors
fn main() {
	if let Err(e) = run(Args::parse()) {
		e.colorize().unwrap();
		std::process::exit(1);
	}
}
