use std::io;
use std::io::prelude::*;

fn react(s: String) -> String {
	let mut result = "".to_string();
	let mut p: char = ' ';
	for c in s.chars() {
		if c.is_whitespace() {
			continue;
		}
		if (c.is_lowercase() && p.is_uppercase() && c == p.to_lowercase().next().unwrap())
			|| (c.is_uppercase() && p.is_lowercase() && c == p.to_uppercase().next().unwrap()) {
			result.pop();
			p = ' ';
		} else {
			result.push(c);
			p = c;
		}
	}
	result
}

fn main() {
	let stdin = io::stdin();
	let mut buffer = "".to_string();
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		buffer.push_str(line.as_str());
	}
	let mut pl = 0;
	let mut result = buffer.clone();
	loop {
		result = react(result);
		if pl == result.len() {
			break;
		}
		pl = result.len();
	}
	println!("{}", result.len());
}
