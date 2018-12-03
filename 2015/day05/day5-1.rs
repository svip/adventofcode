use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let mut niceness = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		if line.chars().filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u').count() >= 3 {
			let mut nice = false;
			let mut p: char = ' ';
			for c in line.chars() {
				if p != ' ' {
					if c == p {
						nice = true;
					}
					let m = format!("{}{}", p, c);
					if ["ab","cd","pq","xy"].iter().any(|s| *s == m.as_str()) {
						nice = false;
						break;
					}
				}
				p = c;
			}
			if nice {
				niceness += 1;
			}
		}
	}
	println!("{}", niceness);
}
