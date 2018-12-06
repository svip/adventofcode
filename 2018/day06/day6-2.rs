use std::io;
use std::io::prelude::*;
use std::cmp::{min, max};
use std::collections::HashMap;

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
	let mut map: HashMap<(u16, u16), u32> = HashMap::new();
	for x in x1..x2+1 {
		for y in y1..y2+1 {
			let mut sum = 0;
			for c in coords.clone() {
				let distance = (max(c.0, x) - min(c.0, x)) + (max(c.1, y) - min(c.1, y));
				sum += distance as u32;
			}
			map.insert((x, y), sum);
		}
	}
	let size = map.values().filter(|s| **s < 10000).count();
	println!("{}", size);
}
