use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let mut map: [bool; 1000*1000] = [false; 1000*1000];
	
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(|c| c == ' ' || c == 'l' || c == ',').collect();
		let (action, dir, from_x, from_y, to_x, to_y) = (
			v[0], v[1], 
			v[2].parse::<i64>().unwrap(),
			v[3].parse::<i64>().unwrap(),
			v[5].parse::<i64>().unwrap(),
			v[6].parse::<i64>().unwrap());
		let what = match action {
			"turn" => {
				match dir {
					"on" => { 1 }
					"off" => { 2 }
					_ => { -1 }
				}
			}
			"togg" => { 3 }
			_ => { -1 }
		};
		for y in from_y..to_y+1 {
			for x in from_x..to_x+1 {
				let id = (y*1000+x) as usize;
				map[id] = match what {
					1 => { true }
					2 => { false }
					3 => { !map[id] }
					_ => { map[id] }
				}
			}
		}
	}
	let c = map.iter().filter(|i| **i).count();
	println!("{}", c);
}
