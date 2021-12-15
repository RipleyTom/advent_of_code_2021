use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<Vec<Vec<u8>>, std::io::Error> {
	let file = File::open("input_15.txt")?;

	let map = BufReader::new(file)
		.lines()
		.map(|l| {
			let l = l.unwrap();
			l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
		})
		.collect();

	Ok(map)
}

fn get_min_risk(x: usize, y: usize, risk_map: &Vec<Vec<u64>>) -> u64 {
	let max_x = risk_map[0].len();
	let max_y = risk_map.len();

	let mut min = u64::MAX;
	if x != 0 {
		min = risk_map[y][x - 1];
	}
	if x != (max_x - 1) && risk_map[y][x + 1] < min {
		min = risk_map[y][x + 1];
	}
	if y != 0 && risk_map[y - 1][x] < min {
		min = risk_map[y - 1][x];
	}
	if y != (max_y - 1) && risk_map[y + 1][x] < min {
		min = risk_map[y + 1][x];
	}

	min
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let map = parse_input()?;

	let max_x = map[0].len();
	let max_y = map.len();

	let mut risk_map: Vec<Vec<u64>> = vec![vec![u64::MAX; max_x]; max_y];

	risk_map[0][0] = 0;

	let mut changed = true;

	while changed {
		changed = false;
		for y in 0..max_y {
			for x in 0..max_x {
				if x == 0 && y == 0 {
					continue;
				}

				let res = get_min_risk(x, y, &risk_map) + map[y][x] as u64;
				if risk_map[y][x] != res {
					changed = true;
					risk_map[y][x] = res;
				}
			}
		}
	}

	Ok(risk_map[max_y - 1][max_x - 1])
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let original_map = parse_input()?;
	let ori_x = original_map[0].len();
	let ori_y = original_map.len();

	let mut map: Vec<Vec<u8>> = vec![vec![0; ori_x * 5]; ori_y * 5];

	for i_y in 0..5 {
		for i_x in 0..5 {
			for y in 0..ori_y {
				for x in 0..ori_x {
					let mut value = original_map[y][x] + i_x as u8 + i_y as u8;
					if value >= 10 {
						value -= 9;
					}
					map[y + (ori_y * i_y)][x + (ori_x * i_x)] = value;
				}
			}
		}
	}

	let max_x = map[0].len();
	let max_y = map.len();

	let mut risk_map: Vec<Vec<u64>> = vec![vec![u64::MAX; max_x]; max_y];

	risk_map[0][0] = 0;

	let mut changed = true;

	while changed {
		changed = false;
		for y in 0..max_y {
			for x in 0..max_x {
				if x == 0 && y == 0 {
					continue;
				}

				let res = get_min_risk(x, y, &risk_map) + map[y][x] as u64;
				if risk_map[y][x] != res {
					changed = true;
					risk_map[y][x] = res;
				}
			}
		}
	}

	Ok(risk_map[max_y - 1][max_x - 1])
}
