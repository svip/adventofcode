use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> io::Result<()> {
	// We know the full size of the map
	let worldwidth: i32 = 1000;
	let worldheight: i32 = 1000;
	let stdin = io::stdin();
	let mut map: HashMap<i32, i32> = HashMap::with_capacity((worldwidth*worldheight) as usize);
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(|c| c == '@' || c == ',' || c == ':' || c == 'x').collect();
		let (left, top, width, height) = (
			v[1].to_string().trim().to_string().parse::<i32>().unwrap(),
			v[2].to_string().parse::<i32>().unwrap(),
			v[3].to_string().trim().to_string().parse::<i32>().unwrap(),
			v[4].to_string().parse::<i32>().unwrap());
		let mut pos = left + top * worldwidth;
		for _ in 0..height {
			for x in 0..width {
				*map.entry(pos + x).or_insert(0) += 1;
			}
			pos += worldwidth;
		}
	}
	let mut c = 0;
	for v in map.values() {
		if *v > 1 {
			c += 1;
		}
	}
	println!("{}", c);
	
	Ok(())
}

