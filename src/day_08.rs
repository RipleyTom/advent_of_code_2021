use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct SignalsNumbers {
	signals: Vec<HashSet<u8>>,
	numbers: Vec<HashSet<u8>>,
}

fn parse_input() -> Result<Vec<SignalsNumbers>, std::io::Error> {
	let file = File::open("input_08.txt")?;
	let lines: Vec<SignalsNumbers> = BufReader::new(file)
		.lines()
		.map(|l| {
			let l = l.unwrap();
			let parts: Vec<&str> = l.split(" | ").collect();
			assert_eq!(parts.len(), 2);

			let parse_data = |e: &str| -> Vec<HashSet<u8>> { e.split(' ').map(|e| e.chars().map(|c| c as u8 - 'a' as u8).collect()).collect() };

			let signals = parse_data(&parts[0]);
			let numbers = parse_data(&parts[1]);
			SignalsNumbers { signals, numbers }
		})
		.collect();

	Ok(lines)
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let entries = parse_input()?;

	let sum = entries
		.iter()
		.map(|e| {
			e.numbers
				.iter()
				.map(|v| match v.len() {
					2 | 4 | 3 | 7 => 1,
					_ => 0,
				})
				.sum::<u64>()
		})
		.sum();

	Ok(sum)
}

fn find_number(numbers: &mut Vec<HashSet<u8>>, segments: usize, subsets: Option<Vec<&HashSet<u8>>>) -> HashSet<u8> {
	'main_loop: for i in 0..numbers.len() {
		if let Some(ref v) = subsets {
			for s in v {
				if !numbers[i].is_superset(s) {
					continue 'main_loop;
				}
			}
		}

		if numbers[i].len() == segments {
			return numbers.remove(i);
		}
	}

	panic!("Couldn't find number!");
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let mut entries = parse_input()?;
	let mut sum = 0;

	// If you want a smart solution, look elsewhere :D
	for e in &mut entries {
		let mut numbers = vec![HashSet::new(); 10];
		numbers[1] = find_number(&mut e.signals, 2, None);
		numbers[4] = find_number(&mut e.signals, 4, None);
		numbers[7] = find_number(&mut e.signals, 3, None);
		numbers[8] = find_number(&mut e.signals, 7, None);
		numbers[9] = find_number(&mut e.signals, 6, Some(vec![&numbers[1], &numbers[4], &numbers[7]]));
		numbers[0] = find_number(&mut e.signals, 6, Some(vec![&numbers[1], &numbers[7]]));
		numbers[6] = find_number(&mut e.signals, 6, None);
		numbers[3] = find_number(&mut e.signals, 5, Some(vec![&numbers[1], &numbers[7]]));
		if numbers[6].is_superset(&e.signals[0]) {
			numbers[5] = e.signals[0].clone();
			numbers[2] = e.signals[1].clone();
		} else {
			numbers[2] = e.signals[0].clone();
			numbers[5] = e.signals[1].clone();
		}

		let mut str_number = String::new();

		for i in 0..4 {
			for j in 0..10 {
				if e.numbers[i] == numbers[j] {
					str_number.push(('0' as u8 + j as u8) as char);
					break;
				}
			}
		}

		sum += str_number.parse::<u64>().unwrap();
	}

	Ok(sum)
}
