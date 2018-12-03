use std::io;
use std::io::prelude::*;

extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

// Based on https://gist.github.com/gkbrk/2e4835e3a17b3fb6e1e7
// Which ironically also solves this very puzzle.  But it's the best MD5 example
// for Rust there is. :/

fn main() {
	let mut buffer = String::new();
	let stdin = io::stdin();
	let mut handle = stdin.lock();
	
	handle.read_to_string(&mut buffer).unwrap();
	
	let mut hasher = Md5::new();
	let input = buffer.trim().as_bytes();
	for k in 0..std::u64::MAX {
		hasher.input(input);
		hasher.input(k.to_string().as_bytes());
		
		let output = hasher.result_str();
		if output[0..5].to_string() == "00000".to_string() {
			println!("{}", k);
			break;
		}
		hasher.reset();
	}
}
