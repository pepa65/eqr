// eqr - Encode text into svg/png/jpg/terminal-format QR codes with optional logo

mod args;
mod error;
mod utils;

use args::{Args, Parser};
use dialoguer::{theme::ColorfulTheme, Confirm};
use error::{Error, ErrorKind};
use image::{DynamicImage, ImageBuffer, ImageReader, RgbaImage};
use qrcodegen::QrCode;
use std::{
	fmt::Write as _,
	fs,
	io::{self, Cursor, Read, Write},
	path::Path,
	process,
};
use utils::hex_to_rgba;

/// QR code:
/// - data: QrCode instance containing QR code data information
/// - border: output border size expressed in unit blocks
struct Qr {
	data: QrCode,
	border: i32,
}

impl Qr {
	fn new(data: QrCode, border: u8) -> Self {
		Self { data, border: border.into() }
	}
}

/// Trait defining methods to output QR code to various formats
trait QrOutput {
	/// Create SVG file of the QR code
	fn svg(&self, output: &Path, scale: i32, bg: &str, fg: &str) -> Result<(), ErrorKind>;

	/// Create raster image (png|jpg) file of the QR code
	fn rst(&self, output: &Path, scale: i32, bg: &str, fg: &str, logo: image::DynamicImage) -> Result<(), ErrorKind>;

	/// Output QR to terminal
	fn terminal(&self);
}

impl QrOutput for Qr {
	/// Create SVG file of the QR code
	fn svg(&self, output: &Path, scale: i32, bg: &str, fg: &str) -> Result<(), ErrorKind> {
		// Create output file
		let mut file = match fs::File::create(output) {
			Ok(file) => file,
			Err(err) => return Err(ErrorKind::Error(Error::SvgOutputErr(err.to_string()))),
		};
		// Generate a string of SVG code for an image depicting the given QR Code
		// (use Unix newlines '\n' regardless of the platform)
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
		// Write actual QR code information
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
		// Write SVG to output file
		if let Err(err) = file.write_all(svg_str.as_bytes()) {
			return Err(ErrorKind::Error(Error::SvgOutputErr(err.to_string())));
		}

		Ok(())
	}

	/// Create raster image (png|jpg) file of the QR code
	fn rst(&self, output: &Path, scale: i32, bg: &str, fg: &str, logo: image::DynamicImage) -> Result<(), ErrorKind> {
		// Convert colors to RGB values
		let fg = hex_to_rgba(fg);
		let bg = hex_to_rgba(bg);
		let scaled_border = scale * self.border;
		// Size of the image including the borders
		let img_size = self.data.size() * scale + (2 * scaled_border);
		// Create square image: image needs border on each side of the square
		let mut img: RgbaImage = ImageBuffer::new(img_size as u32, img_size as u32);
		// Write image pixels
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
		let w = img_size / 4;
		let m = img_size / 2 - w / 2;
		let logosmall = logo.resize(w as u32, w as u32, image::imageops::FilterType::Nearest).to_rgba8();
		image::imageops::overlay(&mut img, &DynamicImage::ImageRgba8(logosmall), m.into(), m.into());
		// Save image
		if let Err(err) = img.save(output) {
			return Err(ErrorKind::Error(Error::RasterOutputErr(err.to_string())));
		}

		Ok(())
	}

	/// Output QR code to terminal
	fn terminal(&self) {
		for y in -self.border..self.data.size() + self.border {
			for x in -self.border..self.data.size() + self.border {
				print!("{0}{0}", if self.data.get_module(x, y) { ' ' } else { '█' });
			}
			println!();
		}
	}
}

/// Run the program and catch errors
fn run(args: Args) -> Result<(), ErrorKind> {
	// If string to encode is not passed in as CLI argument, check stdin for piped string
	let mut string = args.string.unwrap_or("".to_string());
	if string.is_empty() {
		if atty::is(atty::Stream::Stdin) {
			return Err(ErrorKind::Error(Error::NoStringGiven()));
		};
		io::stdin().lock().read_to_string(&mut string).unwrap();
		string = string.trim_end().to_string();
		if string.is_empty() {
			return Err(ErrorKind::Error(Error::NoStringPiped()));
		};
	};

	// Generate QR code
	let qr = match QrCode::encode_text(&string, args.error_correction_level) {
		Ok(data) => Qr::new(data, args.border),
		Err(err) => return Err(ErrorKind::Error(Error::QrCodeErr(err.to_string()))),
	};

	// Prep logo
	let pix = include_bytes!("../1x1.png").to_vec();
	let logo = if args.logo_path.is_none() {
		//		std::fs::read("1x1.png").unwrap()
		pix
	} else {
		std::fs::read(args.logo_path.unwrap()).unwrap()
	};
	let logo = ImageReader::new(Cursor::new(logo))
		.with_guessed_format()
		.map_err(|_| panic!("Image should be either PNG or JPEG"))?
		.decode()
		.unwrap();

	// Write file if output-flag given or no terminal-flag given
	let mut out = match args.output {
		Some(string) => string,
		None => "".to_string(),
	};
	if !out.is_empty() || !args.terminal {
		if out.is_empty() {
			out = "qr.png".to_string()
		};
		let output = Path::new(&out);
		// Check if output file exists and if so ask for overwrite
		if output.is_file() {
			let _ = Confirm::with_theme(&ColorfulTheme::default())
				.with_prompt(format!("Overwrite {:?}?", &output))
				.interact()
				.expect("dialog interaction failed");
		};

		// Determine output file type based on file extension
		match &output.extension().map(|ext| ext.to_str().unwrap()) {
			Some("svg") => qr.svg(output, i32::from(args.scale), &args.bg, &args.fg)?,
			Some("png" | "jpg") => qr.rst(output, i32::from(args.scale), &args.bg, &args.fg, logo)?,
			_ => return Err(ErrorKind::Error(Error::InvalidOutputExt)),
		};
	};

	// Output to terminal if terminal-flag given
	if args.terminal {
		qr.terminal()
	}
	Ok(())
}

/// Main function: calls run function and prints errors
fn main() {
	if let Err(e) = run(Args::parse()) {
		e.colorize().unwrap();
		process::exit(1);
	}
}
