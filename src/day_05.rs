use std::fs::File;
use std::io::{BufRead, BufReader};

struct Line {
	x_o: u64,
	x_d: u64,
	y_o: u64,
	y_d: u64,
}

impl Line {
	fn new(x_o: u64, y_o: u64, x_d: u64, y_d: u64) -> Line {
		Line { x_o, x_d, y_o, y_d }
	}

	fn get_points(&self) -> Vec<(u64, u64)> {
		let mut res = Vec::new();
		let mut x = self.x_o;
		let mut y = self.y_o;

		let adjust = |v: &mut u64, goal: u64| {
			if *v < goal {
				*v += 1;
			} else if *v > goal {
				*v -= 1;
			}
		};

		loop {
			res.push((x, y));

			if x == self.x_d && y == self.y_d {
				break;
			}

			adjust(&mut x, self.x_d);
			adjust(&mut y, self.y_d);
		}
		res
	}
}

fn parse_input() -> Result<Vec<Line>, std::io::Error> {
	let file = File::open("input_05.txt")?;
	let lines: Vec<Line> = BufReader::new(file)
		.lines()
		.map(|l| {
			let l = l.unwrap();
			let parts: Vec<&str> = l.split(" -> ").collect();
			assert_eq!(parts.len(), 2);
			let values: Vec<Vec<u64>> = parts.iter().map(|p| p.split(',').map(|v| v.parse().unwrap()).collect()).collect();
			Line::new(values[0][0], values[0][1], values[1][0], values[1][1])
		})
		.collect();

	Ok(lines)
}

fn get_intersections(lines: Vec<&Line>) -> u64 {
	let mut ocean = [[0; 1000]; 1000];

	for l in lines {
		let points = l.get_points();
		for (x, y) in points {
			ocean[x as usize][y as usize] += 1;
		}
	}

	ocean.iter().map(|l| l.iter().map(|v| if *v > 1 { 1 } else { 0 }).sum::<u64>()).sum()
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let lines = parse_input()?;
	let lines: Vec<&Line> = lines.iter().filter(|l| l.x_o == l.x_d || l.y_o == l.y_d).collect();

	Ok(get_intersections(lines))
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let lines = parse_input()?;
	let lines: Vec<&Line> = lines.iter().collect();

	Ok(get_intersections(lines))
}
