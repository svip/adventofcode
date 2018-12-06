use std::io;
use std::io::prelude::*;

fn parse_chunk(s: String, is: String) -> i32 {
	if is.contains("red") {
		return 0;
	}
	let mut subscope = "".to_string();
	let mut immsubscope = "".to_string();
	let mut chunk = "".to_string();
	let mut sum = 0;
	let (mut depth, mut totaldepth) = (0, 0);
	for c in s.chars() {
		if c == '{' {
			depth += 1;
			totaldepth += 1;
		}
		if c == '}' {
			depth -= 1;
			totaldepth -= 1;
		}
		if c == '[' && depth > 0 {
			totaldepth += 1;
		}
		if c == ']' && depth > 0 {
			totaldepth -= 1;
		}
		if c == '{' && depth == 1 {
			continue;
		} else if c == '}' && depth == 0 {
			sum += parse_chunk(subscope.clone(), immsubscope.clone());
			subscope.clear();
			immsubscope.clear();
		} else if depth >= 1 {
			subscope.push(c);
			if totaldepth == 1 {
				immsubscope.push(c);
			}
		} else {
			chunk.push(c);
		}
	}
	let v: Vec<&str> = chunk.split(|c: char| !c.is_numeric() && c != '-').collect();
	for s in v {
		match s.parse::<i32>() {
		Ok(i) => { sum += i; }
		Err(_) => {}
		}
	}
	sum
}

fn main() {
	let stdin = io::stdin();
	let mut sum = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		sum += parse_chunk(line, "".to_string());
	}
	
	println!("{}", sum);
}
