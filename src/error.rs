// eqr - Encode text into svg/png/jpg/terminal-format QR codes with optional logo

use std::fmt::{Display, Formatter};
use std::io::{IsTerminal, Write};
use std::{fmt, io};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

pub enum Error {
	/// Occurs when unable to generate QR code.
	QrCodeErr(String),
	/// Occurs when no file is found at the path for the logo.
	BadPath(),
	/// Occurs when user choses unsupported output file extension.
	InvalidOutputExt,
	/// Occurs when unable to generate SVG output file.
	SvgOutputErr(String),
	/// Occurs when unable to generate raster image output file.
	RasterOutputErr(String),
	/// Occurs when no string is piped in and no string given
	NoStringPiped(),
	/// Occurs when no string is given nor piped in
	NoStringGiven(),
}

pub enum ErrorKind {
	Error(Error),
}

impl ErrorKind {
	/// Colorize warning|error output.
	pub fn colorize(&self) -> io::Result<()> {
		//let color_choice = match atty::is(atty::Stream::Stderr) {
		let color_choice = match io::stderr().is_terminal() {
			true => ColorChoice::Auto,
			false => ColorChoice::Never,
		};
		// Color based on ErrorKind variant:
		//  * Warning -> ("warning:", Yellow)
		//  * Error -> ("error:", Red)
		let (prefix, color) = match self {
			Self::Error(_) => ("error", Some(Color::Red)),
		};
		let writer = BufferWriter::stderr(color_choice);
		let mut buffer = writer.buffer();
		buffer.set_color(ColorSpec::new().set_fg(color).set_bold(true))?;
		write!(&mut buffer, "{}: ", prefix)?;
		buffer.reset()?;
		writeln!(&mut buffer, "{}", self)?;
		writer.print(&buffer)
	}
}

impl Display for ErrorKind {
	/// Print colored error message, but ONLY on Stderr stream.
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			Self::Error(error) => match error {
				Error::QrCodeErr(msg) => write!(f, "unable to generate QR code: {msg}"),
				Error::BadPath() => write!(f, "no logo found at given path"),
				Error::InvalidOutputExt => write!(f, "invalid output file extension"),
				Error::SvgOutputErr(msg) => write!(f, "unable to write SVG output file: {msg}"),
				Error::RasterOutputErr(msg) => write!(f, "unable to write raster image file: {msg}"),
				Error::NoStringPiped() => write!(f, "no string piped into command"),
				Error::NoStringGiven() => write!(f, "no string given on commandline"),
			},
		}
	}
}
