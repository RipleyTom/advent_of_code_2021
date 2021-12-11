use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<Vec<Vec<u8>>, std::io::Error> {
	let file = File::open("input_11.txt")?;

	let result = BufReader::new(file)
		.lines()
		.map(|line| line.unwrap().trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
		.collect();

	Ok(result)
}

fn flash(octs: &mut Vec<Vec<u8>>, x: usize, y: usize, max_x: usize, max_y: usize) -> u64 {
	let mut num_flashes = 1;

	let start_x = if x == 0 { 0 } else { x - 1 };
	let start_y = if y == 0 { 0 } else { y - 1 };
	let end_x = if x == (max_x - 1) { max_x } else { x + 2 };
	let end_y = if y == (max_y - 1) { max_y } else { y + 2 };

	for ny in start_y..end_y {
		for nx in start_x..end_x {
			if nx == x && ny == y {
				continue;
			}

			octs[ny][nx] += 1;
			if octs[ny][nx] == 10 {
				num_flashes += flash(octs, nx, ny, max_x, max_y);
			}
		}
	}

	num_flashes
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let mut octs = parse_input()?;

	let max_x = octs[0].len();
	let max_y = octs.len();

	let mut num_flashes = 0;

	for _ in 0..100 {
		for y in 0..max_y {
			for x in 0..max_x {
				octs[y][x] += 1;
				if octs[y][x] == 10 {
					num_flashes += flash(&mut octs, x, y, max_x, max_y);
				}
			}
		}

		for y in 0..max_y {
			for x in 0..max_x {
				if octs[y][x] >= 10 {
					octs[y][x] = 0;
				}
			}
		}
	}

	Ok(num_flashes)
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let mut octs = parse_input()?;

	let max_x = octs[0].len();
	let max_y = octs.len();

	let mut step = 0;

	loop {
		step += 1;
		let mut num_flashes = 0;
		for y in 0..max_y {
			for x in 0..max_x {
				octs[y][x] += 1;
				if octs[y][x] == 10 {
					num_flashes += flash(&mut octs, x, y, max_x, max_y);
				}
			}
		}

		if num_flashes == 100 {
			break;
		}

		for y in 0..max_y {
			for x in 0..max_x {
				if octs[y][x] >= 10 {
					octs[y][x] = 0;
				}
			}
		}
	}

	Ok(step)
}
