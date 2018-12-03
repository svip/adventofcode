use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut totallength = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let mut v: Vec<i32> = line.split('x').map(|c| c.parse::<i32>().unwrap()).collect();
		v.sort_by(|x, y| x.cmp(y));
		totallength += v[0]*2 + v[1]*2 + v[0]*v[1]*v[2];
	}
	println!("{}", totallength);
	
	Ok(())
}
