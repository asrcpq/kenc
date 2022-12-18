// 512 bit(=128 hex) based detkey builder

use std::io::Read;

use mykeccak::{Hasher, Sha3};

fn main() {
	let mut buf = [0u8; 130];
	let len = std::io::stdin().lock().read(&mut buf).unwrap();
	if len == 0 {
		eprintln!("root key");
	} else {
		if len == 129 {
			if buf[128] != b'\n' {
				panic!("invalid input");
			}
		} else if len != 128 {
			panic!("invalid input");
		}
		eprintln!("sub key");
		for i in 0..128 {
			if (b'A'..=b'Z').contains(&buf[i]) {
				buf[i] = buf[i] - b'A' + b'a';
			} else if !(b'0'..=b'9').contains(&buf[i]) &&
				!(b'a'..=b'z').contains(&buf[i])
			{
				panic!("invalid input");
			}
		}
	}
	let args: Vec<String> = std::env::args().collect();
	let key: Vec<u8> = args[1].as_bytes().to_vec();
	let mut sha = Sha3::v512();
	sha.update(&buf[..128]);
	sha.update(&key);
	let mut result = [0u8; 64];
	sha.finalize(&mut result);
	kenc::hexp(&result);
}
