// eqr - Encode text into svg/png/jpg/terminal-format QR codes with optional logo

pub use clap::Parser;
use lazy_static::lazy_static;
use qrcodegen::QrCodeEcc;
use regex::Regex;

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

/// Parse QR error correction level (ecl: L/low/M/medium/Q/quartile/H/high)
pub fn parse_level(ecl: &str) -> Result<QrCodeEcc, String> {
	Ok(match ecl {
		"L" | "low" => QrCodeEcc::Low,
		"M" | "medium" => QrCodeEcc::Medium,
		"Q" | "quartile" => QrCodeEcc::Quartile,
		"H" | "high" => QrCodeEcc::High,
		_ => return Err("invalid error correction level".to_string()),
	})
}

const DEF_EDGE: u8 = 4;

/// Encode text into svg/png/jpg/terminal-format QR codes
#[derive(Parser, Debug)]
#[clap(version, about)]
#[clap(help_template(
	"\
{name} {version} - {about}
{usage-heading} {usage}
{all-args}
"
))]
pub struct Args {
	/// Output file (jpg/png/svg) [default: qr.png]
	#[clap(short = 'o', long = "output", default_missing_value = "qr.png", value_parser)]
	pub qr_path: Option<String>,

	/// Output to terminal (never the logo)
	#[clap(short, long)]
	pub terminal: bool,

	/// QR error correction level (L: 7%, M: 15%, Q: 25%, H: 30%)
	#[clap(
		short = 'l',
		long = "level",
		default_value = "M",
		value_parser = parse_level
	)]
	pub level: QrCodeEcc,

	/// Path to logo (png/jpg)
	#[clap(short = 'p', long = "path", required = false)]
	pub logo_path: Option<std::path::PathBuf>,

	/// Logo proportion to the whole image (0..1)
	#[clap(short = 'P', long = "proportion", default_value_t = 0.25, value_parser)]
	pub proportion: f64,

	/// Edge size (in unit blocks)
	#[clap(short = 'e', long = "edge", default_value_t = DEF_EDGE, value_parser)]
	pub edge: u8,

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

	/// Size of unit block in pixels (1..255)
	#[clap(short, long, conflicts_with = "terminal", default_value_t = 8, value_parser)]
	pub scale: u8,

	/// String to encode (can also be piped in)
	#[clap(value_parser)]
	pub string: Option<String>,
}
