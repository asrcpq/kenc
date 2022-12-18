pub mod encdec;

pub fn hexp(h: &[u8]) {
	for ch in h.iter() {
		print!("{:02X}", ch);
	}
	println!();
}
