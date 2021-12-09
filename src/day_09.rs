use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<Vec<Vec<u8>>, std::io::Error> {
	let file = File::open("input_09.txt")?;
	let lines: Vec<Vec<u8>> = BufReader::new(file)
		.lines()
		.map(|l| l.unwrap().trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
		.collect();

	Ok(lines)
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let dm = parse_input()?;

	let max_x = dm[0].len();
	let max_y = dm.len();

	let mut risk_sum = 0;

	for y in 0..max_y {
		for x in 0..max_x {
			let value = dm[y][x];

			if x != 0 && value >= dm[y][x - 1] {
				continue;
			}

			if x != (max_x - 1) && value >= dm[y][x + 1] {
				continue;
			}

			if y != 0 && value >= dm[y - 1][x] {
				continue;
			}

			if y != (max_y - 1) && value >= dm[y + 1][x] {
				continue;
			}

			risk_sum += (1 + value) as u64;
		}
	}

	Ok(risk_sum)
}

fn flood_fill(dm: &Vec<Vec<u8>>, ffm: &mut Vec<Vec<u8>>, x: usize, y: usize, max_x: usize, max_y: usize) -> u64 {
	if dm[y][x] == 9 {
		return 0;
	}

	let mut res = 1;
	ffm[y][x] = 1;

	if x != 0 && ffm[y][x - 1] == 0 {
		res += flood_fill(dm, ffm, x - 1, y, max_x, max_y);
	}

	if x != (max_x - 1) && ffm[y][x + 1] == 0 {
		res += flood_fill(dm, ffm, x + 1, y, max_x, max_y);
	}

	if y != 0 && ffm[y - 1][x] == 0 {
		res += flood_fill(dm, ffm, x, y - 1, max_x, max_y);
	}

	if y != (max_y - 1) && ffm[y + 1][x] == 0 {
		res += flood_fill(dm, ffm, x, y + 1, max_x, max_y);
	}

	res
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let dm = parse_input()?;

	let max_x = dm[0].len();
	let max_y = dm.len();

	let mut low_points = Vec::new();

	for y in 0..max_y {
		for x in 0..max_x {
			let value = dm[y][x];

			if x != 0 && value >= dm[y][x - 1] {
				continue;
			}

			if x != (max_x - 1) && value >= dm[y][x + 1] {
				continue;
			}

			if y != 0 && value >= dm[y - 1][x] {
				continue;
			}

			if y != (max_y - 1) && value >= dm[y + 1][x] {
				continue;
			}

			low_points.push((x, y));
		}
	}

	let mut fill_map: Vec<Vec<u8>> = vec![vec![0; max_x]; max_y];
	let mut basin_sizes: Vec<u64> = Vec::new();

	for (x, y) in &low_points {
		basin_sizes.push(flood_fill(&dm, &mut fill_map, *x, *y, max_x, max_y));
	}

	basin_sizes.sort_by(|a, b| b.cmp(a));
	Ok(basin_sizes[0] * basin_sizes[1] * basin_sizes[2])
}
