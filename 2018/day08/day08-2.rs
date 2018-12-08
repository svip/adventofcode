use std::io;
use std::io::prelude::*;

type NodeId = usize;

struct Node {
	parent: Option<NodeId>,
	node_no: i32,
	nodes: Vec<NodeId>,
	data_no: i32,
	data: Vec<i32>,
	value: i32,
}

fn main() {
	let stdin = io::stdin();
	let mut current = 0;
	let mut arena = vec![Node {
		parent: None, 
		node_no: -1, 
		nodes: vec![], 
		data_no: -1, 
		data: vec![],
		value: -1 }];
	for l in stdin.lock().lines() {
		let line = l.unwrap();
		let numbers: Vec<i32> = line.split(' ').map(|e| e.parse::<i32>().unwrap()).collect();
		for no in numbers {
			while arena[current].node_no == arena[current].nodes.len() as i32
				&& arena[current].data_no == arena[current].data.len() as i32 {
				match arena[current].parent {
				Some(n) => { current = n; }
				None => { panic!("Oh no!"); }
				}
			}
			if arena[current].node_no > -1 && arena[current].data_no > -1 {
				// initalised
				if arena[current].node_no > arena[current].nodes.len() as i32 {
					let next_index = arena.len();
					arena[current].nodes.push(next_index);
					arena.push(Node{
						parent: Some(current),
						node_no: no, 
						nodes: vec![], 
						data_no: -1, 
						data: vec![],
						value: -1});
					current = next_index;
					continue;
				} else if arena[current].data_no > arena[current].data.len() as i32 {
					arena[current].data.push(no);
					continue;
				}
			}
			if arena[current].node_no == -1 { // not initialised
				arena[current].node_no = no;
				continue;
			}
			if arena[current].data_no == -1 {
				arena[current].data_no = no;
				continue;
			}
		}
	}
	
	let mut nodes_done = 0;
	while nodes_done < arena.len() {
		'forloop: for nodeid in 0..arena.len() {
			let node_value = arena[nodeid].value.clone();
			if node_value == -1 {
				// value not done.
				let mut value = 0;
				let node_data = arena[nodeid].data.clone();
				let node_nodes = arena[nodeid].nodes.clone();
				if node_nodes.len() == 0 {
					value = node_data.iter().fold(0, |a, e| a + e);
				} else {
					for entity in &node_data {
						if *entity as usize <= node_nodes.len() {
							let id = node_nodes[(entity-1) as usize];
							let othervalue = arena[id].value.clone();
							if othervalue > -1 {
								value += othervalue;
							} else {
								continue 'forloop;
							}
						}
					}
				}
				arena[nodeid].value = value;
				nodes_done += 1;
			}
		}
	}
	println!("{}", arena[0].value);
}
