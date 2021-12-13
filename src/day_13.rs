use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point {
	x: usize,
	y: usize,
}

impl Point {
	pub fn new(x: usize, y: usize) -> Point {
		Point { x, y }
	}
}

enum FoldAxis {
	XAxis,
	YAxis,
}

struct Fold {
	axis: FoldAxis,
	coord: usize,
}

impl Fold {
	pub fn new(axis: FoldAxis, coord: usize) -> Fold {
		Fold { axis, coord }
	}
}

fn parse_input() -> Result<(Vec<Point>, Vec<Fold>), std::io::Error> {
	let file = File::open("input_13.txt")?;

	let mut list_folds = false;

	let mut vec_points = Vec::new();
	let mut vec_folds = Vec::new();

	for l in BufReader::new(file).lines() {
		let l = l.unwrap();

		if l.is_empty() {
			list_folds = true;
			continue;
		}

		if list_folds {
			let (axis, coord) = l.split_once('=').unwrap();
			let axis = match axis {
				"fold along x" => FoldAxis::XAxis,
				"fold along y" => FoldAxis::YAxis,
				_ => panic!("Invalid axis!"),
			};
			vec_folds.push(Fold::new(axis, coord.parse().unwrap()));
		} else {
			let (x, y) = l.split_once(',').unwrap();
			vec_points.push(Point::new(x.parse().unwrap(), y.parse().unwrap()));
		}
	}

	Ok((vec_points, vec_folds))
}

fn print_points(dots: &Vec<Point>) {
	let mut max_x = 0;
	let mut max_y = 0;

	for d in dots {
		max_x = std::cmp::max(d.x, max_x);
		max_y = std::cmp::max(d.y, max_y);
	}

	max_x += 1;
	max_y += 1;

	let mut map = vec![vec!['.'; max_x]; max_y];

	for d in dots {
		map[d.y][d.x] = '#';
	}

	println!("-");
	for y in 0..max_y {
		let line: String = map[y].iter().collect();
		println!("{}", line);
	}
	println!("-");
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let (mut dots, folds) = parse_input()?;

	let coord = folds[0].coord;

	match folds[0].axis {
		FoldAxis::XAxis => {
			for d in &mut dots {
				if d.x > coord {
					d.x = coord - (d.x - coord);
				}
			}
		}
		FoldAxis::YAxis => {
			for d in &mut dots {
				if d.y > coord {
					d.y = coord - (d.y - coord);
				}
			}
		}
	}

	let list_points: HashSet<(usize, usize)> = dots.iter().map(|d| (d.x, d.y)).collect();

	Ok(list_points.len() as u64)
}

pub fn run_b() -> Result<(), std::io::Error> {
	let (mut dots, folds) = parse_input()?;

	for f in &folds {
		let coord = f.coord;

		match f.axis {
			FoldAxis::XAxis => {
				for d in &mut dots {
					if d.x >= coord {
						d.x = coord - (d.x - coord);
					}
				}
			}
			FoldAxis::YAxis => {
				for d in &mut dots {
					if d.y >= coord {
						d.y = coord - (d.y - coord);
					}
				}
			}
		}
	}

	print_points(&dots);

	Ok(())
}
