use std::io;
use std::io::prelude::*;

enum Op {
	Addr, //(add register) stores into register C the result of adding register A and register B.
    Addi, //(add immediate) stores into register C the result of adding register A and value B.

    Mulr, // (multiply register) stores into register C the result of multiplying register A and register B.
    Muli, // (multiply immediate) stores into register C the result of multiplying register A and value B.

    Banr, // (bitwise AND register) stores into register C the result of the bitwise AND of register A and register B.
    Bani, // (bitwise AND immediate) stores into register C the result of the bitwise AND of register A and value B.

    Borr, // (bitwise OR register) stores into register C the result of the bitwise OR of register A and register B.
    Bori, // (bitwise OR immediate) stores into register C the result of the bitwise OR of register A and value B.

    Setr, // (set register) copies the contents of register A into register C. (Input B is ignored.)
    Seti, // (set immediate) stores value A into register C. (Input B is ignored.)

    Gtir, // (greater-than immediate/register) sets register C to 1 if value A is greater than register B. Otherwise, register C is set to 0.
    Gtri, // (greater-than register/immediate) sets register C to 1 if register A is greater than value B. Otherwise, register C is set to 0.
    Gtrr, // (greater-than register/register) sets register C to 1 if register A is greater than register B. Otherwise, register C is set to 0.

    Eqir, // (equal immediate/register) sets register C to 1 if value A is equal to register B. Otherwise, register C is set to 0.
    Eqri, // (equal register/immediate) sets register C to 1 if register A is equal to value B. Otherwise, register C is set to 0.
    Eqrr, // (equal register/register) sets register C to 1 if register A is equal to register B. Otherwise, register C is set to 0.
}

struct Sample {
	before: [u8; 4],
	after: [u8; 4],
	instructions: [u8; 4],
	behaves_like: Vec<Op>,
}

fn main() {
	let stdin = io::stdin();
	let mut samples: Vec<Sample> = vec![];
	let mut current_sample = Sample{before:[0;4], after:[0;4], instructions:[0;4], behaves_like:vec![]};
	let mut blank_lines = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		if line == "" {
			blank_lines += 1;
			if blank_lines > 1 {
				break;
			}
			continue;
		} else {
			blank_lines = 0;
		}
		let v: Vec<&str> = line.split(' ').collect();
		let mut nos: [u8; 4];
		if v[0] == "Before:" || v[0] == "After:" {
			let chars: Vec<char> = line.chars().collect();
			nos = [chars[9].to_string().parse::<u8>().unwrap(),
				chars[12].to_string().parse::<u8>().unwrap(), 
				chars[15].to_string().parse::<u8>().unwrap(),
				chars[18].to_string().parse::<u8>().unwrap()];
		} else {
			nos = [v[0].parse::<u8>().unwrap(), v[1].parse::<u8>().unwrap(),
				v[2].parse::<u8>().unwrap(), v[3].parse::<u8>().unwrap()];
		}
		match v[0] {
		"Before:" => {
			current_sample = Sample{before:[0;4], after:[0;4], instructions:[0;4], behaves_like:vec![]};
			current_sample.before = nos;
		}
		"After:" => {
			current_sample.after = nos;
			samples.push(Sample{before:current_sample.before, after:current_sample.after,
				instructions:current_sample.instructions, behaves_like:vec![]});
		}
		_ => {
			current_sample.instructions = nos;
		}		
		}
	}
	
	let mut final_samples: Vec<Sample> = vec![];
	for sample in samples {
		let mut behaves_like: Vec<Op> = vec![];
		let (ai, bi, ci) = (sample.instructions[1], sample.instructions[2], sample.instructions[3]);
		let (a, b, c) = (ai as usize, bi as usize, ci as usize);
		if sample.after[c] == sample.before[a] + sample.before[b] {
			behaves_like.push(Op::Addr);
		}
		if sample.after[c] == sample.before[a] + bi {
			behaves_like.push(Op::Addi);
		}
		if sample.after[c] == sample.before[a] * sample.before[b] {
			behaves_like.push(Op::Mulr);
		}
		if sample.after[c] == sample.before[a] * bi {
			behaves_like.push(Op::Muli);
		}
		if sample.after[c] == sample.before[a] & sample.before[b] {
			behaves_like.push(Op::Banr);
		}
		if sample.after[c] == sample.before[a] & bi {
			behaves_like.push(Op::Bani);
		}
		if sample.after[c] == sample.before[a] | sample.before[b] {
			behaves_like.push(Op::Borr);
		}
		if sample.after[c] == sample.before[a] | bi {
			behaves_like.push(Op::Bori);
		}
		if sample.after[c] == sample.before[a] {
			behaves_like.push(Op::Setr);
		}
		if sample.after[c] == ai {
			behaves_like.push(Op::Seti);
		}
		if sample.after[c] == if ai > sample.before[b] { 1 } else { 0 } {
			behaves_like.push(Op::Gtir);
		}
		if sample.after[c] == if sample.before[a] > bi { 1 } else { 0 } {
			behaves_like.push(Op::Gtri);
		}
		if sample.after[c] == if sample.before[a] > sample.before[b] { 1 } else { 0 } {
			behaves_like.push(Op::Gtrr);
		}
		if sample.after[c] == if ai == sample.before[b] { 1 } else { 0 } {
			behaves_like.push(Op::Eqir);
		}
		if sample.after[c] == if sample.before[a] == bi { 1 } else { 0 } {
			behaves_like.push(Op::Eqri);
		}
		if sample.after[c] == if sample.before[a] == sample.before[b] { 1 } else { 0 } {
			behaves_like.push(Op::Eqrr);
		}
		final_samples.push(Sample{before:sample.before, after:sample.after,
			instructions:sample.instructions, behaves_like:behaves_like});
	}
	
	println!("{} / {}", final_samples.iter().filter(|s| s.behaves_like.len() >= 3).count(), final_samples.len());
}

