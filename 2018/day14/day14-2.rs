use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let after_amount = stdin.lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
	let digits: Vec<u8> = after_amount.to_string().chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect();
	let mut elves: [usize; 2] = [0, 1];
	let mut recipes: Vec<u8> = vec![0; 100_000_000];
	recipes[0] = 3; recipes[1] = 7;
	let digitslen = digits.len();
	
	let mut end = 1;
	loop {
		let new_score = recipes[elves[0]] + recipes[elves[1]];
		if new_score >= 10 {
			let first = new_score/10;
			let second = new_score%10;
			recipes[end+1] = first;
			recipes[end+2] = second;
			end += 2;
		} else {
			recipes[end+1] = new_score;
			end += 1;
		}
		let length = end + 1;
		elves[0] = elves[0] + recipes[elves[0]] as usize + 1;
		while elves[0] >= length {
			elves[0] -= length;
		}
		elves[1] = elves[1] + recipes[elves[1]] as usize + 1;
		while elves[1] >= length {
			elves[1] -= length;
		}
		if length > digitslen {
			let (start, end) = (length-digitslen, length);
			if recipes[start] != digits[0] && recipes[start-1] != digits[0] {
				continue;
			}
			if digitslen > 1 && recipes[start+1] != digits[1] && recipes[start] != digits[1] {
				continue;
			}
			if digitslen > 2 && recipes[start+2] != digits[2] && recipes[start+1] != digits[2] {
				continue;
			}
			if digitslen > 3 && recipes[start+3] != digits[3] && recipes[start+2] != digits[3] {
				continue;
			}
			if digitslen > 4 && recipes[start+4] != digits[4] && recipes[start+3] != digits[4] {
				continue;
			}
			let slice = &mut recipes.clone()[start..end];
			let test: Vec<u8> = slice.iter().map(|d| *d).collect();
			if test == digits {
				println!("{}", length - digitslen);
				break;
			}
			if new_score >= 10 {
				let (start, end) = (length-digitslen-1, length-1);
				let slice = &mut recipes.clone()[start..end];
				let test: Vec<u8> = slice.iter().map(|d| *d).collect();
				if test == digits {
					println!("{}", length - digitslen -1);
					break;
				}
			}
		}
	}
}
