use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	println!("{}", stdin.lock().lines().map(|l|
		l.unwrap().chars().map(|c| if c == '(' { 1 } else { -1 }).sum::<i64>()
	).sum::<i64>());
	Ok(())
}
