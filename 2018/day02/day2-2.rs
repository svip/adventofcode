use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines: HashSet<String> = HashSet::new();
	println!("{}", stdin.lock().lines().map(|l| {
		let line = l.unwrap();
		// Create a list of previous strings with only the characters they
		// have in common.  And find the longest of those.
		let s = lines.iter().fold(String::new(), |mut longest, x| {
			let s = x.chars().zip(line.chars())
				.filter_map(|x| if x.0 == x.1 { Some(x.0) } else { None } )
				.collect::<String>();
			if s.len() > (*longest).len() { longest.clear(); longest.push_str(s.as_str()); }
			longest
		});
		lines.insert(line);	
		s
	}).fold(String::new(), |mut longest, x| {
		if x.len() > (*longest).len() { longest.clear(); longest.push_str(x.as_str()); }
		longest
	}));
	
	Ok(())
}

