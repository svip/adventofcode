use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let (mut v2, mut v3) = (0, 0);
	let mut set: HashMap<char, i32> = HashMap::new();
	for l in stdin.lock().lines() {
		set.clear();
		for c in l.unwrap().chars() {
			*set.entry(c).or_insert(0) += 1;
		}
		
		if set.values().any(|&v| v == 2) {
			v2 += 1;
		}
		if set.values().any(|&v| v == 3) {
			v3 += 1;
		}
	}
	println!("{}", v2*v3);
	
	Ok(())
}

