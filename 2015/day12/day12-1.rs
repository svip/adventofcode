use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let mut sum = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(|c: char| !c.is_numeric() && c != '-').collect();
		for s in v {
			match s.parse::<i32>() {
			Ok(i) => { sum += i; }
			Err(_) => {}
			}
		}
	}
	
	println!("{}", sum);
}
