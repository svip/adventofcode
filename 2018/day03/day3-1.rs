use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
	// We know the full size of the map
	let worldwidth: i32 = 1000;
	let stdin = io::stdin();
	let mut map: [i8; 1000 * 1000] = [0; 1000*1000];
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(|c| c == '@' || c == ',' || c == ':' || c == 'x').collect();
		let (left, top, width, height) = (
			v[1].trim().parse::<i32>().unwrap(),
			v[2].parse::<i32>().unwrap(),
			v[3].trim().parse::<i32>().unwrap(),
			v[4].parse::<i32>().unwrap());
		let mut pos = left + top * worldwidth;
		for _ in 0..height {
			for x in 0..width {
				map[(pos + x) as usize] += 1;
			}
			pos += worldwidth;
		}
	}
	println!("{}", map.iter().filter(|x| *x > &1).count());
	
	Ok(())
}

