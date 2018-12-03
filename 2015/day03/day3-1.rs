use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut map: HashMap<(i32, i32), i32> = HashMap::new();
	let mut pos = (0, 0);
	map.insert(pos, 1);
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		for c in line.chars() {
			pos = match c {
				'<' => { (pos.0-1, pos.1) }
				'>' => { (pos.0+1, pos.1) }
				'^' => { (pos.0, pos.1-1) }
				'v' => { (pos.0, pos.1+1) }
				_ => { println!("{:?}", pos); pos }
			};
			*map.entry(pos).or_insert(0) += 1;
		}
	}
	println!("{}", map.values().filter(|x| *x > &0).count());
	
	Ok(())
}
