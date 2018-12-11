use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let serial = stdin.lock().lines().next().unwrap().unwrap().parse::<i32>().unwrap();
	const WIDTH: usize = 299;
	const HEIGHT: usize = 299;
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
			if x > 1 && y > 1 {
				sumtable[id] = value + sumtable[id-1] 
					+ sumtable[id-WIDTH] - sumtable[id-WIDTH-1];
			} else if x > 1 {
				sumtable[id] = value + sumtable[id-1];
			} else {
				sumtable[id] = value;
			}
		}
	}
	let mut most_power = 0;
	let (mut result_x, mut result_y) = (0, 0);
	for y in 1..HEIGHT-2 {
		for x in 1..WIDTH-2 {
			let gridid = (y-1)*WIDTH + x-1;
			let size = 3;
			let power = sumtable[gridid] + sumtable[gridid+size+WIDTH*size]
				- sumtable[gridid+size] - sumtable[gridid+WIDTH*size];
			if power > most_power {
				most_power = power;
				// Not entirely sure why, but I had an off by one error, so...
				result_x = x+1;
				result_y = y+1;
			}
		}
	}
	println!("{}, {} with {}", result_x, result_y, most_power);
}

