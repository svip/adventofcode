use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
		
	let mut line = "".to_string();
	stdin.lock().read_to_string(&mut line).unwrap();
	line = line.trim().to_string() + " ";
	for _ in 0..40 {
		let mut new_line = "".to_string();
		let mut work_group = "".to_string();
		let mut work_char: char = ' ';
		for c in line.chars() {
			if c != work_char && work_char != ' ' {
				new_line.push_str(format!("{}{}", work_group.len(), work_char).as_str());
				work_group.clear();
			}
			work_group.push(c);
			work_char = c;
		}
		line = new_line + " ";
	}
	println!("{}", line.trim().len());
}
