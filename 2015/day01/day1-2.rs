use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let (mut pos, mut floor) = (1, 0);
	'outer: for l in stdin.lock().lines() {
		let line = l.unwrap();
		for (i, c) in line.char_indices() {
			floor += if c == '(' { 1 } else { -1 };
			if floor == -1 {
				println!("{}", pos+i);
				break 'outer;
			} 
		}
		pos += line.len();
	}
	Ok(())
}
