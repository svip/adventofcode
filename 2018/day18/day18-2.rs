use std::io;
use std::io::prelude::*;
use std::fmt;

#[derive(Copy,Debug,Eq)]
enum Tile {
	Open,
	Trees,
	Lumberyard,
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
		})
	}
}

fn main() {
	let stdin = io::stdin();
	const WIDTH: usize = 50;
	const HEIGHT: usize = 50;
	const MAP_SIZE: usize = WIDTH*HEIGHT;
	let mut map: [Tile; MAP_SIZE] = [Tile::Open; MAP_SIZE];
	let mut y = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let mut x = 0;
		for c in line.chars() {
			let id = x+y*WIDTH;
			match c {
			'.' => { map[id] = Tile::Open; }
			'|' => { map[id] = Tile::Trees; }
			'#' => { map[id] = Tile::Lumberyard; }
			_ => { }
			}
			x += 1;
		}
		y += 1;
	}
	const MAX_MINUTES: usize = 1_000_000_000;
	let mut earlier_maps: Vec<[Tile; MAP_SIZE]> = vec![];
	let mut change_map: [bool; MAP_SIZE] = [true; MAP_SIZE];
	let mut to_modify: [bool; MAP_SIZE];
	'main: for minute in 0..MAX_MINUTES {
		let now_state = map.clone();
		to_modify = [false; MAP_SIZE];
		for t in change_map.iter().enumerate().filter(|t| *t.1) {
			let id = t.0;
			let (x, y) = (id%WIDTH, id/WIDTH);
			let tile = now_state[id];
			let neighbours: [usize; 8] = [
				if x == 0 || y == 0 { MAP_SIZE } else { id-WIDTH-1 },
				if y == 0 { MAP_SIZE } else { id-WIDTH },
				if y == 0 || x == WIDTH-1 { MAP_SIZE } else { id-WIDTH+1},
				if x == 0 { MAP_SIZE } else { id-1},
				if x == WIDTH-1 { MAP_SIZE } else { id+1},
				if x == 0 || y == HEIGHT-1 { MAP_SIZE } else { id+WIDTH-1 },
				if y == HEIGHT-1 { MAP_SIZE} else { id+WIDTH},
				if x == WIDTH-1 || y == HEIGHT-1 { MAP_SIZE } else { id+WIDTH+1 }];
			match tile {
			Tile::Open => {
				if neighbours.iter().fold(0, |a, t| if *t < MAP_SIZE && now_state[*t] == Tile::Trees { a + 1 } else { a }) >= 3 {
					to_modify[id] = true;
					map[id] = Tile::Trees;
				} else {
					continue;
				}
			}
			Tile::Trees => {
				if neighbours.iter().fold(0, |a, t| if *t < MAP_SIZE && now_state[*t] == Tile::Lumberyard { a + 1 } else { a }) >= 3 {
					to_modify[id] = true;
					map[id] = Tile::Lumberyard;
				} else {
					continue;
				}
			}
			Tile::Lumberyard => {
				if neighbours.iter().fold(0, |a, t| if *t < MAP_SIZE && now_state[*t] == Tile::Trees { a + 1 } else { a }) == 0 || neighbours.iter().fold(0, |a, t| if *t < MAP_SIZE && now_state[*t] == Tile::Lumberyard { a + 1 } else { a }) == 0 {
					to_modify[id] = true;
					map[id] = Tile::Open;
				} else {
					continue;
				}
			}
			}
			for neighbour in neighbours.iter().filter(|t| **t < MAP_SIZE) {
				to_modify[*neighbour] = true;
			}
		}
		if minute > 0 {
			let mut matches = 0;
			let mut count = 0;
			let mut counts: Vec<usize> = vec![];
			'maploop: for a_map in earlier_maps.clone() {
				for y in 0..HEIGHT {
					for x in 0..WIDTH {
						let id = x+y*WIDTH;
						if a_map[id] != map[id] {
							count += 1;
							continue 'maploop;
						}
					}
				}
				matches += 1;
				counts.push(count+1);
				count = 0;
			}
			if matches >= 5 {
				if counts[1] == counts[2] && counts[2] == counts[3] && counts[3] == counts[4] {
					if counts[0] + ((MAX_MINUTES-counts[0])/counts[1])*counts[1] == MAX_MINUTES {
						break;
					}
				}
			}
		}
		earlier_maps.push(map);
		change_map = to_modify;
	}
	
	let trees = map.iter().filter(|t| **t == Tile::Trees).count();
	let lumberyards = map.iter().filter(|t| **t == Tile::Lumberyard).count();
	println!("{} * {} = {}", trees, lumberyards, trees*lumberyards);
}
// wrong answer: 178425 (too low)
