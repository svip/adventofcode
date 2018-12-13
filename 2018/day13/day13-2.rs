use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

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
	inactive: bool,
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
				carts.push(Cart{id: id, location:(x, y), heading: Direction::Right, next_move: Move::Left, inactive: false});
				TrackPiece::Horizontal
			}
			'<' => {
				let id = carts.clone().len();
				carts.push(Cart{id: id, location:(x, y), heading: Direction::Left, next_move: Move::Left, inactive: false});
				TrackPiece::Horizontal
			}
			'^' => {
				let id = carts.clone().len();
				carts.push(Cart{id: id, location:(x, y), heading: Direction::Up, next_move: Move::Left, inactive: false});
				TrackPiece::Vertical
			}
			'v' => {
				let id = carts.clone().len();
				carts.push(Cart{id: id, location:(x, y), heading: Direction::Down, next_move: Move::Left, inactive: false});
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
		work_carts = work_carts.iter().filter(|c| !c.inactive).map(|c| *c).collect();
		
		'worker: while work_carts.len() > 0 {
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
			work_carts.remove(id);
			let mut found: HashMap<(u32, u32), usize> = HashMap::new();
			for cart in carts.clone() {
				if cart.inactive { continue; }
				match found.clone().get(&cart.location) {
				Some(c) => {
					println!("{} and {} crashed into one another at {}, {} (at {})", *c, cart.id, cart.location.0, cart.location.1, tick);
					carts[*c].inactive = true;
					carts[cart.id].inactive = true;
					let mut i = 0;
					for wcart in work_carts.clone() {
						if wcart.id == *c { work_carts.remove(i); }
						if wcart.id == cart.id { work_carts.remove(i); }
						i += 1;
					}
					continue 'worker;
				}
				None => {
					found.insert(cart.location, cart.id);
				}
				}
			}
		}
		if carts.iter().filter(|c| !c.inactive).count() == 1 {
			let c = carts.iter().filter(|c| !c.inactive).map(|c| *c).collect::<Vec<Cart>>()[0];
			println!("{},{} at {}", c.location.0, c.location.1, tick);
			break;
		}
		tick += 1;
	}
}
