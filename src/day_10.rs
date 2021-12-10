use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<Vec<Vec<char>>, std::io::Error> {
	let file = File::open("input_10.txt")?;

	let result = BufReader::new(file).lines().map(|line| line.unwrap().trim().chars().collect()).collect();

	Ok(result)
}

fn get_first_invalid_char(v: &Vec<char>) -> Option<char> {
	let mut next_closing: Vec<char> = Vec::new();

	for c in v {
		match *c {
			'(' => next_closing.push(')'),
			'[' => next_closing.push(']'),
			'{' => next_closing.push('}'),
			'<' => next_closing.push('>'),
			')' | ']' | '}' | '>' => {
				if next_closing.is_empty() || next_closing.pop().unwrap() != *c {
					return Some(*c);
				}
			}
			_ => panic!("Invalid character!"),
		}
	}

	None
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let lines = parse_input()?;

	let mut sum = 0;

	for l in &lines {
		if let Some(c) = get_first_invalid_char(l) {
			sum += match c {
				')' => 3,
				']' => 57,
				'}' => 1197,
				'>' => 25137,
				_ => panic!("Invalid character!"),
			};
		}
	}

	Ok(sum)
}

fn get_completion_score(v: &Vec<char>) -> u64 {
	let mut next_closing: Vec<char> = Vec::new();

	for c in v {
		match *c {
			'(' => next_closing.push(')'),
			'[' => next_closing.push(']'),
			'{' => next_closing.push('}'),
			'<' => next_closing.push('>'),
			')' | ']' | '}' | '>' => {
				next_closing.pop().unwrap();
			}
			_ => panic!("Invalid character!"),
		}
	}

	let mut res = 0;

	while !next_closing.is_empty() {
		let c = next_closing.pop().unwrap();
		let v = match c {
			')' => 1,
			']' => 2,
			'}' => 3,
			'>' => 4,
			_ => panic!("Invalid character!"),
		};

		res = (res * 5) + v;
	}

	res
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let lines = parse_input()?;
	let mut incomp_lines = Vec::new();

	for l in &lines {
		if let None = get_first_invalid_char(l) {
			incomp_lines.push(l);
		}
	}

	let mut scores: Vec<u64> = incomp_lines.iter().map(|l| get_completion_score(l)).collect();
	scores.sort();

	Ok(scores[scores.len() / 2])
}
