use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy)]
enum Tile {
	Sand,
	Clay,
	Spring,
	WaterRest,
	WaterFlowing(bool),
}

impl Clone for Tile {
	fn clone(&self) -> Tile { *self }
}

impl PartialEq for Tile {
	fn eq(&self, other: &Tile) -> bool {
		match (&self, other) {
			(Tile::Sand, Tile::Sand) => true,
			(Tile::Clay, Tile::Clay) => true,
			(Tile::Spring, Tile::Spring) => true,
			(Tile::WaterRest, Tile::WaterRest) => true,
			(Tile::WaterFlowing(false), Tile::WaterFlowing(false)) => true,
			(Tile::WaterFlowing(true), Tile::WaterFlowing(true)) => true,
			(_, _) => false,
		}
	}
}

impl fmt::Display for Tile {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Tile::Sand => ".",
			Tile::Clay => "#",
			Tile::Spring => "+",
			Tile::WaterRest => "~",
			Tile::WaterFlowing(true) => "|",
			Tile::WaterFlowing(false) => "!",
		})
	}
}

impl fmt::Debug for Tile {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Tile({})", match self {
			Tile::Sand => "Sand",
			Tile::Clay => "Clay",
			Tile::Spring => "Spring",
			Tile::WaterRest => "WaterRest",
			Tile::WaterFlowing(true) => "WaterFlowing(true)",
			Tile::WaterFlowing(false) => "WaterFlowing(false)",
		})
	}
}

fn main() {
	let stdin = io::stdin();
	let mut map: HashMap<(i32, i32), Tile> = HashMap::new();
	map.insert((500, 0), Tile::Spring);
	let (mut x_min, mut x_max, mut y_min, mut y_max) = (500, 500, 500, 0);
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(", ").collect();
		let (first, last) = (v[0].split('=').collect::<Vec<&str>>(),
			v[1].split('=').collect::<Vec<&str>>());
		let is_fixed_y = first[0] == "y";
		let first_value = first[1].parse::<i32>().unwrap();
		let range = last[1].split("..").collect::<Vec<&str>>();
		let (range_from, range_to) = (range[0].parse::<i32>().unwrap(),
			range[1].parse::<i32>().unwrap());
		if is_fixed_y {
			let y = first_value;
			if y > y_max {
				y_max = y;
			}
			if y < y_min {
				y_min = y;
			}
			for x in range_from..range_to+1 {
				if x < x_min {
					x_min = x;
				}
				if x > x_max {
					x_max = x;
				}
				map.insert((x, y), Tile::Clay);
			}
		} else {
			let x = first_value;
			if x < x_min {
				x_min = x;
			}
			if x > x_max {
				x_max = x;
			}
			for y in range_from..range_to+1 {
				if y > y_max {
					y_max = y;
				}
				if y < y_min {
					y_min = y;
				}
				map.insert((x, y), Tile::Clay);
			}
		}
	}
	// The spring is flowing...
	map.insert((500, 1), Tile::WaterFlowing(false));
	let mut draw_from = (500, 0);
	let mut last_count = 0;
	loop {
		let mut points = map.clone().iter()
			.filter(|t| *t.1 == Tile::WaterFlowing(false) && (t.0).1 >= 1 && (t.0).1 <= y_max)
			.map(|t| *t.0).collect::<Vec<(i32, i32)>>();
		points.sort_by(|x, y| if x.1 == y.1 { x.0.cmp(&y.0) } else { x.1.cmp(&y.1) } );
		points.reverse();
		if points.len() > 0 {
			draw_from = points[0];
		}
		for p in points.clone() {
			let (x, y) = (p.0, p.1);
			if *map.clone().get(&p).unwrap_or(&Tile::Sand) != Tile::WaterFlowing(false) {
				continue;
			}
			let below = (*map.clone().get(&(x, y+1)).unwrap_or(&Tile::Sand)).clone();
			if below == Tile::Sand {
				map.insert((x, y+1), Tile::WaterFlowing(false));
				map.insert((x, y), Tile::WaterFlowing(true));
			} else if below != Tile::WaterFlowing(true) && below != Tile::WaterFlowing(false) {
				let neighbours: [(i32, i32); 2] = [(x-1, y), (x+1, y)];
				let mut flowed = false;
				for neighbour in neighbours.iter() {
					if *map.clone().get(&neighbour).unwrap_or(&Tile::Sand) == Tile::Sand {
						map.insert(*neighbour, Tile::WaterFlowing(false));
						flowed = true;
					}
				}
				// And now we're settled.
				map.insert(p, Tile::WaterFlowing(true));
				if !flowed {
					// We couldn't flow?  Maybe we need to rest!
					let mut found_opening = false;
					// Search left first
					let mut points: Vec<(i32, i32)> = vec![(x, y)];
					let mut work_x = x - 1;
					let mut going_left = true;
					while !found_opening {
						match map.clone().get(&(work_x, y)).unwrap_or(&Tile::Sand) {
						Tile::WaterFlowing(_) => {
							points.push((work_x, y));
							let below = (*map.clone().get(&(work_x, y+1)).unwrap_or(&Tile::Sand)).clone();
							if below != Tile::Clay && below != Tile::WaterRest {
								found_opening = true;
							}
						}
						Tile::Clay => {
							if !going_left {
								break;
							} else {
								work_x = x;
								going_left = false;
							}
						}
						_ => { found_opening = true; }
						}
						if going_left {
							work_x -= 1;
						} else {
							work_x += 1;
						}
					}
					if !found_opening {
						for point in points {
							map.insert(point, Tile::WaterRest);
							let above = (point.0, point.1-1);
							if *map.clone().get(&above).unwrap_or(&Tile::Sand) == Tile::WaterFlowing(true) {
								map.insert(above, Tile::WaterFlowing(false));
							}
						}
					}
				}
			} else {
				map.insert(p, Tile::WaterFlowing(true));
			}
		}
		
		let count = map.iter()
			.filter(|t| *t.1 == Tile::WaterRest && (t.0).1 >= y_min && (t.0).1 <= y_max)
			.count();
		if count != last_count { // draw less
			println!("{}, {} -> {}, {} ({}, {} -> {}, {}) ({})", 
				draw_from.0-15, draw_from.1-10,
				draw_from.0+15, draw_from.1+10,
				x_min, y_min, x_max, y_max, points.len());
			for y in draw_from.1-10..draw_from.1+10 {
				for x in draw_from.0-15..draw_from.0+15 {
					print!("{}", map.get(&(x, y)).unwrap_or(&Tile::Sand));
				}
				println!("");
			}
			if points.len() != 0 {
				println!("{}", count);
			}
		}
		last_count = count;
		if points.len() == 0 {
			break;
		}
	}
	
	println!("{}", map.iter()
			.filter(|t| *t.1 == Tile::WaterRest && (t.0).1 >= y_min && (t.0).1 <= y_max)
			.count());
}

