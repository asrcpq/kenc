use std::io::Read;
use std::sync::mpsc::{channel, Sender};

use mykeccak::{Hasher, Sha3};

enum Msg {
	Found(u32, [u8; 32]),
	End(u32),
}

const NP: u32 = 12;

fn worker(tx: Sender<Msg>, sha: Sha3, offset: u32) {
	let mut minx;
	let mut min = [u8::MAX; 32];
	for x in (offset..=u32::MAX).step_by(NP as usize) {
		let mut out = [u8::MAX; 32];
		if x % (1 << 26) == 0 { eprintln!("{}", x); }
		let mut sha = sha.clone();
		sha.update(&unsafe {
			std::mem::transmute::<u32, [u8; 4]>(x)
		});
		sha.finalize(&mut out);
		if out < min {
			min = out;
			minx = x;
			tx.send(Msg::Found(minx, min)).unwrap();
		}
	}
	tx.send(Msg::End(offset)).unwrap();
}

fn main() {
	let mut x = 0;
	let mut min = [u8::MAX; 32];
	let mut sha = Sha3::v256();
	let mut inbuf = [0u8; 1024];
	let mut stdin = std::io::stdin().lock();
	loop {
		let len = stdin.read(&mut inbuf).unwrap();
		if len == 0 { break }
		sha.update(&inbuf[..len]);
	}
	let (tx, rx) = channel();
	for i in 0..NP {
		let tx = tx.clone();
		let sha = sha.clone();
		std::thread::spawn(move || worker(tx, sha, i));
	}
	let mut end_count = 0;
	loop {
		match rx.recv().unwrap() {
			Msg::End(_) => {
				end_count += 1;
				if end_count == NP { break }
			}
			Msg::Found(x2, min2) => {
				if min2 < min {
					eprintln!("{:02X?}", min2);
					min = min2;
					x = x2;
				}
			}
		}
	}
	eprintln!("{}: {:02X?}", x, min);
}
