use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct BingoTable {
	table: [[u64; 5]; 5],
	numbers: HashSet<u64>,
	checked: HashSet<u64>,
}

impl BingoTable {
	fn new() -> BingoTable {
		BingoTable {
			table: [[0; 5]; 5],
			numbers: HashSet::new(),
			checked: HashSet::new(),
		}
	}

	fn finish(&mut self) {
		let mut set = HashSet::new();
		for i in 0..5 {
			for j in 0..5 {
				set.insert(self.table[i][j]);
			}
		}
		self.numbers = set;
	}

	fn find_number_pos(&self, number: u64) -> Option<(usize, usize)> {
		for i in 0..5 {
			for j in 0..5 {
				if self.table[i][j] == number {
					return Some((i, j));
				}
			}
		}

		None
	}

	fn check_new_draw(&mut self, value: u64) -> Option<u64> {
		if self.numbers.contains(&value) {
			self.checked.insert(value);

			if self.checked.len() < 5 {
				return None;
			}

			let (x, y) = self.find_number_pos(value).unwrap();

			let mut found_horizontal = true;
			let mut found_vertical = true;

			for i in 0..5 {
				if !self.checked.contains(&self.table[i][y]) {
					found_horizontal = false;
					break;
				}
			}
			for i in 0..5 {
				if !self.checked.contains(&self.table[x][i]) {
					found_vertical = false;
					break;
				}
			}

			if found_horizontal || found_vertical {
				let sum: u64 = self.numbers.difference(&self.checked).sum();
				return Some(sum * value);
			}
		}
		None
	}
}

fn parse_input() -> Result<(Vec<u64>, Vec<BingoTable>), std::io::Error> {
	let file = File::open("input_04.txt")?;
	let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

	let draws: Vec<u64> = lines[0].split(',').map(|v| v.parse().unwrap()).collect();
	let mut tables = Vec::new();

	let mut bingo = BingoTable::new();
	let mut cur_line = 0;

	for i in 2..lines.len() {
		if lines[i].is_empty() {
			tables.push(bingo.clone());
			cur_line = 0;
			continue;
		}

		let numbers: Vec<u64> = lines[i]
			.split(' ')
			.filter(|v| !v.is_empty())
			.map(|v| v.parse().unwrap())
			.collect();
		assert_eq!(numbers.len(), 5);

		for j in 0..5 {
			bingo.table[cur_line][j] = numbers[j];
		}

		cur_line += 1;
	}
	tables.push(bingo.clone());

	for table in &mut tables {
		table.finish();
	}

	Ok((draws, tables))
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let (draws, mut tables) = parse_input()?;

	for draw in &draws {
		for table in &mut tables {
			if let Some(res) = table.check_new_draw(*draw) {
				return Ok(res);
			}
		}
	}

	Err(std::io::Error::new(
		std::io::ErrorKind::Other,
		"Internal error!",
	))
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let (draws, mut tables) = parse_input()?;

	for draw in &draws {
		let mut tables_not_won: Vec<BingoTable> = Vec::new();
		let num_tables_left = tables.len();
		for table in &mut tables {
			if let Some(res) = table.check_new_draw(*draw) {
				if num_tables_left == 1 {
					return Ok(res);
				}
			} else {
				tables_not_won.push(table.clone());
			}
		}

		tables = tables_not_won;
	}

	Err(std::io::Error::new(
		std::io::ErrorKind::Other,
		"Internal error!",
	))
}
