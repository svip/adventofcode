use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let mut map: [i32; 1000*1000] = [0; 1000*1000];
	
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
					"off" => { -1 }
					_ => { 0 }
				}
			}
			"togg" => { 2 }
			_ => { 0 }
		};
		for y in from_y..to_y+1 {
			for x in from_x..to_x+1 {
				let id = (y*1000+x) as usize;
				if what == -1 && map[id] == 0 {
					continue;
				}
				map[id] = map[id] + what;
			}
		}
	}
	let c = map.iter().fold(0, |a, i| a + i);
	println!("{}", c);
}
