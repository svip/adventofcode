use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines: HashMap<String, String> = HashMap::new();
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		// Create a list of previous strings with only the characters they
		// have in common.
		let mut map: Vec<String> = lines.iter().map(|x| 
			x.0.chars().zip(line.chars())
			.filter(|x| x.0 == x.1)
			.map(|x| x.0)
			.collect::<String>()
		).collect::<Vec<String>>();
		
		// Then sort the list, so the longest one is last and then add that
		// element to our list of previous strings (as well as the characters of
		// the one it had the most in common with).
		map.sort_by(|x, y| x.len().cmp(&y.len()));		
		match map.pop() {
			Some(s) => { lines.insert(line, s); }
			None => { lines.insert(line, "".to_string()); }
		}
	}
	// Turn our list of values() (i.e. our common character lists) into a 
	// vector so we can sort it, and take the first (longest) element.
	let mut values: Vec<String> = lines.values().map(|v| v.clone()).collect();
	values.sort_by(|x, y| y.len().cmp(&x.len()));
	println!("{}", values[0]);
	
	Ok(())
}

