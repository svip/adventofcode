use std::io;
use std::io::prelude::*;
use std::collections::{HashMap, HashSet};
use std::cmp::max;

#[derive(Copy)]
enum TrackPiece {
	Horizontal, // left - right
	Vertical, // top - bottom
	Intersection, // top, right, bottom, left
	DiagonalRight, // / left top OR right bottom
	DiagonalLeft, // \ right top OR left bottom
	Nothing,
}

impl Clone for TrackPiece {
	fn clone(&self) -> TrackPiece { *self }
}

#[derive(Copy)]
enum Direction {
	Up,
	Right,
	Down,
	Left,
}

impl Clone for Direction {
	fn clone(&self) -> Direction { *self }
}

#[derive(Copy)]
enum Move {
	Left,
	Straight,
	Right,
}

impl Clone for Move {
	fn clone(&self) -> Move { *self }
}

#[derive(Copy)]
struct Cart {
	id: usize,
	location: (u32, u32),
	heading: Direction,
	next_move: Move,
}

impl Clone for Cart {
	fn clone(&self) -> Cart { *self }
}

fn main() {
	let stdin = io::stdin();
	let mut map: HashMap<(u32, u32), TrackPiece> = HashMap::new();
	let mut carts: Vec<Cart> = vec![];
	let mut y = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let mut x = 0;
		for c in line.chars() {
			let piece = match c {
			'-' => { TrackPiece::Horizontal }
			'|' => { TrackPiece::Vertical }
			'+' => { TrackPiece::Intersection }
			'/' => { TrackPiece::DiagonalRight }
			'\\' => { TrackPiece::DiagonalLeft }
			'>' => {
				let id = carts.clone().len();
				carts.push(Cart{id: id, location:(x, y), heading: Direction::Right, next_move: Move::Left});
				TrackPiece::Horizontal
			}
			'<' => {
				let id = carts.clone().len();
				carts.push(Cart{id: id, location:(x, y), heading: Direction::Left, next_move: Move::Left});
				TrackPiece::Horizontal
			}
			'^' => {
				let id = carts.clone().len();
				carts.push(Cart{id: id, location:(x, y), heading: Direction::Up, next_move: Move::Left});
				TrackPiece::Vertical
			}
			'v' => {
				let id = carts.clone().len();
				carts.push(Cart{id: id, location:(x, y), heading: Direction::Down, next_move: Move::Left});
				TrackPiece::Vertical
			}
			_ => { TrackPiece::Nothing }
			};
			map.insert((x, y), piece);
			x += 1;
		}
		y += 1;
	}
	let mut tick = 0;
	'main: loop {
		let mut work_carts = carts.clone();
		
		while work_carts.len() > 0 {
			let mut our_cart: Cart = work_carts[0];
			let mut id = 0;
			let mut i = 0;
			for cart in work_carts.clone() {
				if cart.location.1 < our_cart.location.1 ||
					(cart.location.1 == our_cart.location.1 &&
					cart.location.0 < our_cart.location.0) {
					our_cart = cart;
					id = i;
				}
				i += 1;
			}
			let from = our_cart.location;
			let (x, y) = match our_cart.heading {
				Direction::Up =>    { (from.0, from.1-1) }
				Direction::Right => { (from.0+1, from.1) }
				Direction::Down =>  { (from.0, from.1+1) }
				Direction::Left =>  { (from.0-1, from.1) }
			};
			let cid = our_cart.id;
			carts[cid].location = (x, y);
			match map.get(&(x, y)) {
			Some(p) => {
				match p {
				TrackPiece::Horizontal => { }
				TrackPiece::Vertical => { }
				TrackPiece::Intersection => {
					match our_cart.next_move {
					Move::Left => { 
						carts[cid].heading = match our_cart.heading {
							Direction::Up =>    { Direction::Left }
							Direction::Right => { Direction::Up }
							Direction::Down =>  { Direction::Right }
							Direction::Left =>  { Direction::Down }
						};
						carts[cid].next_move = Move::Straight;
					}
					Move::Straight => { 
						carts[cid].next_move = Move::Right;
					}
					Move::Right => { 
						carts[cid].heading = match our_cart.heading {
							Direction::Up =>    { Direction::Right }
							Direction::Right => { Direction::Down }
							Direction::Down =>  { Direction::Left }
							Direction::Left =>  { Direction::Up }
						};
						carts[cid].next_move = Move::Left;
					}
					}
				}
				TrackPiece::DiagonalRight => {
					let (froms, xs, ys) = ((from.0 as i32, from.1 as i32), x as i32, y as i32);
					let next: (u32, u32) = 
						if froms == (xs-1, ys) { (x, y-1) }
						else if froms == (xs, ys-1) { (x-1, y) }
						else if froms == (xs+1, ys) { (x, y+1) }
						else if froms == (xs, ys+1) { (x+1, y) }
						else { panic!("ZOMG!"); };
					if next.0 < x { carts[cid].heading = Direction::Left; }
					if next.0 > x { carts[cid].heading = Direction::Right; }
					if next.1 < y { carts[cid].heading = Direction::Up; }
					if next.1 > y { carts[cid].heading = Direction::Down; }
				}
				TrackPiece::DiagonalLeft => {
					let (froms, xs, ys) = ((from.0 as i32, from.1 as i32), x as i32, y as i32);
					let next: (u32, u32) = 
						if froms == (xs-1, ys) { (x, y+1) }
						else if froms == (xs, ys-1) { (x+1, y) }
						else if froms == (xs+1, ys) { (x, y-1) }
						else if froms == (xs, ys+1) { (x-1, y) }
						else { panic!("ZOMG!"); };
					if next.0 < x { carts[cid].heading = Direction::Left; }
					if next.0 > x { carts[cid].heading = Direction::Right; }
					if next.1 < y { carts[cid].heading = Direction::Up; }
					if next.1 > y { carts[cid].heading = Direction::Down; }
				}
				TrackPiece::Nothing => {
					panic!("Attempt to {} move to {}, {} from {}, {}", cid, x, y, from.0, from.1);
				}
				}
			}
			None => {
				panic!("Attempt to {} move to {}, {} from {}, {}", cid, x, y, from.0, from.1);
			}
			}
			println!("Tick {}, moved {} from ({:2}, {:2}) to ({:2}, {:2}) (new direction: {} / move: {}):", tick, cid, from.0, from.1, x, y, match carts[cid].heading { 
				Direction::Up => { '^' }
				Direction::Left => { '<' }
				Direction::Down => { 'v' }
				Direction::Right => { '>' }
			}, match carts[cid].next_move {
				Move::Left => { '<' }
				Move::Straight => { '^' }
				Move::Right => { '>' }
			});
			for y in max(0, y as i32 -2) as u32..y+3 {
				for x in max(0, x as i32 -2) as u32..x+3 {
					let (mut drawn_cart, mut two_carts) = (false, false);
					let mut tmp_cart = carts[0];
					for cart in carts.clone() {
						if cart.location == (x, y) {
							if drawn_cart {
								two_carts = true;
							}
							tmp_cart = cart;
							drawn_cart = true;
						}
					}
					if two_carts {
						print!("X");
					} else if drawn_cart {
						print!("{}", match tmp_cart.heading {
						Direction::Up => { '^' }
						Direction::Right => { '>' }
						Direction::Down => { 'v' }
						Direction::Left => { '<' }
						});
					} else {
						print!("{}", match map.get(&(x, y)) {
							Some(p) => {
								match p {
								TrackPiece::Horizontal => { '-' }
								TrackPiece::Vertical => { '|' }
								TrackPiece::Intersection => { '+' }
								TrackPiece::DiagonalRight => { '/' }
								TrackPiece::DiagonalLeft => { '\\' }
								TrackPiece::Nothing => { ' ' }
								}
							}
							None => { '#' }
						});
					}
				}
				println!("");
			}
			work_carts.remove(id);
			let mut found: HashSet<(u32, u32)> = HashSet::new();
			for cart in carts.clone() {
				if found.contains(&cart.location) {
					println!("{},{}", cart.location.0, cart.location.1);
					break 'main;
				}
				found.insert(cart.location);
			}
		}
		tick += 1;
	}
}
