use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut set: HashMap<char, i32> = HashMap::new();
	let count = stdin.lock().lines().map(|x| {
		set.clear();
		for c in x.unwrap().chars() {
			*set.entry(c).or_insert(0) += 1;
		}
		set.values().fold((false,false), |a, &x| {
			if x == 2 { (true, a.1) }
			else if x == 3 { (a.0, true) }
			else { (a.0, a.1) }
		})
	}).fold((0,0), |a, x| (a.0 + (x.0 as i64), a.1 + (x.1 as i64)));
	println!("{}", count.0*count.1);
	
	Ok(())
}

