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
	let mut earlier_patterns: Vec<Vec<bool>> = vec![];
	const MAX_GENERATIONS: u64 = 50_000_000_000;
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
		let mut pattern: Vec<bool> = vec![];
		for i in smallest..largest+1 {
			pattern.push(newpots.contains(&i));
		} 
		pots = newpots.clone();
		
		if earlier_patterns.len() > 10 {
			let mut matches = 0;
			let mut count = 0;
			let mut counts: Vec<u64> = vec![];
			'maploop: for a_pattern in earlier_patterns.clone() {
				if a_pattern == pattern {
					matches += 1;
					counts.push(count);
					count = 0;
				}
				count += 1;
			}
			if matches >= 5 {
				if counts[1] == counts[2] && counts[2] == counts[3] && counts[3] == counts[4] {
					if counts[0] + ((MAX_GENERATIONS-counts[0])/counts[1])*counts[1] == MAX_GENERATIONS {
						let diff = MAX_GENERATIONS-generation;
						let mut sum: u64 = 0;
						for v in pots.clone() {
							sum += v as u64 + diff;
						}
						println!("{}", sum);
						break;
					}
				}
			}
		}
		earlier_patterns.push(pattern);
		
		generation += 1;
	}
}
