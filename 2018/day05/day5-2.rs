use std::io;
use std::io::prelude::*;
use std::thread;
use std::collections::{HashSet, HashMap};

fn react(s: String) -> String {
	let mut result = "".to_string();
	let mut p: char = ' ';
	for c in s.chars() {
		if c.is_whitespace() {
			continue;
		}
		if (c.is_lowercase() && p.is_uppercase() && c == p.to_ascii_lowercase())
			|| (c.is_uppercase() && p.is_lowercase() && c == p.to_ascii_uppercase()) {
			result.pop();
			p = ' ';
		} else {
			result.push(c);
			p = c;
		}
	}
	result
}

fn fullreact(s: String) -> String {
	let mut pl = 0;
	let mut result = s.clone();
	loop {
		result = react(result);
		if pl == result.len() {
			break;
		}
		pl = result.len();
	}
	result
}

fn remove_and_react(s: String, to_remove: char) -> String {
	let mut result = "".to_string();
	for c in s.chars() {
		if c.to_lowercase().next().unwrap() != to_remove {
			result.push(c);
		}
	}
	fullreact(result)
}

fn main() {
	let stdin = io::stdin();
	let mut buffer = "".to_string();
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		buffer.push_str(line.as_str());
	}
	let mut used_characters: HashSet<char> = HashSet::new();
	let mut handles = vec![];
	for c in buffer.clone().chars() {
		let l = c.to_ascii_lowercase();
		if used_characters.get(&l) == None {
			used_characters.insert(l);
			let text = buffer.clone();
			handles.push(thread::spawn(move || {
				(l, remove_and_react(text, l).len())
			}));
		}
	}
	let mut map: HashMap<char, usize> = HashMap::new();
	for t in handles {
		let res = t.join().unwrap();
		map.insert(res.0, res.1);
	}
	let mut least = buffer.len();
	for v in map.values() {
		if *v < least {
			least = *v;
		}
	}
	println!("{}", least);
	
}
