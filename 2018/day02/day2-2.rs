use std::io;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut lines: HashSet<String> = HashSet::new();
	'outer: for l in stdin.lock().lines() {
		let line = l.unwrap();
		'inner: for testline in lines.iter() {
			let mut result = line.clone(); // Our working line.
			// The characters we are testing against.
			let mut testchars = testline.chars();
			// Since the lines are equal in size, we should never be calling
			// .next() on testchars when it would return None.
			for (i, c) in line.char_indices() {
				// Attempt to remove the ones that don't match.
				if c != testchars.next().unwrap() {
					// If we are about to make the string too distinct (more than
					// 1 difference) from the current one, it's not the right one.
					if result.len() == line.len() - 1 {
						continue 'inner;
					}
					result.remove(i);
				}
			}
			// If we ever get down here, it's over.
			println!("{}", result);
			break 'outer;
		}
		lines.insert(line);
	}
	
	Ok(())
}

