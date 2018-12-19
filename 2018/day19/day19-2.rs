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
	let first_line = lines.remove(0);
	let ipr = first_line.values[0] as usize;
	let eop = lines.len();
	let mut counts: Vec<u16> = vec![0; lines.len()];
	let mut test_run = true;
	let mut largest_count = 0;
	let mut largest_value = 0;
	let mut largest_value_at = 0;
	let mut work_at = 0;
	let mut registers: [u64; 6];
	
	// This loop runs twice, first in test mode and then in actual mode.
	// The test mode is used to determine where the critical loop takes place.
	// The loop is just at attempt at N % D == 0 checking, but since the 
	// instructions are so simple, it's not possible for it to do it quickly.
	// So using a histogram, we can basically determine where the loop is by
	// seeing after a number of runs (in this case 100'000), which instructions
	// receive the most attention.
	// This allows us to find the following:
	// - The start pointer of the loop.
	// - The end pointer of it (so we can escape our code out of it).
	// - The value it's trying to match again.
	// - Where it is stored at.
	// - And the work register.
	
	// Then the program runs for real.  When the program encounters the state
	// where it is at where the loop is supposed to begin, and the largest value
	// we are looking for is stored at the register we know it should be at,
	// we intervene and insert our own code.
	// Basically doing the same thing as the loop, but much faster.
	// Then we loop out of it again, by setting the ip to the end pointer + 1.
	
	'main: loop {
		registers = [0; 6];
		registers[0] = 1;
		let mut total_count: u64 = 0;
		let mut start_pointer: usize = 0;
		let mut end_pointer: usize = 0;
		if !test_run {
			let pointers = counts.iter().enumerate()
				.filter(|t| *t.1 >= largest_count)
				.map(|t| t.0);
			start_pointer = pointers.clone().fold(lines.len(), |a, x| if x < a { x } else { a });
			end_pointer = pointers.fold(0, |a, x| if x > a { x } else { a });
		}
		loop {
			let p = registers[ipr] as usize;
			if p >= eop {
				break;
			}
			let line = lines[p];
			let values = line.values;
			let (ai, bi, ci) = (values[0] as u64, values[1] as u64, values[2] as u64);
			let (a, b, c) = (ai as usize, bi as usize, ci as usize);
			if !test_run {
				if p == start_pointer && registers[largest_value_at] == largest_value {
					while registers[work_at] <= largest_value {
						if largest_value % registers[work_at] == 0 {
							registers[0] = registers[0] + registers[work_at];
						}
						registers[work_at] += 1;
					}
					registers[ipr] = end_pointer as u64 + 1;
				}
			}
			
			counts[p] += 1;
			
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
			registers[ipr] += 1;
			total_count += 1;
			if test_run && total_count >= 100_000 {
				largest_count = counts.iter().fold(0, |a, x| if *x > a { *x } else { a }) - 10;
				largest_value = registers.iter().fold(0, |a, x| if *x > a { *x } else { a });
				largest_value_at = registers.iter().enumerate()
					.fold(0, |a, x| if *x.1 == largest_value { x.0 } else { a });
				// At 100'000 times, it's determined that the register used for
				// the multiple against value is 1, and no other register is.
				work_at = registers.iter().enumerate()
					.fold(0, |a, x| if *x.1 == 1 { x.0 } else { a });
				test_run = false;
				continue 'main;
			}
		}
		if !test_run {
			break;
		}
	}
	println!("{}", registers[0]);
}
