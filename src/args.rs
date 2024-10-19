// eqr - Encode text into svg/png/jpg/terminal-format QR codes

pub use clap::Parser;
use lazy_static::lazy_static;
use qrcodegen::QrCodeEcc;
use regex::Regex;
use std::path::PathBuf;

/// Parse hex code colors.
pub fn parse_hex_color(hex: &str) -> Result<String, String> {
    lazy_static! {
        static ref HEX_RE: Regex = Regex::new("^([0-9A-Fa-f]{3}){1,2}$").unwrap();
    }

    match HEX_RE.is_match(hex) {
        true => Ok(hex.to_string()),
        false => Err(format!("{hex} is not a valid hex color code")),
    }
}

/// Parse QR error correction level (assumes ecl being one of ["low", "medium", "quartile", "high"]).
pub fn parse_error_correction_level(ecl: &str) -> Result<QrCodeEcc, String> {
    Ok(match ecl {
        "low" => QrCodeEcc::Low,
        "medium" => QrCodeEcc::Medium,
        "quartile" => QrCodeEcc::Quartile,
        "high" => QrCodeEcc::High,
        _ => return Err("invalid error correction level".to_string()),
    })
}

/// A CLI utility to encode URLs or text into QR codes in various formats and colors.
#[derive(Parser, Debug)]
#[clap(version, about)]
#[clap(help_template("\
{before-help}{name} {version} - {about}
{usage-heading} {usage}
{all-args}{after-help}
"))]
pub struct Args {
    /// Output file (supported file extensions: jpg, png, svg); omit to print QR code to console.
    #[clap(short, long, value_parser)]
    pub output: Option<PathBuf>,

    /// Foreground RGB color (hex code).
    #[clap(
        short,
        long,
        requires = "output",
        default_value = "000",
        value_parser = parse_hex_color
    )]
    pub fg: String,

    /// Background RGB color (hex code).
    #[clap(
        short,
        long,
        requires = "output",
        default_value = "fff",
        value_parser = parse_hex_color
    )]
    pub bg: String,

    /// Border size (expressed in unit blocks).
    #[clap(short = 'B', long, default_value_t = 1, value_parser)]
    pub border: u8,

    /// QR error correction level.
    #[clap(
        short = 'L',
        long,
        default_value = "medium",
        possible_values = ["low", "medium", "quartile", "high"],
        value_parser = parse_error_correction_level
    )]
    pub error_correction_level: QrCodeEcc,

    /// Scale factor (1..255).
    #[clap(short, long, requires = "output", default_value_t = 16, value_parser)]
    pub scale: u8,

    /// String to encode.
    #[clap(value_parser)]
    pub string: Option<String>,
}
