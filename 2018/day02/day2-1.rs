use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let count = stdin.lock().lines()
		.map(|l| {
			let mut chars = l.unwrap().chars().collect::<Vec<char>>(); 
			chars.sort_by(|x, y| x.cmp(y));
			// First we combine our strings as we see repeats.
			chars.iter().scan(" ".to_string(), |s, &x| {
				if (*s).find(x) != None {
					(*s).push(x);
				} else {
					*s = x.to_string();
				}
				
				Some((*s).clone())
			// If there are three (e.g. "aaa"), there will also be an element
			// in the list (appearing just beforehand!) that is two (e.g. "aa"),
			// therefore we subtract when we see len() == 3.
			}).fold((0, 0), |a, x| {
				if x.len() == 3 { (a.0 - 1, a.1 + 1) }
				else if x.len() == 2 { (a.0 + 1, a.1) }
				else { (a.0, a.1) }
			})
		})
		.fold((0, 0), |a, x| (a.0 + (x.0 as i64).signum(), a.1 + (x.1 as i64).signum()));
	println!("{}", count.0*count.1);
	
	Ok(())
}

