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
	
	//const TOTALWORKERS: usize = 2;
	//const BASETIME: u8 = 0;
	const TOTALWORKERS: usize = 5;
	const BASETIME: u8 = 60;
	let mut workers: [(char, u8); TOTALWORKERS] = [(' ', 0); TOTALWORKERS];
	//let mut working: HashMap<char, (u8, u8)>;
	index.sort_by(|x, y| x.0.cmp(&y.0));
	let mut done: Vec<char> = vec![];
	let mut sec = 0;
	loop {
		let mut toadd = ' ';
		let mut i = 0;
		for l in workers.clone().iter() {
			if l.0 != ' ' {
				let newtime = l.1 - 1;
				if newtime == 0 {
					toadd = l.0;
					workers[i] = (' ', 0);
				} else {
					workers[i] = (l.0, newtime);
				}
			}
			i += 1;
		}
		if toadd != ' ' {
			done.push(toadd);
		}
		if index.len() == 0 && !workers.iter().any(|c| c.0 != ' ') {
			break;
		}
		
		let mut tmp: Vec<char> = vec![];
		for e in index.clone() {
			if e.1 == ' ' || done.contains(&e.1) {
				if e.1 == ' ' || !index.clone().iter().filter(|q| q.0 == e.0 && q.1 != e.1).any(|w| !done.contains(&w.1)) {
					if !tmp.contains(&e.0) {
						tmp.push(e.0);
					}
					if tmp.len() == TOTALWORKERS {
						break;
					}
				}
			}
		}
		let mut i = 0;
		for l in workers.clone().iter() {
			if tmp.len() > 0 && l.0 == ' ' {
				let c = tmp[0];
				tmp.remove(0);
				workers[i] = (c, BASETIME + (c as u8 - 'A' as u8) + 1);
				let mut j = 0i16;
				for e in index.clone() {
					if e.0 == c {
						index.remove(j as usize);
						j -= 1;
					}
					j += 1;
				}
			}
			i += 1;
		}
		println!("{:4} {}{}", sec,
			workers.iter().map(|i| format!("{} ", i.0)).collect::<String>(),
			done.clone().iter().collect::<String>());
		sec += 1;
	}
	
	println!("{:4} {}{}", sec,
		workers.iter().map(|i| format!("{} ", i.0)).collect::<String>(),
		done.clone().iter().collect::<String>());
}
