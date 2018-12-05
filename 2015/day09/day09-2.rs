use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

fn find_route(d: i32, path: Vec<String>, city_count: usize, map: HashMap<(String, String), i32>, result: &mut Vec<(Vec<String>, i32)>) {
	if path.len() == city_count {
		result.push((path, d));
	} else {
		let city = path.last().unwrap();
		for (k, v) in map.clone() {
			if k.0 == *city {
				if !path.contains(&k.1) {
					let mut p = path.clone();
					p.push(k.1);
					find_route(d+v, p, city_count, map.clone(), result); 
				}
			}
		}
	}
}

fn main() {
	let stdin = io::stdin();
	let mut set: Vec<String> = vec![];
	let mut map: HashMap<(String, String), i32> = HashMap::new();;
	
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let v: Vec<&str> = line.split(' ').collect();
		let (c1, c2, d) = (v[0].to_string(), v[2].to_string(), v[4].parse::<i32>().unwrap());
		if !set.contains(&c1) { set.push(c1.clone()); }
		if !set.contains(&c2) { set.push(c2.clone()); }
		*map.entry((c1.clone(), c2.clone())).or_insert(0) = d;
		*map.entry((c2.clone(), c1.clone())).or_insert(0) = d;
	}
	
	let mut result = vec![];
	for city in set.clone() {
		find_route(0, vec![city], set.len(), map.clone(), &mut result);
	}
	let longest = result.iter().fold(0, |a, x| if x.1 > a { x.1 } else { a });
	println!("{}", longest);
}
