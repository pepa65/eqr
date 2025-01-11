// eqr - Encode text into svg/png/jpg/terminal-format QR codes

pub use clap::{AppSettings::DeriveDisplayOrder, Parser};
use lazy_static::lazy_static;
use qrcodegen::QrCodeEcc;
use regex::Regex;
use std::path::PathBuf;

/// Parse hex code colors
pub fn parse_hex_color(hex: &str) -> Result<String, String> {
	lazy_static! {
		static ref HEX_RE: Regex = Regex::new("^([0-9A-Fa-f]{3}){1,2}$").unwrap();
	}
	match HEX_RE.is_match(hex) {
		true => Ok(hex.to_string()),
		false => Err(format!("{hex} is not a valid hex color code")),
	}
}

/// Parse QR error correction level (assumes ecl being one of ["L", "low", "M", "medium", "Q", "quartile", "H", "high"])
pub fn parse_error_correction_level(ecl: &str) -> Result<QrCodeEcc, String> {
	Ok(match ecl {
		"L" | "low" => QrCodeEcc::Low,
		"M" | "medium" => QrCodeEcc::Medium,
		"Q" | "quartile" => QrCodeEcc::Quartile,
		"H" | "high" => QrCodeEcc::High,
		_ => return Err("invalid error correction level".to_string()),
	})
}

/// Encode text into svg/png/jpg/terminal-format QR codes
#[derive(Parser, Debug)]
#[clap(version, about, global_settings=&[DeriveDisplayOrder])]
#[clap(help_template(
	"\
{before-help}{name} {version} - {about}
{usage-heading} {usage}
{all-args}{after-help}
"
))]
pub struct Args {
	/// Output file (jpg/png/svg)
	#[clap(short, long, value_parser, default_value = "qr.png")]
	pub output: PathBuf,

	/// Output to terminal
	#[clap(short, long)]
	pub terminal: bool,

	/// QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)
	#[clap(
		short = 'l',
		long = "level",
		default_value = "M",
		possible_values = ["L", "low", "M", "medium", "Q", "quartile", "H", "high"],
		value_parser = parse_error_correction_level
	)]
	pub error_correction_level: QrCodeEcc,

	/// Edge size (in unit blocks)
	#[clap(short = 'e', long = "edge", default_value_t = 2, value_parser)]
	pub border: u8,

	/// Foreground RGB color (hex code)
	#[clap(
		short,
		long,
		conflicts_with = "terminal",
		default_value = "000",
		value_parser = parse_hex_color
	)]
	pub fg: String,

	/// Background RGB color (hex code)
	#[clap(
		short,
		long,
		conflicts_with = "terminal",
		default_value = "fff",
		value_parser = parse_hex_color
	)]
	pub bg: String,

	/// Scale factor (1..255)
	#[clap(short, long, conflicts_with = "terminal", default_value_t = 16, value_parser)]
	pub scale: u8,

	/// String to encode
	#[clap(required = true, value_parser)]
	pub string: Option<String>,
}
