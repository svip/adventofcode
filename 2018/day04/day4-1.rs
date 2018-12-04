use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

enum Action {
	StartsShift(i32),
	FallsAsleep,
	WakesUp,
	Unknown,
}

fn main() {
	let stdin = io::stdin();
	// sort, month, day, hour, minute
	let mut logs: Vec<(u64, u16, u16, u16, u16, Action)> = vec![];
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(|c| c == ' ' || c == '-' || c == ':').collect();
		let (month, day, hour, minute, stat) = (
			v[1].parse::<u16>().unwrap(),
			v[2].parse::<u16>().unwrap(),
			v[3].parse::<u16>().unwrap(),
			v[4].trim_matches(']').parse::<u16>().unwrap(),
			v[5]);
		let action = match stat {
			"falls" => { Action::FallsAsleep }
			"wakes" => { Action::WakesUp }
			"Guard" => { 
				let id = v[6].trim_matches('#').parse::<i32>().unwrap();
				Action::StartsShift(id)
			}
			_ => { Action::Unknown }
		};
		let sort = (month as u64)*43830+(day as u64)*1440+(hour as u64)*60+minute as u64;
		logs.push((sort, month, day, hour, minute, action));
	}
	logs.sort_by(|x, y| x.0.cmp(&y.0));
	
	let mut curid = 0;
	let mut curminute = 0;
	let mut curday: String = "".to_string();
	let mut shift: [bool; 60] = [false; 60];
	let mut mostasleep: HashMap<i32, i32> = HashMap::new();
	let mut shifts: HashMap<(i32, String), [bool; 60]> = HashMap::new();
	for line in logs {
		match line.5 {
			Action::StartsShift(id) => {
				if curid > 0 {
					*mostasleep.entry(curid).or_insert(0) += shift.iter().filter(|i| **i).count() as i32;
					shifts.insert((curid, curday.clone()), shift);
				}
				curid = id;
				shift = [false; 60];
			}
			Action::FallsAsleep => {
				curday = format!("{}-{}", line.1, line.2);
				curminute = line.4;
			}
			Action::WakesUp => {
				for i in curminute..line.4 {
					shift[i as usize] = true;
				}
			}
			_ => { }
		}
	}
	
	let mut most = 0;
	let mut id = 0;
	for (key, val) in mostasleep.iter() {
		if *val > most {
			most = *val;
			id = *key;
		}
	}
	
	let mut combinedshift: [i32; 60] = [0; 60];
	for (key, val) in shifts.iter() {
		if (*key).0 == id {
			let mut i = 0;
			for v in (*val).iter() {
				if *v {
					combinedshift[i] = combinedshift[i] + 1;
				}
				i += 1;
			}
		}
	}
	let mut i = 0;
	most = 0;
	let mut minute = 0;
	for v in combinedshift.iter() {
		if *v > most {
			most = *v;
			minute = i;
		}
		i += 1;
	}
	println!("{} * {} = {}", id, minute, id*minute);
}

