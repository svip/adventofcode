use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
	let stdin = io::stdin();
	let mut wires: HashMap<String, u16> = HashMap::new();
	let mut circuit_lines: Vec<(String, String, String, String)> = vec![];
	
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v = line.split(" -> ").collect::<Vec<&str>>();
		let action: Vec<&str> = v[0].split(' ').collect();
		let wire = v[1];
		if action.len() == 1 {
			match action[0].parse::<u16>() {
			Ok(v) => {
				wires.insert(wire.to_string(), v);
				continue;
			}
			Err(_) => { }
			}
		}
		circuit_lines.push((action[0].to_string(), 
			if action.len() >= 2 { action[1].to_string() } else { "".to_string() },
			if action.len() == 3 { action[2].to_string() } else { "".to_string() },
			wire.to_string()));
	}
	
	while circuit_lines.len() != 0 {
		let mut i: i32 = 0;
		for line in circuit_lines.clone() {
			let wire = line.3;
			if wires.clone().get(&wire) != None {
				circuit_lines.remove(i as usize);
				i -= 1;
				continue;
			}
			if line.1 == "" {				
				match wires.clone().get(&line.0) {
				Some(v) => {
					circuit_lines.remove(i as usize);
					i -= 1;
					wires.insert(wire, *v);
				} 
				None => { }
				}
			} else if line.2 == "" {
				match wires.clone().get(&line.1) {
				Some(v) => {
					circuit_lines.remove(i as usize);
					i -= 1;
					wires.insert(wire, !*v);
				}
				None => { }
				}
			} else {
				let op = line.1.as_str();
				let mut c = false;
				let x = match wires.clone().get(&line.0) {
					Some(x) => { *x }
					None => { match line.0.parse::<u16>() {
						Ok(x) => { x }
						Err(_) => { c = true; 0 }
					} }
				};
				if !c {
					if op == "AND" || op == "OR" {
						match wires.clone().get(&line.2) {
						Some(y) => { 
							let v = match op {
								"AND" => { x & *y }
								"OR" => { x | *y }
								_ => { panic!("Wait what?"); }
							};
							circuit_lines.remove(i as usize);
							i -= 1;
							wires.insert(wire, v);
						} 
						None => { }
						}
					} else {
						let y = line.2.parse::<u32>().unwrap();
						
						let v = match op {
						"RSHIFT" => { x >> y }
						"LSHIFT" => { x << y }
						_ => { panic!("Wait what?"); }
						};
						circuit_lines.remove(i as usize);
						i -= 1;
						wires.insert(wire, v);
					}
				}
			}
			i += 1;
		}
	}
	
	let c = wires.get("a").unwrap();
	println!("{}", c);
}
