use std::io;
use std::io::prelude::*;

#[derive(Copy,PartialEq,Debug)]
enum Op {
	Ip, // Set instruction pointer
	
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

#[derive(Copy,Debug)]
struct Line {
	op: Op,
	values: [u64; 3],
}

impl Clone for Line {
	fn clone(&self) -> Line { *self }
}

fn main() {
	let stdin = io::stdin();
	let mut lines: Vec<Line> = vec![];
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(' ').collect();
		if v[0] == "#ip" {
			lines.push(Line{op:Op::Ip, values: [v[1].parse::<u64>().unwrap(), 0, 0]});
		} else {
			let op = match v[0] {
				"addr" => Op::Addr,
				"addi" => Op::Addi,
				"mulr" => Op::Mulr,
				"muli" => Op::Muli,
				"banr" => Op::Banr,
				"bani" => Op::Bani,
				"borr" => Op::Borr, 
				"bori" => Op::Bori,
				"setr" => Op::Setr, 
				"seti" => Op::Seti,
				"gtir" => Op::Gtir, 
				"gtri" => Op::Gtri, 
				"gtrr" => Op::Gtrr,
				"eqir" => Op::Eqir,
				"eqri" => Op::Eqri,
				"eqrr" => Op::Eqrr,
				_ => { continue; }
			};
			let values = [v[1].parse::<u64>().unwrap(), v[2].parse::<u64>().unwrap(),
				v[3].parse::<u64>().unwrap()];
			lines.push(Line{op: op, values: values});
		}
	}
	let first_line = lines.remove(0);
	let ipr = first_line.values[0] as usize;
	
	// We know we are interested in the line that tests whether the programs
	// stops or do not.  This line is something to the effect of eqrr a b c,
	// where b = 0 and the c is added to our ipr the following line:
	// addr a b c
	// An example could look like this (where ip = 5):
	// eqrr 3 0 2
	// addr 2 5 5
	// Once we've found this pattern in the program, we are interested in
	// the a register for eqrr (in this example, 3).
	// This register keeps looping to a value that is tested against 0 (our
	// value).  Our value doesn't influence the loop, so we need a value, it
	// actually gets to.  So basically, to find the fewest amount of
	// instructions to do, we must run the program until we have a value at
	// register a (in the example 3), when the program is at our ipr of the eqrr
	// operation.
	
	// But first we must find the eqrr addr pattern.
	
	let mut eqipr = 0;
	let mut work_register_at = 0;
	
	for l in lines.iter().enumerate() {
		let line = l.1;
		let ip = l.0;
		if line.op == Op::Eqrr && lines[ip+1].op == Op::Addr {
			work_register_at = line.values[0] as usize;
			eqipr = ip as u64;
			break;
		}
	}
	
	// Good, now let's just run the program as normal until we get to
	// ipr == eqipr
	
	let mut registers: [u64; 6] = [0; 6];		
	loop {
		let p = registers[ipr] as usize;
		if p >= lines.len() {
			break;
		}
		let line = lines[p];
		let values = line.values;
		let (ai, bi, ci) = (values[0], values[1], values[2]);
		let (a, b, c) = (ai as usize, bi as usize, ci as usize);
		
		registers[c] = match line.op {
			Op::Ip => { registers[c] }
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
		};
		if registers[ipr] == eqipr {
			// Great, we found the value.
			println!("{}", registers[work_register_at]);
			break;
		}
		registers[ipr] += 1;
	}
}
