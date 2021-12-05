use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
	Forward(i64),
	Up(i64),
	Down(i64),
}

struct Submarine {
	hoz_pos: i64,
	depth_pos: i64,
	aim: i64,
}

impl Submarine {
	fn new() -> Submarine {
		Submarine { hoz_pos: 0, depth_pos: 0, aim: 0 }
	}

	fn apply_directions(&mut self, vec_dirs: &Vec<Direction>) {
		for d in vec_dirs {
			match d {
				Direction::Forward(dist) => self.hoz_pos += dist,
				Direction::Up(dist) => self.depth_pos -= dist,
				Direction::Down(dist) => self.depth_pos += dist,
			}
		}
	}

	fn apply_new_directions(&mut self, vec_dirs: &Vec<Direction>) {
		for d in vec_dirs {
			match d {
				Direction::Forward(dist) => {
					self.hoz_pos += dist;
					self.depth_pos += self.aim * dist;
				}
				Direction::Up(dist) => self.aim -= dist,
				Direction::Down(dist) => self.aim += dist,
			}
		}
	}
}

fn parse_input() -> Result<Vec<Direction>, std::io::Error> {
	let file = File::open("input_02.txt")?;

	let result: Vec<Direction> = BufReader::new(file)
		.lines()
		.filter_map(|line| {
			let line = line.unwrap().trim().to_string();
			let infos: Vec<&str> = line.split(' ').collect();
			if infos.len() != 2 {
				return None;
			}
			let distance: i64 = infos[1].parse().unwrap();
			match infos[0] {
				"forward" => Some(Direction::Forward(distance)),
				"up" => Some(Direction::Up(distance)),
				"down" => Some(Direction::Down(distance)),
				_ => None,
			}
		})
		.collect();
	Ok(result)
}

pub fn run_a() -> Result<i64, std::io::Error> {
	let input = parse_input()?;

	let mut sub = Submarine::new();
	sub.apply_directions(&input);

	Ok(sub.hoz_pos * sub.depth_pos)
}

pub fn run_b() -> Result<i64, std::io::Error> {
	let input = parse_input()?;

	let mut sub = Submarine::new();
	sub.apply_new_directions(&input);

	Ok(sub.hoz_pos * sub.depth_pos)
}
