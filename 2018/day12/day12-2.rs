use std::io;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};

fn main() {
	let stdin = io::stdin();
	let mut pots: HashSet<i32> = HashSet::new();
	let mut changes: HashMap<[bool; 5], bool> = HashMap::new();
	let (mut first, mut last) = (0, 0);
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
					pots.insert(i);
				}
				last = i as i32;
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
	let mut generation: u64 = 1;
	loop {
		let begin = first - 2;
		let end = last + 2;
		let mut newpots: HashSet<i32> = HashSet::new();
		let (mut smallest, mut largest) = (end, begin);
		for pot in begin..end+1 {
			let surrounding: [bool; 5] = [pots.contains(&(pot-2)),
				pots.contains(&(pot-1)),
				pots.contains(&pot),
				pots.contains(&(pot+1)),
				pots.contains(&(pot+2))];
			match changes.get(&surrounding) {
			Some(c) => {
				if *c {
					newpots.insert(pot);
					if pot < first { first = pot; }
					if pot > last { last = pot; }
					if pot < smallest { smallest = pot; }
					if pot > largest { largest = pot; }
				} else {
					if pot < smallest && pot > first { first = pot + 1; }
					if pot > largest && pot < last { last = pot - 1; }
				}
			}
			None => {
				if pot < smallest && pot > first { first = pot + 1; }
				if pot > largest && pot < last { last = pot - 1; }
			}
			}
		}
		pots = newpots.clone();
		
		if generation == 200 {
			// Assume the pattern has been established here.
			let g = 50000000000/generation;
			let mut sum = 0;
			for v in pots.clone() {
				sum += v as i64;
			}
			println!("{}", ((sum/1000)*1000)*g as i64 + sum % 1000);
			break;
		}
		
		generation += 1;
	}
}
