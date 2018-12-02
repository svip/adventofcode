use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	println!("{}", stdin.lock().lines().map(|l|
		l.unwrap().to_string().parse::<i64>().unwrap()
	).sum::<i64>());
	Ok(())
}
