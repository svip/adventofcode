use std::io;
use std::io::prelude::*;
use std::cmp::max;
use std::thread;

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
	let mut result_grid: Vec<(usize, usize, usize, i32)> = vec![];
	let mut handles = vec![];
	const DIVISION: usize = 8;
	for xy in 0..HEIGHT/DIVISION {
		let (y1, y2) = (xy*DIVISION+1, xy*DIVISION+DIVISION+1);
		let builder = thread::Builder::new()
						.name(format!("Thread: {}", xy))
						.stack_size(WIDTH*WIDTH*HEIGHT);
		handles.push(builder.spawn(move || {
			let mut sub: Vec<(usize, usize, usize, i32)> = vec![];
			for y in y1..y2 {
				for x in 1..WIDTH+1 {
					let gridid = (y-1)*WIDTH + x-1;
					for size in 1..(WIDTH-(max(x,y)-1))+1 {
						let mut power = 0;
						for ix in 0..size {
							for iy in 0..size {
								power += grid[gridid+(iy*WIDTH)+ix];
							}
						}
						sub.push((x, y, size, power));
					};
				}
			}
			print!(".");
			let _ = io::stdout().flush();
			sub
		}));
	}
	let mut init = true;
	for t in handles {
		let mut res = t.unwrap().join().unwrap();
		if init { println!(""); init = false; }
		result_grid.append(&mut res);
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
