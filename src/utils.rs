// eqr - Encode text into svg/png/jpg/terminal-format QR codes

/// Convert HEX color code to RGB values
/// This function assumes `hex` parameter being a valid HEX color code.
pub fn hex_to_rgb(hex: &str) -> [u8; 3] {
	let mut hex = hex.to_string();
	if hex.len() == 3 {
		let mut expanded = String::new();
		for c in hex.chars() {
			for _ in 0..2 {
				expanded.push(c)
			}
		}
		hex = expanded;
	}

	let mut rgb: [u8; 3] = [0, 0, 0];
	for (i, rgb_val) in rgb.iter_mut().enumerate() {
		let (f, s) = hex[2 * i..2 * (i + 1)].split_at(1);
		let f = u8::from_str_radix(f, 16).unwrap();
		let s = u8::from_str_radix(s, 16).unwrap();
		*rgb_val = f * 16 + s;
	}

	rgb
}
