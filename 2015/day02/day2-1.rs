use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
	let stdin = io::stdin();
	let mut totalarea = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<i32> = line.split('x').map(|c| c.parse::<i32>().unwrap()).collect();
		let (l, w, h) = (v[0], v[1], v[2]);
		let mut a = vec![l*w, w*h, h*l];
		a.sort_by(|x, y| x.cmp(y));
		totalarea += a[0] + a[0] * 2 + a[1] * 2 + a[2] * 2;
	}
	println!("{}", totalarea);
	
	Ok(())
}
