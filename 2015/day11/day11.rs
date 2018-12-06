use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
		
	let mut line = "".to_string();
	stdin.lock().read_to_string(&mut line).unwrap();
	let mut chars: [char; 8] = [' '; 8];
	let mut i = 0;
	for c in line.trim().to_string().chars() {
		chars[i] = c;
		i += 1;
		if i == 8 { break; }
	}
	
	loop {
		let mut c = 7;
		chars[c] = (chars[c] as u8 + 1) as char;
		while chars[c] == ('z' as u8 + 1) as char {
			chars[c] = 'a';
			c -= 1;
			chars[c] = (chars[c] as u8 + 1) as char;
		}
		if chars.iter().any(|c| *c == 'i' || *c == 'o' || *c == 'l') {
			continue;
		}
		let (mut a, mut b, mut p) = (' ', ' ', ' ');
		let mut valid = false;
		let mut dc = 0;
		for c in chars.iter() {
			if *c as u8 == (b as u8 + 1) && *c as u8 == (a as u8 + 2) {
				valid = true;
			}
			if *c == p {
				dc += 1;
				p = ' ';
			} else {
				p = *c;
			}
			a = b;
			b = *c;
		}
		if valid && dc == 2 {
			break;
		}
	}
	println!("{}{}{}{}{}{}{}{}", chars[0], chars[1], chars[2], chars[3],
		chars[4], chars[5], chars[6], chars[7]);
}
