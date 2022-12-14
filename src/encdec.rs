use std::io::{Read, Write};

use mykeccak::{Hasher, Xof};

pub fn encdec(enc: bool) -> std::io::Result<()> {
	// a dead simple encryptor, serialized only, and slow
	// TODO: not string, but bytes, use os args
	// TODO: padding for length attack
	let args = std::env::args().collect::<Vec<_>>();
	let mut shake = mykeccak::Shake::v256();
	shake.update(args[1].as_bytes());
	let mut stdin = std::io::stdin().lock();
	let mut stdout = std::io::stdout().lock();
	let mut buf = [0u8; 1024];
	let mut dbuf = [0u8; 1024];
	loop {
		let len = stdin.read(&mut buf)?;
		if len == 0 { break Ok(()) }
		shake.squeeze(&mut dbuf);
		for i in 0..len {
			if enc {
				buf[i] = buf[i].wrapping_add(dbuf[i]);
			} else {
				buf[i] = buf[i].wrapping_sub(dbuf[i]);
			}
		}
		stdout.write(&buf[..len])?;
	}
}
