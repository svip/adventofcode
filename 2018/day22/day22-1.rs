use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy,Clone,PartialEq,Debug)]
enum Tile {
	Rocky,
	Wet,
	Narrow,
}

impl fmt::Display for Tile {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Tile::Rocky => ".",
			Tile::Wet => "=",
			Tile::Narrow => "|",
		})
	}
}

fn main() {
	let stdin = io::stdin();
	let mut depth = 0;
	let mut target = (0,0);
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(' ').collect();
		if v[0] == "depth:" {
			depth = v[1].parse::<u32>().unwrap();
		} else {
			let vv: Vec<&str> = v[1].split(',').collect();
			target = (vv[0].parse::<u32>().unwrap(), vv[1].parse::<u32>().unwrap());
		}
	}
	let mut erosion_map: HashMap<(u32, u32), u32> = HashMap::new();
	let mut map: HashMap<(u32, u32), Tile> = HashMap::new();
	for x in 0..target.0+1 {
		for y in 0..target.1+1 {
			let geologic_index = if (x == 0 && y == 0) || (x == target.0 && y == target.1) { 0 }
				else if y == 0 { x * 16807 }
				else if x == 0 { y * 48271 }
				else { erosion_map.get(&(x-1, y)).unwrap() * erosion_map.get(&(x, y-1)).unwrap() };
			let erosion_level = (geologic_index + depth) % 20183;
			erosion_map.insert((x, y), erosion_level);
			let t = match erosion_level % 3 {
			0 => Tile::Rocky,
			1 => Tile::Wet,
			2 => Tile::Narrow,
			_ => panic!("How can anything % 3 be something else than 0,1,2?!?"),
			};
			map.insert((x, y), t);
		}
	}
	let risk = map.iter()
		.fold(0, |a, x| if (x.0).0 <= target.0 && (x.0).1 <= target.1 { a + match x.1 {
		Tile::Rocky => 0, Tile::Wet => 1, Tile::Narrow => 2,
		} } else { a });
	println!("{}", risk);
}
