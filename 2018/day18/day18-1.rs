use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy,Debug)]
enum Tile {
	Open,
	Trees,
	Lumberyard,
	Unknown,
}

impl Clone for Tile {
	fn clone(&self) -> Tile { *self }
}

impl PartialEq for Tile {
	fn eq(&self, other: &Tile) -> bool {
		match (&self, other) {
			(Tile::Open, Tile::Open) => true,
			(Tile::Trees, Tile::Trees) => true,
			(Tile::Lumberyard, Tile::Lumberyard) => true,
			(Tile::Unknown, Tile::Unknown) => true,
			(_, _) => false,
		}
	}
}

impl fmt::Display for Tile {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Tile::Open => ".",
			Tile::Trees => "|",
			Tile::Lumberyard => "#",
			Tile::Unknown => "?",
		})
	}
}

fn main() {
	let stdin = io::stdin();
	let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
	let (mut max_y, mut max_x) = (0, 0);
	let mut y = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let mut x = 0;
		for c in line.chars() {
			match c {
			'.' => { map.insert((x, y), Tile::Open); }
			'|' => { map.insert((x, y), Tile::Trees); }
			'#' => { map.insert((x, y), Tile::Lumberyard); }
			_ => { }
			}
			if x > max_x {
				max_x = x;
			}
			x += 1;
		}
		if y > max_y {
			max_y = y;
		}
		y += 1;
	}
	const MAX_MINUTES: usize = 10;
	for minute in 0..MAX_MINUTES {
		let now_state = map.clone();
		let open_tiles = now_state.iter().filter(|t| *t.1 == Tile::Open).map(|t| *t.0).collect::<Vec<(i32, i32)>>();
		for open_tile in open_tiles {
			let (x, y) = (open_tile.0, open_tile.1);
			let neighbours: [(i32, i32); 8] = [(x-1, y-1), (x, y-1), (x+1, y-1), (x-1, y), (x+1, y), (x-1, y+1), (x, y+1), (x+1, y+1)];
			let mut tree_count = 0;
			for neighbour in neighbours.iter() {
				if *now_state.get(&neighbour).unwrap_or(&Tile::Unknown) == Tile::Trees {
					tree_count += 1;
				}
			}
			if tree_count >= 3 {
				map.insert(open_tile, Tile::Trees);
			}
		}
		let tree_tiles = now_state.iter().filter(|t| *t.1 == Tile::Trees).map(|t| *t.0).collect::<Vec<(i32, i32)>>();
		for tree_tile in tree_tiles {
			let (x, y) = (tree_tile.0, tree_tile.1);
			let neighbours: [(i32, i32); 8] = [(x-1, y-1), (x, y-1), (x+1, y-1), (x-1, y), (x+1, y), (x-1, y+1), (x, y+1), (x+1, y+1)];
			let mut lumberyard_count = 0;
			for neighbour in neighbours.iter() {
				if *now_state.get(&neighbour).unwrap_or(&Tile::Unknown) == Tile::Lumberyard {
					lumberyard_count += 1;
				}
			}
			if lumberyard_count >= 3 {
				map.insert(tree_tile, Tile::Lumberyard);
			}
		}
		let lumberyards = now_state.iter().filter(|t| *t.1 == Tile::Lumberyard).map(|t| *t.0).collect::<Vec<(i32, i32)>>();
		for lumberyard in lumberyards {
			let (x, y) = (lumberyard.0, lumberyard.1);
			let neighbours: [(i32, i32); 8] = [(x-1, y-1), (x, y-1), (x+1, y-1), (x-1, y), (x+1, y), (x-1, y+1), (x, y+1), (x+1, y+1)];
			let mut lumberyard_count = 0;
			let mut tree_count = 0;
			for neighbour in neighbours.iter() {
				if *now_state.get(&neighbour).unwrap_or(&Tile::Unknown) == Tile::Lumberyard {
					lumberyard_count += 1;
				}
				if *now_state.get(&neighbour).unwrap_or(&Tile::Unknown) == Tile::Trees {
					tree_count += 1;
				}
			}
			if lumberyard_count == 0 || tree_count == 0 {
				map.insert(lumberyard, Tile::Open);
			}
		}
		println!("After {} minute:", minute+1);
		for y in 0..max_y+1 {
			for x in 0..max_x+1 {
				print!("{}", map.get(&(x, y)).unwrap_or(&Tile::Unknown));
			}
			println!("");
		}
	}
	
	let trees = map.iter().filter(|t| *t.1 == Tile::Trees).count();
	let lumberyards = map.iter().filter(|t| *t.1 == Tile::Lumberyard).count();
	println!("{} * {} = {}", trees, lumberyards, trees*lumberyards);
}

