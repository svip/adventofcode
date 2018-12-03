use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn move_pos(c: char, mut pos: (i32, i32)) -> (i32, i32) {
	pos = match c {
		'<' => { (pos.0-1, pos.1) }
		'>' => { (pos.0+1, pos.1) }
		'^' => { (pos.0, pos.1-1) }
		'v' => { (pos.0, pos.1+1) }
		_ => { println!("{:?}", pos); pos }
	};
	pos
}

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut map: HashMap<(i32, i32), i32> = HashMap::new();
	let (mut s_pos, mut r_pos) = ((0, 0), (0, 0));
	map.insert(s_pos, 2);
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		for (i, c) in line.char_indices() {
			let pos = if i % 2 == 0 {
				s_pos = move_pos(c, s_pos);
				s_pos.clone()
			} else {
				r_pos = move_pos(c, r_pos);
				r_pos.clone()
			};
			*map.entry(pos).or_insert(0) += 1;
		}
	}
	println!("{}", map.values().filter(|x| *x > &0).count());
	
	Ok(())
}
