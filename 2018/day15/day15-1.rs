use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Copy)]
struct Unit {
	is_goblin: bool,
	position: (i32, i32),
	hp: i32,
}

impl Clone for Unit {
	fn clone(&self) -> Unit { *self }
}

fn draw_map(round: i32, map: HashMap<(i32, i32), bool>, width: usize, height: usize, units: HashMap<i32, Unit>) {
	println!("After {} rounds:", round);
	for y in 0..height as i32 {
		for x in 0..width as i32 {
			print!("{}", match map.get(&(x, y)) {
			Some(t) => {
				if *t {
					if units.values().any(|u| u.is_goblin && u.position == (x, y)) {
						'G'
					} else if units.values().any(|u| !u.is_goblin && u.position == (x, y)) {
						'E'
					} else {
						'.'
					}
				} else { '#' }
			}
			None => { ' ' }
			});
		}
		let mut us = units.values().filter(|u| u.position.1 == y).map(|u| *u).collect::<Vec<Unit>>();
		us.sort_by(|x, y| x.position.0.cmp(&y.position.0));
		for u in us {
			print!(" {}({:3})", if u.is_goblin { 'G' } else { 'E' }, u.hp);
		}
		println!("");
	}
}

fn to_index(pos: (i32, i32), width: usize) -> usize {
	(pos.0 + pos.1*width as i32) as usize
}

fn to_pos(index: usize, width: usize) -> (i32, i32) {
	let x = index%width;
	let y = index/width;
	(x as i32, y as i32)
}

fn get_neighbours(pos: (i32, i32)) -> [(i32, i32); 4] {
	[(pos.0, pos.1-1), (pos.0-1, pos.1), (pos.0+1, pos.1), (pos.0, pos.1+1)]
}

fn path_finder(map: HashMap<(i32, i32), bool>, width: usize, height: usize, units: HashMap<i32, Unit>, from: (i32, i32), to: (i32, i32)) -> i32 {
	if from == to {
		return 0;
	}
	let high = map.len() as i32;
	let mut distances: Vec<i32> = vec![high; width*height];
	let mut hdistances: Vec<i32> = vec![high; width*height];
	let mut calculated: Vec<usize> = vec![];
	let mut visited: Vec<bool> = vec![true; width*height];
	
	let mut unchecked_nodes = 0;
	
	// initialise our temporary maps.
	for (p, t) in map.clone() {
		if t && !units.values().any(|u| u.position == p && u.hp > 0) {
			let id = to_index(p, width);
			visited[id] = false;
			unchecked_nodes += 1;
		}
	}
	
	let mut tmp_pos = from;
	let mut tmp_id = to_index(tmp_pos, width);
	let to_id = to_index(to, width);
	
	distances[tmp_id] = 0;
	hdistances[tmp_id] = (to.0-tmp_pos.0).abs() + (to.1-tmp_pos.1).abs();
	calculated.push(tmp_id);
	let mut prev_id = 0;
	
	while unchecked_nodes > 0 {
		if distances[tmp_id] == high {
			return -1;
		}
		unchecked_nodes -= 1;
		let neighbours: [(i32, i32); 4] = get_neighbours(tmp_pos);
		
		let current_distance = distances[tmp_id];
		
		for neighbour in neighbours.iter() {
			let nid = to_index(*neighbour, width);
			if !visited[nid] {
				let new_distance = current_distance + 1;
				distances[nid] = new_distance;
				hdistances[nid] = new_distance + 
					(to.0-neighbour.0).abs() + (to.1-neighbour.1).abs();
				calculated.push(nid);
			}
		}
		visited[tmp_id] = true;
		let mut short_distance = high;
		
		for p in calculated.clone() {
			let check = hdistances[p];
			if check < short_distance && !visited[p] {
				short_distance = check;
				tmp_id = p;
				tmp_pos = to_pos(tmp_id, width);
			}
		}
		
		if tmp_id == prev_id || tmp_id == to_id {
			break;
		}
		prev_id = tmp_id;
	}
	if tmp_id != to_id {
		return -1;
	}
	
	return distances[tmp_id];
}

