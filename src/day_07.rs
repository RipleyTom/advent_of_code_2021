use std::fs::File;
use std::io::Read;

fn parse_input() -> Result<Vec<u64>, std::io::Error> {
	let mut file = File::open("input_07.txt")?;
	let mut str_file = String::new();
	file.read_to_string(&mut str_file)?;
	let res = str_file.trim().split(',').map(|v| v.parse().unwrap()).collect();
	Ok(res)
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let pos = parse_input()?;

	let min = *pos.iter().min().unwrap();
	let max = *pos.iter().max().unwrap();

	let mut fuels = Vec::new();

	for i in min..=max {
		let mut fuel = 0;
		for p in &pos {
			fuel += if i > *p { i - *p } else { *p - i };
		}
		fuels.push(fuel);
	}

	Ok(*fuels.iter().min().unwrap())
}

fn triangular_number(n: u64) -> u64 {
	(n * (n + 1)) / 2
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let pos = parse_input()?;

	let min = *pos.iter().min().unwrap();
	let max = *pos.iter().max().unwrap();

	let mut fuels = Vec::new();

	for i in min..=max {
		let mut fuel = 0;
		for p in &pos {
			fuel += triangular_number(if i > *p { i - *p } else { *p - i });
		}
		fuels.push(fuel);
	}

	Ok(*fuels.iter().min().unwrap())
}
