// eqr - Encode text into svg/png/jpg/terminal-format QR codes

mod args;
mod error;
mod utils;

use args::{Args, Parser};
use dialoguer::{theme::ColorfulTheme, Confirm};
use error::{Error, ErrorKind};
use image::{ImageBuffer, RgbImage};
use qrcodegen::QrCode;
use std::{
	fmt::Write as _,
	fs,
	io::{self, Read, Write},
	path::Path,
	process,
};
use utils::hex_to_rgb;

/// QR code.
/// - data: `QrCode` instance containing QR code data information
/// - border: output border size expressed in unit blocks
struct Qr {
	data: QrCode,
	border: i32,
}

impl Qr {
	// Construct `Qr` instance.
	fn new(data: QrCode, border: u8) -> Self {
		Self { data, border: border.into() }
	}
}

/// Trait defining methods to output QR code to various formats.
trait QrOutput {
	/// Create SVG file containing the QR code.
	fn svg(&self, output: &Path, scale: i32, bg: &str, fg: &str) -> Result<(), ErrorKind>;

	/// Create raster image (png|jpg) file containing the QR code.
	fn rst(&self, output: &Path, scale: i32, bg: &str, fg: &str) -> Result<(), ErrorKind>;

	/// Print QR code to the console.
	fn console(&self);
}

impl QrOutput for Qr {
	/// Create SVG file containing the QR code.
	fn svg(&self, output: &Path, scale: i32, bg: &str, fg: &str) -> Result<(), ErrorKind> {
		// Create output file.
		let mut file = match fs::File::create(output) {
			Ok(file) => file,
			Err(err) => return Err(ErrorKind::Error(Error::SvgOutputErr(err.to_string()))),
		};

		// Generate a string of SVG code for an image depicting the given QR Code.
		// The string always uses Unix newlines (\n), regardless of the platform.
		let mut svg_str = String::new();

		let size = self.data.size();
		let dimension = size.checked_add(self.border * 2).unwrap();
		let pix = scale * dimension;

		write!(
			svg_str,
			"<svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" \
                  viewBox=\"0 0 {dimension} {dimension}\" width=\"{pix}\" height=\"{pix}\">\
                  <rect width=\"100%\" height=\"100%\" fill=\"#{bg}\"/><path d=\""
		)
		.unwrap();

		// Write actual QR code information.
		for y in 0..size {
			for x in 0..size {
				if self.data.get_module(x, y) {
					if x != 0 || y != 0 {
						write!(svg_str, " ").unwrap();
					}
					write!(svg_str, "M{},{}h1v1h-1z", x + self.border, y + self.border).unwrap();
				}
			}
		}
		writeln!(svg_str, "\" fill=\"#{fg}\"/></svg>").unwrap();

		// Write SVG to output file.
		if let Err(err) = file.write_all(svg_str.as_bytes()) {
			return Err(ErrorKind::Error(Error::SvgOutputErr(err.to_string())));
		}

		Ok(())
	}

	/// Create raster image (png|jpg) file containing the QR code.
	fn rst(&self, output: &Path, scale: i32, bg: &str, fg: &str) -> Result<(), ErrorKind> {
		// Convert colors to RGB values.
		let fg = hex_to_rgb(fg);
		let bg = hex_to_rgb(bg);

		let scaled_border = scale * self.border;
		// Size of the image including the borders.
		let img_size = self.data.size() * scale + (2 * scaled_border);

		// Create square image: image needs border on each side of the square.
		let mut img: RgbImage = ImageBuffer::new(img_size as u32, img_size as u32);

		// Write image pixels.
		for y in 0..img_size {
			for x in 0..img_size {
				let pixel = img.get_pixel_mut(x as u32, y as u32);

				if x < scaled_border || y < scaled_border {
					pixel.0 = bg;
					continue;
				}

				let x = (x - scaled_border) / scale;
				let y = (y - scaled_border) / scale;
				pixel.0 = if self.data.get_module(x, y) { fg } else { bg };
			}
		}

		// Save image.
		if let Err(err) = img.save(output) {
			return Err(ErrorKind::Error(Error::RasterOutputErr(err.to_string())));
		}

		Ok(())
	}

	/// Print QR code to standard output.
	fn console(&self) {
		for y in -self.border..self.data.size() + self.border {
			for x in -self.border..self.data.size() + self.border {
				print!("{0}{0}", if self.data.get_module(x, y) { ' ' } else { '█' });
			}
			println!();
		}
	}
}

/// Runs the program & catches errors.
fn run(args: Args) -> Result<(), ErrorKind> {
	// If string to encode is not passed in as CLI argument, check stdin for piped string.
	let string = args.string.unwrap_or_else(|| {
		if atty::is(atty::Stream::Stdin) {
			clap::Command::new("qr [OPTIONS] [STRING]")
				.error(
					clap::ErrorKind::MissingRequiredArgument,
					"Missing input string.\n\n\
                        \tEither provide it as a CLI argument or pipe it in from standard input.",
				)
				.exit();
		} else {
			let mut string = String::new();
			io::stdin().lock().read_to_string(&mut string).unwrap();
			string.trim_end().to_string()
		}
	});

	// Generate QR code.
	let qr = match QrCode::encode_text(&string, args.error_correction_level) {
		Ok(data) => Qr::new(data, args.border),
		Err(err) => return Err(ErrorKind::Error(Error::QrCodeErr(err.to_string()))),
	};

	// Print to stdout if args.output is None, otherwhise print to file.
	match &args.output {
		Some(output) => {
			// Check if output file exists and if so ask for overwrite.
			if output.is_file()
				&& !Confirm::with_theme(&ColorfulTheme::default())
					.with_prompt(format!("Overwrite {:?}?", output))
					.interact()
					.expect("dialog interaction failed")
			{
				return Ok(());
			}

			// Determine output file type based on file extension.
			match output.extension().map(|ext| ext.to_str().unwrap()) {
				Some("svg") => qr.svg(output, i32::from(args.scale), &args.bg, &args.fg)?,
				Some("png" | "jpg") => qr.rst(output, i32::from(args.scale), &args.bg, &args.fg)?,
				_ => return Err(ErrorKind::Error(Error::InvalidOutputExt)),
			}
		}
		// When no output file is specified, print QR code to stdout.
		None => qr.console(),
	};

	Ok(())
}

/// Main function: calls run function and prints errors.
fn main() {
	if let Err(e) = run(Args::parse()) {
		e.colorize().unwrap();
		process::exit(1);
	}
}
