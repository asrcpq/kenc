use std::io::Read;

use mykeccak::{Hasher, Sha3};

fn main() -> std::io::Result<()> {
	let mut result = [0u8; 32];
	let mut sha = Sha3::v256();
	let mut stdin = std::io::stdin().lock();
	let mut buf = [0u8; 1024];
	loop {
		let len = stdin.read(&mut buf)?;
		if len == 0 { break }
		sha.update(&buf[..len]);
	}
	sha.finalize(&mut result);
	kenc::hexp(&result);
	Ok(())
}
