use std::io;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() -> io::Result<()> {
	// We know the full size of the map
	let worldwidth: i32 = 1000;
	let worldheight: i32 = 1000;
	let stdin = io::stdin();
	let mut map: HashMap<i32, i32> = HashMap::with_capacity((worldwidth*worldheight) as usize);
	let mut set: HashSet<i32> = HashSet::new();
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(|c| c == '@' || c == ',' || c == ':' || c == 'x').collect();
		let (claim, left, top, width, height) = (
			v[0].to_string().trim_matches('#').trim().to_string().parse::<i32>().unwrap(),
			v[1].to_string().trim().to_string().parse::<i32>().unwrap(),
			v[2].to_string().parse::<i32>().unwrap(),
			v[3].to_string().trim().to_string().parse::<i32>().unwrap(),
			v[4].to_string().parse::<i32>().unwrap());
		let mut pos = left + top * worldwidth;
		let mut unique = true;
		for _ in 0..height {
			for x in 0..width {
				let t = map.entry(pos + x).or_insert(claim);
				if *t != claim {
					set.remove(&*t);
					*t = 0;
					unique = false;
				}
			}
			pos += worldwidth;
		}
		if unique {
			set.insert(claim);
		}
	}
	for k in &set {
		println!("{}", k);
	}
	
	Ok(())
}

