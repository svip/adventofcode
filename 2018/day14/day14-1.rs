use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let after_amount = stdin.lock().lines().next().unwrap().unwrap().parse::<usize>().unwrap();
	let mut elves: [usize; 2] = [0, 1];
	let mut recipes: Vec<u8> = vec![3, 7];
	
	while recipes.len() < after_amount + 10 {
		let new_score = recipes[elves[0]] + recipes[elves[1]];
		if new_score >= 10 {
			let first = new_score/10;
			let second = new_score%10;
			recipes.push(first);
			recipes.push(second);
		} else {
			recipes.push(new_score);
		}
		elves[0] = elves[0] + recipes[elves[0]] as usize + 1;
		while elves[0] >= recipes.len() {
			elves[0] -= recipes.len();
		}
		elves[1] = elves[1] + recipes[elves[1]] as usize + 1;
		while elves[1] >= recipes.len() {
			elves[1] -= recipes.len();
		}
	}
	
	println!("{}{}{}{}{}{}{}{}{}{}", recipes[after_amount], recipes[after_amount+1],
		recipes[after_amount+2],recipes[after_amount+3],recipes[after_amount+4],
		recipes[after_amount+5],recipes[after_amount+6],recipes[after_amount+7],
		recipes[after_amount+8],recipes[after_amount+9]);
}
