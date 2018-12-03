use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
	// We know the full size of the map
	let worldwidth: i32 = 1000;
	let stdin = io::stdin();
	let mut map: [i32; 1000*1000] = [0; 1000*1000];
	let mut set: HashSet<i32> = HashSet::new();
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(|c| c == '@' || c == ',' || c == ':' || c == 'x').collect();
		let (claim, left, top, width, height) = (
			v[0].trim_matches(|c| c == '#' || c == ' ').parse::<i32>().unwrap(),
			v[1].trim().parse::<i32>().unwrap(),
			v[2].parse::<i32>().unwrap(),
			v[3].trim().parse::<i32>().unwrap(),
			v[4].parse::<i32>().unwrap());
		let mut pos = left + top * worldwidth;
		let mut unique = true;
		for _ in 0..height {
			for x in 0..width {
				let id: usize = (pos + x) as usize;
				if map[id] == 0 {
					map[id] = claim;
				}
				if map[id] != claim {
					set.remove(&map[id]);
					map[id] = -1;
					unique = false;
				}
			}
			pos += worldwidth;
		}
		if unique {
			set.insert(claim);
		}
	}
	println!("{}", set.iter().next().unwrap());
	
	Ok(())
}

