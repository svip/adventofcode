use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
	let stdin = io::stdin();
	let mut pots: HashMap<i32, bool> = HashMap::new();
	let mut changes: HashMap<[bool; 5], bool> = HashMap::new();
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		if line == "" {
			continue;
		}
		if pots.len() == 0 {
			// assume initial state
			let state: String = line.split(' ').collect::<Vec<&str>>()[2].to_string();
			let mut i = 0;
			for c in state.chars() {
				if c == '#' {
					pots.insert(i, true);
				} else {
					pots.insert(i, false);
				}
				i += 1;
			}
		} else {
			let v: Vec<&str> = line.split(" => ").collect();
			let mut from: [bool; 5] = [false; 5];
			let mut i = 0;
			for c in v[0].chars() {
				if c == '#' {
					from[i] = true;
				}
				i += 1;
			}
			let result = v[1] == "#";
			changes.insert(from, result);
		}
	}
	let (mut first, mut last) = (0, pots.len() as i32 -1);
	let mut generation = 1;
	loop {
		let begin = first - 2;
		let end = last + 2;
		let mut newpots: HashMap<i32, bool> = HashMap::new();
		for pot in begin..end+1 {
			let surrounding: [bool; 5] = [*pots.get(&(pot-2)).unwrap_or(&false),
				*pots.get(&(pot-1)).unwrap_or(&false),
				*pots.get(&pot).unwrap_or(&false),
				*pots.get(&(pot+1)).unwrap_or(&false),
				*pots.get(&(pot+2)).unwrap_or(&false)];
			match changes.get(&surrounding) {
			Some(c) => {
				newpots.insert(pot, *c);
				if pot < first { first = pot; }
				if pot > last { last = pot; }
			}
			None => {}
			}
		}
		pots = newpots.clone();
		
		if generation >= 20 {
			break;
		}
		generation += 1;
	}
	let mut sum = 0;
	for (k, v) in pots {
		if v { sum += k; }
	}
	println!("{}", sum);
}
