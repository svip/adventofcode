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
	values: [u8; 3],
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
			lines.push(Line{op:Op::Ip, values: [v[1].parse::<u8>().unwrap(), 0, 0]});
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
			let values = [v[1].parse::<u8>().unwrap(), v[2].parse::<u8>().unwrap(),
				v[3].parse::<u8>().unwrap()];
			lines.push(Line{op: op, values: values});
		}
	}
	let mut registers: [u64; 6] = [0; 6];
	let first_line = lines.remove(0);
	let ipr = first_line.values[0] as usize;
	
	loop {
		let p = registers[ipr] as usize;
		if p >= lines.len() {
			break;
		}
		let line = lines[p];
		let values = line.values;
		let (ai, bi, ci) = (values[0] as u64, values[1] as u64, values[2] as u64);
		let (a, b, c) = (ai as usize, bi as usize, ci as usize);
		//let copy = registers.clone();
		
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
		//println!("ip={} {:?} {:?} {} {} {} {:?}", p, copy, line.op, ai, bi, ci, registers);
		registers[ipr] += 1;
	}
	println!("{}", registers[0]);
}
