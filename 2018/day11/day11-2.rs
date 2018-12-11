use std::io;
use std::io::prelude::*;
use std::cmp::max;

fn main() {
	let stdin = io::stdin();
	let serial = stdin.lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
	const WIDTH: usize = 299;
	const HEIGHT: usize = 299;
	let mut grid: [i32; WIDTH*HEIGHT] = [0; WIDTH*HEIGHT];
	let mut sumtable: [i32; WIDTH*HEIGHT] = [0; WIDTH*HEIGHT];
	for y in 1..HEIGHT+1 {
		for x in 1..WIDTH+1 {
			let id = (y-1)*WIDTH + x-1;
			let mut value = x as i32 + 10;
			value *= y as i32;
			value += serial;
			value = value * (x as i32 + 10);
			value = if value >= 100 { (value/100)%10 } else { 0 };
			value -= 5;
			grid[id] = value;
			if x > 1 && y > 1 {
				sumtable[id] = grid[id] + sumtable[id-1] 
					+ sumtable[id-WIDTH] - sumtable[id-WIDTH-1];
			} else if x > 1 {
				sumtable[id] = grid[id] + sumtable[id-1];
			} else {
				sumtable[id] = grid[id];
			}
		}
	}
	let mut result_grid: Vec<(usize, usize, usize, i32)> = vec![];
	for y in 1..HEIGHT+1 {
		for x in 1..WIDTH+1 {
			let gridid = (y-1)*WIDTH + x-1;
			let extreme = WIDTH-max(x,y)+1;
			for size in 1..extreme {
				let power = sumtable[gridid] + sumtable[gridid+size+WIDTH*size]
					- sumtable[gridid+size] - sumtable[gridid+WIDTH*size];
				// Not entirely sure why, but I had an off by one error, so...
				result_grid.push((x+1, y+1, size, power));
			};
		}
	}
	let mut most_power = 0;
	let (mut x, mut y, mut size) = (0, 0, 0);
	for e in result_grid.iter() {
		if e.3 > most_power {
			most_power = e.3;
			x = e.0;
			y = e.1;
			size = e.2;
		}
	}
	println!("{}, {}, {} with {}", x, y, size, most_power);
}
