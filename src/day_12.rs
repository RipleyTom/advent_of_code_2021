use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

use u32 as NodeId;

#[derive(PartialEq)]
enum NodeType {
	Start(),
	End(),
	Big(char),
	Small(char),
}

struct Node {
	node_type: NodeType,
	links: HashSet<NodeId>,
}

impl Node {
	pub fn new(node_type: NodeType) -> Node {
		Node { node_type, links: HashSet::new() }
	}
}

struct Network {
	nodes: HashMap<NodeId, Node>,
	node_counter: u32,
}

impl Network {
	pub fn new() -> Network {
		Network {
			nodes: HashMap::new(),
			node_counter: 0,
		}
	}

	fn insert_node(&mut self, node_type: NodeType) -> NodeId {
		for (n_id, node) in &self.nodes {
			if node.node_type == node_type {
				return *n_id;
			}
		}
		self.nodes.insert(self.node_counter, Node::new(node_type));
		self.node_counter += 1;
		return self.node_counter - 1;
	}

	pub fn add_node(&mut self, desc: &str) -> NodeId {
		if desc == "start" {
			return self.insert_node(NodeType::Start());
		}

		if desc == "end" {
			return self.insert_node(NodeType::End());
		}

		let first_char = desc.chars().next().unwrap();

		let node_type = match first_char {
			'a'..='z' => NodeType::Small(first_char),
			'A'..='Z' => NodeType::Big(first_char),
			_ => panic!("Invalid first character!"),
		};

		return self.insert_node(node_type);
	}

	pub fn add_link(&mut self, first: NodeId, second: NodeId) {
		self.nodes.get_mut(&first).unwrap().links.insert(second);
		self.nodes.get_mut(&second).unwrap().links.insert(first);
	}
}

fn parse_input() -> Result<Network, std::io::Error> {
	let file = File::open("input_12.txt")?;

	let mut network = Network::new();

	let lines = BufReader::new(file).lines();
	for l in lines {
		let l = l.unwrap();
		let (first, second) = l.split_once('-').unwrap();
		let first_id = network.add_node(first);
		let second_id = network.add_node(second);
		network.add_link(first_id, second_id);
	}

	Ok(network)
}

fn visit_tree(network: &Network, n_id: NodeId, mut visited: HashSet<NodeId>, start_node: NodeId, end_node: NodeId, begin: bool) -> u64 {
	if n_id == start_node && !begin {
		return 0;
	}

	if n_id == end_node {
		return 1;
	}

	match network.nodes[&n_id].node_type {
		NodeType::Small(_) => {
			if visited.contains(&n_id) {
				return 0;
			}

			visited.insert(n_id);
		}
		_ => {}
	}

	let mut valid_paths = 0;
	for l in &network.nodes[&n_id].links {
		valid_paths += visit_tree(network, *l, visited.clone(), start_node, end_node, false);
	}

	valid_paths
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let network = parse_input()?;

	let mut start_node = None;
	let mut end_node = None;
	for (n_id, n) in &network.nodes {
		if n.node_type == NodeType::Start() {
			start_node = Some(*n_id);
		}

		if n.node_type == NodeType::End() {
			end_node = Some(*n_id);
		}

		if start_node.is_some() && end_node.is_some() {
			break;
		}
	}
	let start_node = start_node.unwrap();
	let end_node = end_node.unwrap();

	let res = visit_tree(&network, start_node, HashSet::new(), start_node, end_node, true);

	Ok(res)
}

fn visit_tree_b(network: &Network, n_id: NodeId, visited: &mut HashSet<NodeId>, start_node: NodeId, end_node: NodeId, mut twice_visited: bool) -> u64 {
	if n_id == start_node {
		return 0;
	}

	if n_id == end_node {
		return 1;
	}

	let mut inserted = false;

	match network.nodes[&n_id].node_type {
		NodeType::Small(_) => {
			if visited.contains(&n_id) {
				if twice_visited {
					return 0;
				} else {
					twice_visited = true;
				}
			} else {
				visited.insert(n_id);
				inserted = true;
			}
		}
		_ => {}
	}

	let mut valid_paths = 0;
	for l in &network.nodes[&n_id].links {
		valid_paths += visit_tree_b(network, *l, visited, start_node, end_node, twice_visited);
	}

	if inserted {
		visited.remove(&n_id);
	}

	valid_paths
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let network = parse_input()?;

	let mut start_node = None;
	let mut end_node = None;
	for (n_id, n) in &network.nodes {
		if n.node_type == NodeType::Start() {
			start_node = Some(*n_id);
		}

		if n.node_type == NodeType::End() {
			end_node = Some(*n_id);
		}

		if start_node.is_some() && end_node.is_some() {
			break;
		}
	}
	let start_node = start_node.unwrap();
	let end_node = end_node.unwrap();

	let mut res = 0;
	for l in &network.nodes[&start_node].links {
		res += visit_tree_b(&network, *l, &mut HashSet::new(), start_node, end_node, false);
	}

	Ok(res)
}
