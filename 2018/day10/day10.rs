use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
	let stdin = io::stdin();
	let mut map: HashMap<(i32, i32), HashMap<u32, (i32, i32)>> = HashMap::new();
	let (mut x1, mut y1, mut x2, mut y2) = (0, 0, 0, 0);
	let mut p = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(|c| c == '<' || c == '>' || c == ',').collect();
		let (x, y, vx, vy) = (
			v[1].trim().parse::<i32>().unwrap(),
			v[2].trim().parse::<i32>().unwrap(),
			v[4].trim().parse::<i32>().unwrap(),
			v[5].trim().parse::<i32>().unwrap());
		if x < x1 { x1 = x; }
		if y < y1 { y1 = y; }
		if x > x2 { x2 = x; }
		if y > y2 { y2 = y; }
		let e = map.entry((x, y)).or_insert(HashMap::new());
		(*e).insert(p, (vx, vy));
		p += 1;
	}
	let mut seconds = 0;
	loop {
		for (coord, list) in map.clone() {
			for (p, item) in list.clone() {
				let newcoord = (coord.0+item.0, coord.1+item.1);
				{
					let newe = map.entry(newcoord).or_insert(HashMap::new());
					(*newe).insert(p, item);
				}
				match map.get_mut(&coord) {
				Some(e) => {
					e.remove(&p);
				}
				None => {}
				};
			}
			if map.clone().get(&coord).unwrap().len() == 0 {
				map.remove(&coord);
			}
		}
		let mut one_without_neighbour = false;
		let (mut tx1, mut ty1, mut tx2, mut ty2) = (0, 0, 0, 0);
		let mut init = true;
		for (coord, _) in map.clone() {
			if init {
				tx1 = coord.0; ty1 = coord.1;
				tx2 = coord.0; ty2 = coord.1;
				init = false;
			}
			if tx1 > coord.0 { tx1 = coord.0; }
			if ty1 > coord.1 { ty1 = coord.1; }
			if tx2 < coord.0 { tx2 = coord.0; }
			if ty2 < coord.1 { ty2 = coord.1; }
			let mut has_neighbour = false;
			let points = vec![(coord.0-1, coord.1), (coord.0+1, coord.1),
				(coord.0, coord.1-1), (coord.0, coord.1+1),
				(coord.0+1, coord.1+1), (coord.0-1, coord.1+1),
				(coord.0-1, coord.1-1), (coord.0+1, coord.1-1)];
			for p in points {
				match map.clone().get(&p) {
				Some(_) => { has_neighbour = true; } None => { }
				}
				if has_neighbour { break; }
			}
			if !has_neighbour {
				one_without_neighbour = true;
			}
			if one_without_neighbour {
				break;
			}
		}
		if !one_without_neighbour {
			println!("");
			println!("Second {}:", seconds+1);
			for y in ty1..ty2+1 {
				for x in tx1..tx2+1 {
					match map.get(&(x, y)) { 
					Some(_) => { print!("#"); }
					None => { print!("."); }
					}
				}
				println!("");
			}
			break;
		}
		if seconds % 100 == 0 {
			print!(".");
			let _ = io::stdout().flush();
		}
		seconds += 1;
	}
}
