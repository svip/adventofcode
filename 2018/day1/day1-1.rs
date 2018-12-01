use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
	let mut f = 0;
	let stdin = io::stdin();
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let tmp = line[1..].to_string().parse::<i64>().unwrap();
		if line.chars().next().unwrap() == '+' {
			f += tmp;
		} else {
			f -= tmp;
		}
		println!("{} ({})", f, line);
	}
	
	println!("{}", f);
	Ok(())
}
