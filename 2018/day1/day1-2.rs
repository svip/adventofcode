use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
	let mut buffer = String::new();
	let stdin = io::stdin();
	let mut handle = stdin.lock();	
	handle.read_to_string(&mut buffer)?;
	
	let mut f = 0;
	let mut set: HashSet<i64> = HashSet::new();
	let mut lines = buffer.lines();
	loop {
		match lines.next() {
			Some(line) => {
				f += line.to_string().parse::<i64>().unwrap();
				if set.contains(&f) {
					println!("{}", f);
					break;
				}
				set.insert(f);
			}
			None => {
				lines = buffer.lines();
			}
		}
	}
	
	Ok(())
}
