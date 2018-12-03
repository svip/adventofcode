use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
	let stdin = io::stdin();
	let mut niceness = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let mut nice = false;
		let mut set: HashMap<(char, char), i32> = HashMap::new();
		let (mut b, mut c, mut d) = (' ', ' ', ' ');
		for a in line.chars() {
			if b != ' ' {
				if (b, a) != (c, b) || (b, a) == (d, c) {
					*set.entry((b, a)).or_insert(0) += 1;
				}
			}
			if a == c {
				nice = true;
			}
			d = c;
			c = b;
			b = a;
		}
		if nice && set.values().any(|v| *v > 1) {
			niceness += 1;
		}
	}
	println!("{}", niceness);
}
