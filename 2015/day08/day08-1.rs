use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let (mut literal_length, mut memory_length) = (0, 0);
	
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		literal_length += line.len();
		let mut parsed_line = line
			.replace("\\\\", "|")
			.replace("\\\"", "|")
			.replace("\"", "");
		loop {
			match parsed_line.clone().find("\\x") {
			Some(i) => { parsed_line.replace_range(i..i+4, "|"); }
			None => { break; }
			}
		}
		memory_length += parsed_line.len();
	}
	
	println!("{} - {} = {}", literal_length, memory_length, literal_length-memory_length);
}
