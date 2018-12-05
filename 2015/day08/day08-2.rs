use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let (mut literal_length, mut encoded_length) = (0i32, 0i32);
	
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		literal_length += line.len() as i32;
		let mut parsed_line = line
			.replace("\\", "\\\\")
			.replace("\"", "\\\"");
		encoded_length += parsed_line.len() as i32 + 2;
	}
	
	println!("{} - {} = {}", encoded_length, literal_length, encoded_length-literal_length);
}
