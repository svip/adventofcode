use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let mut index: Vec<(char, char)> = vec![];
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(" ").collect();
		let (first, before) = (v[1].chars().next().unwrap(), v[7].chars().next().unwrap());
		index.push((before, first));
	}
	'outer: for e in index.clone() {
		for y in index.clone() {
			if e.1 == y.0 {
				continue 'outer;
			}
		}
		index.push((e.1, ' '));
	}
	
	index.sort_by(|x, y| x.0.cmp(&y.0));
	let mut done: Vec<char> = vec![];
	while index.len() > 0 {
		let mut tmp: char = ' ';
		for e in index.clone() {
			if e.1 == ' ' || done.contains(&e.1) {
				if e.1 == ' ' || !index.clone().iter().filter(|q| q.0 == e.0 && q.1 != e.1).any(|w| !done.contains(&w.1)) {
					tmp = e.0;
					break;
				}
			}
		}
		
		let mut i = 0i16;
		for e in index.clone() {
			if e.0 == tmp {
				index.remove(i as usize);
				i -= 1;
			}
			i += 1;
		}
		done.push(tmp);
	}
	
	//let mut list: Vec<(i32, &str)> = index.iter().map(|e| (*e.1, e.0.as_str())).collect();
	//list.sort_by(|x, y| { if x.0 == y.0 { x.1.cmp(&y.1) } else { x.0.cmp(&y.0) } });
	println!("{}", done.iter().collect::<String>());
}