fn main() {
	let stdin = io::stdin();
	let mut map: HashMap<(i32, i32), bool> = HashMap::new();
	let mut units: HashMap<i32, Unit> = HashMap::new();
	let (mut width, mut height) = (0 as usize, 0 as usize);
	let mut y: usize = 0;
	let mut uid = 0;
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let mut x: usize = 0;
		for c in line.chars() {
			let tile = match c {
				'#' => { false }
				'.' => { true }
				'E' => { 
					units.insert(uid, Unit{is_goblin: false, position: (x as i32, y as i32), hp: 200});
					uid += 1;
					true
				}
				'G' => {
					units.insert(uid, Unit{is_goblin: true, position: (x as i32, y as i32), hp: 200});
					uid += 1;
					true
				}
				_ => { false }
			};
			map.insert((x as i32, y as i32), tile);
			x += 1;
			if x > width { width = x; }
		}
		y += 1;
		if y > height { height = y; }
	}
	let mut round = 0;
	'main: loop {
		draw_map(round, map.clone(), width, height, units.clone());
		let mut tmp_units = units.iter().map(|t| (*t.0, *t.1)).collect::<Vec<(i32, Unit)>>();
		tmp_units.sort_by(|x, y| to_index(x.1.position, width).cmp(&to_index(y.1.position, width)) );
		tmp_units.reverse();
		while tmp_units.len() > 0 {
			let tmp = tmp_units.pop().unwrap();
			let tmp_id = tmp.0;
			let mut tmp_unit = tmp.1;
			// Do we have any targets?
			let search_for = !tmp_unit.is_goblin;
			let mut targets: Vec<(i32, Unit)> = vec![];
			for (id, u) in units.clone() {
				if u.is_goblin == search_for && u.hp > 0 {
					targets.push((id, u));
				}
			}
			if targets.len() == 0 {
				// No possible targets; combat ends.
				break 'main;
			}
			// Check if neighbour
			let unit_neighbours = get_neighbours(tmp_unit.position);
			let mut any_neighbour = false;
			for test in unit_neighbours.iter() {
				for t in targets.clone() {
					if t.1.position == *test {
						any_neighbour = true;
						break;
					}
				}
			}
			if !any_neighbour {
				// Time to move then
				// to, distance, from
				let mut target_range: Vec<((i32, i32), i32, (i32, i32))> = vec![];
				let mut nearest = map.len() as i32;
				for t in targets.clone() {
					let pos = t.1.position;
					let testers = get_neighbours(pos);
					for test in testers.iter() {
						if *map.get(test).unwrap_or(&false) && 
							!units.values().any(|u| u.position == *test && u.hp > 0) {
							// no wall and no other unit
							for tmp_pos in unit_neighbours.iter() {
								if *map.get(tmp_pos).unwrap_or(&false) &&
									!units.values().any(|u| u.position == *tmp_pos && u.hp > 0) {
									let range = path_finder(map.clone(), width, height, units.clone(), *tmp_pos, *test);
									if range != -1 {
										target_range.push((*test, range, *tmp_pos));
										if range < nearest {
											nearest = range;
										}
									}
								}
							}
						}
					}
				}
				target_range = target_range.iter()
					.filter(|t| t.1 <= nearest)
					.map(|t| *t).collect();
				if target_range.len() > 0 {
					// These should all be reachable
					// Now find the best destination
					let mut target = target_range.iter()
						.fold(target_range[0],
							|a, x| if to_index(x.0, width) < to_index(a.0, width) 
								{ *x } else { a });
					target_range = target_range.iter()
						.filter(|t| t.0 == target.0)
						.map(|t| *t).collect();
					// Now find the best first step
					target = target_range.iter()
						.fold(target,
							|a, x| if to_index(x.2, width) < to_index(a.2, width) 
								{ *x } else { a });
					// OK, time to move
					let pos = target.2;
					tmp_unit.position = pos;
					let u = units.get_mut(&tmp_id).unwrap();
					u.position = pos;
					
					// Since we've moved, let's check if we now have a neighbour
					let testers = get_neighbours(pos);
					for test in testers.iter() {
						for t in targets.clone() {
							if t.1.position == *test {
								any_neighbour = true;
								break;
							}
						}
					}
				}
			}
			// Any potential moving done, let's fight!
			if any_neighbour {
				// Oh yes.
				let pos = tmp_unit.position;
				let testers: [(i32, i32); 4] = get_neighbours(pos);
				let mut target = targets[0];
				let mut hp = 201;
				for test in testers.iter() {
					for t in targets.clone() {
						if t.1.position == *test && t.1.hp < hp {
							target = t;
							hp = t.1.hp;
						}
					}
				}
				if hp < 201 {
					// OK, lose some hp
					let u = units.get_mut(&target.0).unwrap();
					u.hp = u.hp - 3;
					if u.hp <= 0 {
						let mut i = 0;
						for ou in tmp_units.clone() {
							if ou.1.position == target.1.position {
								tmp_units.remove(i);
								break;
							}
							i += 1;
						}
					}
				}
			}
		}
		units = units.iter().filter(|u| u.1.hp > 0).map(|u| (*u.0, *u.1)).collect();
		println!("After round {}: {} units left ({} goblins, {} elves)", round, units.len(), units.iter().filter(|u| u.1.is_goblin).count(), units.iter().filter(|u| !u.1.is_goblin).count());
		round += 1;
	}
	units = units.iter().filter(|u| u.1.hp > 0).map(|u| (*u.0, *u.1)).collect();
	draw_map(round+1, map.clone(), width, height, units.clone());
	
	let sum = units.values().fold(0, |a, u| a + u.hp);
	println!("Round {}: {} ({}*{})", round, sum * round, sum, round);
}
