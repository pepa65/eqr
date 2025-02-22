// promptpay - Make Thai PromptPay QR code

mod args;
mod error;
mod run;
mod utils;

//use clap::Parser;
use crate::args::{Args, Parser};
use crate::run::run;
use lazy_static::lazy_static;
use regex::Regex;
use std::io::Write;
use tempfile::NamedTempFile;

/// Parse amount
pub fn parse_amount(amountstr: &str) -> Result<String, String> {
	let mut amount = amountstr.to_string();
	amount.retain(|c| c != ',' && c != '_');
	lazy_static! {
		static ref AMOUNT: Regex = Regex::new(r"^[0-9]{1,10}(\.[0-9]{2})?$").unwrap();
	}
	match AMOUNT.is_match(&amount) {
		true => Ok(amount),
		false => Err(format!("{amount} must be numeric, with 2 decimals or no point (underscores and commas are stripped)")),
	}
}

/// Parse Thai phone number
pub fn parse_phone(phonenum: &str) -> Result<String, String> {
	let mut phone = phonenum.to_string();
	phone.retain(|c| c != '-' && c != '.');
	lazy_static! {
		static ref PHONE: Regex = Regex::new("^0[0-9]{9}$").unwrap();
	}
	match PHONE.is_match(&phone) {
		true => Ok(phone[1..].to_string()),
		false => Err(format!("{phone} must be numeric, 10 long and start with 0 (dashes and dots are stripped)")),
	}
}

/// Make Thai PromptPay QR code with optional amount
#[derive(Parser, Debug)]
#[clap(version, about)]
#[clap(help_template(
	"\
{before-help}promptpay {version} - Make Thai PromptPay QR code
{usage-heading} {usage}
{all-args}{after-help}
"
))]
pub struct Opts {
	/// Output file (png) [default: PrompPayPHONE[_AMOUNT].png]
	#[clap(short = 'o', long = "output", value_parser)]
	pub qr_path: Option<String>,

	/// Path to logo (png/jpg) [default: PromptPay logo]
	#[clap(short = 'p', long = "path", required = false)]
	pub logo_path: Option<String>,

	/// Amount in Thai baht (no decimals/point or 2 decimals)
	#[clap(short = 'a', long = "amount", required = false, value_parser = parse_amount)]
	pub amount: Option<String>,

	/// Thai phone number (10 digits starting with 0)
	#[clap(value_parser = parse_phone)]
	pub phone: String,
}

fn crc16(data: Vec<u8>) -> String {
	let mut crc = 0xffff;
	let mut len = data.len();
	let mut i = 0;
	while len > 0 {
		crc ^= (data[i] as u16) << 8;
		i += 1;
		for _ in 0..8 {
			if crc & 0x8000 != 0 {
				crc = (crc << 1) ^ 0x1021;
			} else {
				crc <<= 1;
			}
		}
		len -= 1;
	}
	format!("{:04X}", crc)
}

fn main() {
	let args = Opts::parse();

	// If phonenumber is not passed in as CLI argument, check stdin for piped string
	let phone = args.phone;
	if phone.is_empty() {
		eprintln!("ERROR: PHONE argument required");
		std::process::exit(1);
	};

	// Check output file
	let out = match args.qr_path {
		Some(s) => s,
		None => {
			if args.amount.is_some() {
				format!("PromptPay0{phone}_{}.png", args.amount.clone().unwrap())
			} else {
				format!("PromptPay0{phone}.png")
			}
		}
	};

	// Handle logo
	let mut tmp = NamedTempFile::with_suffix(".png").unwrap();
	let logoflag = if args.logo_path.is_none() {
		// Include at compile-time
		let logo = include_bytes!("../promptpay.png").to_vec();
		match tmp.write_all(&logo) {
			Ok(_) => {}
			Err(e) => {
				eprintln!("ERROR: could not write PP logo to tempfile: {e}");
				std::process::exit(2);
			}
		};
		let path = tmp.path();
		format!("-p {} ", path.display())
	} else {
		let logo_path = args.logo_path.unwrap();
		if logo_path.is_empty() {
			"".to_string()
		} else {
			format!("-p {} ", logo_path)
		}
	};

	let mut str = format!("00020101021129370016A00000067701011101130066{phone}53037645802TH6304");
	if args.amount.is_some() {
		let a = args.amount.unwrap();
		let l = format!("{:02}", a.len());
		str = format!("00020101021229370016A00000067701011101130066{phone}530376454{l}{a}5802TH6304");
	};
	let checksum = crc16(str.clone().into());
	let line = format!("QR -o {out} -l Q {logoflag}{str}{checksum}");
	if let Err(e) = run(Args::try_parse_from(line.split(' ')).unwrap()) {
		e.colorize().unwrap();
		std::process::exit(3);
	}
	let _ = tmp.close();
}
