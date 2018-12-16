use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy)]
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

impl Clone for Op {
	fn clone(&self) -> Op { *self }
}

impl PartialEq for Op {
	fn eq(&self, other: &Op) -> bool {
		match (&self, other) {
			(Op::Addr, Op::Addr) => true,
			(Op::Addi, Op::Addi) => true,
			(Op::Mulr, Op::Mulr) => true,
			(Op::Muli, Op::Muli) => true,
			(Op::Banr, Op::Banr) => true,
			(Op::Bani, Op::Bani) => true,
			(Op::Borr, Op::Borr) => true,
			(Op::Bori, Op::Bori) => true,
			(Op::Setr, Op::Setr) => true,
			(Op::Seti, Op::Seti) => true,
			(Op::Gtir, Op::Gtir) => true,
			(Op::Gtri, Op::Gtri) => true,
			(Op::Gtrr, Op::Gtrr) => true,
			(Op::Eqir, Op::Eqir) => true,
			(Op::Eqri, Op::Eqri) => true,
			(Op::Eqrr, Op::Eqrr) => true,
			(_, _) => false,
		}
	}
}

impl fmt::Debug for Op {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Op({})", match self {
			Op::Addr => "Addr",
			Op::Addi => "Addi",
			Op::Mulr => "Mulr",
			Op::Muli => "Muli",
			Op::Banr => "Banr",
			Op::Bani => "Bani",
			Op::Borr => "Borr",
			Op::Bori => "Bori",
			Op::Setr => "Setr",
			Op::Seti => "Seti",
			Op::Gtir => "Gtir",
			Op::Gtri => "Gtri",
			Op::Gtrr => "Gtrr",
			Op::Eqir => "Eqir",
			Op::Eqri => "Eqri",
			Op::Eqrr => "Eqrr",
		})
	}
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
	let mut test_program: Vec<[u8;4]> = vec![];
	let mut blank_lines = 0;
	let mut test_program_mode = false;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		if line == "" {
			blank_lines += 1;
			if blank_lines > 1 {
				test_program_mode = true;
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
		if !test_program_mode {
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
		} else {
			test_program.push(nos);
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
	
	let mut possible_opcodes: Vec<(Op, Vec<u8>)> = vec![(Op::Addr, vec![]),
		(Op::Addi, vec![]), (Op::Mulr, vec![]), (Op::Muli, vec![]),
		(Op::Banr, vec![]), (Op::Bani, vec![]), (Op::Borr, vec![]),
		(Op::Bori, vec![]), (Op::Setr, vec![]), (Op::Seti, vec![]),
		(Op::Gtir, vec![]), (Op::Gtri, vec![]), (Op::Gtrr, vec![]),
		(Op::Eqir, vec![]), (Op::Eqri, vec![]), (Op::Eqrr, vec![])];
	
	for sample in final_samples {
		let opcode = sample.instructions[0];
		for op in sample.behaves_like {
			let mut i = 0;
			for o in possible_opcodes.clone() {
				if o.0 == op {
					let mut v = o.1;
					if !v.contains(&opcode) {
						v.push(opcode);
						possible_opcodes[i] = (o.0, v);
					}
					break;
				}
				i += 1;
			}
		}
	}
	let mut map: HashMap<u8, Op> = HashMap::new();
	
	while possible_opcodes.len() > 0 {
		for p in possible_opcodes.clone() {
			if p.1.len() == 1 {
				let opcode = p.1[0];
				possible_opcodes = possible_opcodes.iter().map(|b|
					(b.0, b.1.iter().filter(|o| **o != opcode)
						.map(|o| *o).collect::<Vec<u8>>())).collect();
				possible_opcodes = possible_opcodes.iter()
					.filter(|b| b.1.len() > 0)
					.map(|b| (b.0, b.1.clone())).collect();
				map.insert(opcode, p.0);
				break;
			}
		}
	}
	
	let mut registers: [u16; 4] = [0; 4];
	for line in test_program {
		let (ai, bi, ci) = (line[1] as u16, line[2] as u16, line[3] as u16);
		let (a, b, c) = (ai as usize, bi as usize, ci as usize);
		match map.get(&line[0]) {
		Some(op) => {
			registers[c] = match op {
			Op::Addr => { registers[a] + registers[b] }
			Op::Addi => { registers[a] + bi }
			Op::Mulr => { registers[a] * registers[b] }
			Op::Muli => { registers[a] * bi }
			Op::Banr => { registers[a] & registers[b] }
			Op::Bani => { registers[a] & bi }
			Op::Borr => { registers[a] | registers[b] }
			Op::Bori => { registers[a] | bi }
			Op::Setr => { registers[a] }
			Op::Seti => { ai }
			Op::Gtir => { if ai > registers[b] { 1 } else { 0 } }
			Op::Gtri => { if registers[a] > bi { 1 } else { 0 } }
			Op::Gtrr => { if registers[a] > registers[b] { 1 } else { 0 } }
			Op::Eqir => { if ai == registers[b] { 1 } else { 0 } }
			Op::Eqri => { if registers[a] == bi { 1 } else { 0 } }
			Op::Eqrr => { if registers[a] == registers[b] { 1 } else { 0 } }
			}
		}
		None => { panic!("Don't know {}!", line[0]); }
		}
	}
	println!("{}", registers[0]);
}

