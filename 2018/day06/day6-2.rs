use std::io;
use std::io::prelude::*;
use std::cmp::{min, max};
use std::thread;

fn main() {
	let stdin = io::stdin();
	let (mut x1, mut y1, mut x2, mut y2) = (0, 0, 0, 0);
	let mut init = true;
	let mut coords = vec![];
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(", ").collect();
		let (x, y) = (v[0].parse::<u16>().unwrap(), v[1].parse::<u16>().unwrap());
		if init {
			x1 = x; y1 = y; x2 = x; y2 = y;
			init = false;
		} else {
			if x < x1 { x1 = x; }
			if y < y1 { y1 = y; }
			if x > x2 { x2 = x; }
			if y > y2 { y2 = y; }
		}
		coords.push((x, y));
	}
	let limit = 10000;
	let mut handles = vec![];
	for x in x1..x2+1 {
		let nc = coords.clone();
		handles.push(thread::spawn(move || {
			let mut i = 0;
			'y: for y in y1..y2+1 {
				let mut sum = 0;
				for c in nc.clone() {
					let distance = (max(c.0, x) - min(c.0, x)) + (max(c.1, y) - min(c.1, y));
					sum += distance as u32;
					if sum >= limit {
						continue 'y;
					}
				}
				i += 1;
			}
			i
		}));
	}
	let mut region = 0;
	for h in handles {
		region += h.join().unwrap();
	}
	println!("{}", region);
}
