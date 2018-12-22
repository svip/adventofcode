use std::io;
use std::io::prelude::*;

fn main() {
	let stdin = io::stdin();
	let line = stdin.lock().lines().next().unwrap().unwrap();
	let v: Vec<&str> = line.split(' ').collect();
	let (players, lastmarble) = (v[0].parse::<i32>().unwrap(), v[6].parse::<i64>().unwrap());
	
	let mut curplayer = 1;
	const MAXARENA: usize = 100_000;
	let mut arena: Box<[(usize, usize); MAXARENA]> = Box::new([(0,0); MAXARENA]);
	let mut scores: Vec<i64> = vec![];
	for _ in 1..players+1 {
		scores.push(0);
	}
	let mut nextid = 0;
	for marble in 1..(lastmarble+1) as usize {
		if marble % 23 == 0 {
			let mut t = 7;
			let mut id = nextid;
			while t > 0 {
				id = arena[id].0;
				t -= 1;
			}
			let m = id as i64;
			let (prev, next) = (arena[id].0, arena[id].1);
			arena[prev].1 = next;
			arena[next].0 = prev;
			let playerid = (curplayer - 1) as usize;
			scores[playerid] = scores[playerid] + m + marble as i64;
			nextid = next;
		} else {
			let (prev, next) = (arena[nextid].1, arena[arena[nextid].1].1);
			arena[marble] = (prev, next);
			arena[prev].1 = marble;
			arena[next].0 = marble;
			nextid = marble;
		}
		curplayer += 1;
		if curplayer > players {
			curplayer = 1;
		}
	}
	println!("{}", scores.iter().fold(0, |a, x| { if *x > a { *x } else { a } }));
}
