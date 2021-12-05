use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<Vec<i64>, std::io::Error> {
	let file = File::open("input_01.txt")?;

	let result: Vec<i64> = BufReader::new(file).lines().map(|line| line.unwrap().trim().parse().unwrap()).collect();

	Ok(result)
}

pub fn run_a() -> Result<i64, std::io::Error> {
	let input = parse_input()?;

	let mut num_increases = 0;

	for i in 1..input.len() {
		if input[i - 1] < input[i] {
			num_increases += 1;
		}
	}

	Ok(num_increases)
}

pub fn run_b() -> Result<i64, std::io::Error> {
	let input = parse_input()?;
	let mut num_increases = 0;

	let mut vec_windows: Vec<i64> = Vec::new();

	for i in 0..(input.len() - 2) {
		vec_windows.push(input[i] + input[i + 1] + input[i + 2]);
	}

	for i in 1..vec_windows.len() {
		if vec_windows[i - 1] < vec_windows[i] {
			num_increases += 1;
		}
	}

	Ok(num_increases)
}
