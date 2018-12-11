use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let serial = stdin.lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
	const WIDTH: usize = 299;
	const HEIGHT: usize = 299;
	let mut grid: [i32; WIDTH*HEIGHT] = [0; WIDTH*HEIGHT];
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
		}
	}
	let mut result_grid: [(usize, usize, i32); (WIDTH-2)*(HEIGHT-2)] = [(0,0,0); (WIDTH-2)*(HEIGHT-2)];
	for y in 1..HEIGHT-1 {
		for x in 1..WIDTH-1 {
			let id = (y-1)*(WIDTH-2) + x-1;
			let gridid = (y-1)*WIDTH + x-1;
			let power = grid[gridid] + grid[gridid+1] + grid[gridid+2]
				+ grid[gridid+WIDTH] + grid[gridid+WIDTH+1] + grid[gridid+WIDTH+2]
				+ grid[gridid+WIDTH*2] + grid[gridid+WIDTH*2+1] + grid[gridid+WIDTH*2+2];
			result_grid[id] = (x, y, power);
		}
	}
	let mut most_power = 0;
	let (mut x, mut y) = (0, 0);
	for e in result_grid.iter() {
		if e.2 > most_power {
			most_power = e.2;
			x = e.0;
			y = e.1;
		}
	}
	println!("{}, {} with {}", x, y, most_power);
}
