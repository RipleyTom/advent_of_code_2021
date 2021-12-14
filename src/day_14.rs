use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<(Vec<u8>, Vec<(u8, u8, u8)>), std::io::Error> {
	let file = File::open("input_14.txt")?;

	let mut lines = BufReader::new(file).lines();

	let input: Vec<u8> = lines.next().unwrap().unwrap().chars().map(|c| c as u8 - 'A' as u8).collect();
	lines.next();

	let ins: Vec<(u8, u8, u8)> = lines
		.map(|l| {
			let l = l.unwrap();
			let (beg, end) = l.split_once(" -> ").unwrap();
			assert_eq!(beg.len(), 2);
			assert_eq!(end.len(), 1);

			(
				beg.chars().nth(0).unwrap() as u8 - 'A' as u8,
				beg.chars().nth(1).unwrap() as u8 - 'A' as u8,
				end.chars().nth(0).unwrap() as u8 - 'A' as u8,
			)
		})
		.collect();

	Ok((input, ins))
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let (mut input, ins) = parse_input()?;

	let mut output: Vec<u8> = Vec::new();

	for _ in 0..10 {
		for i in 0..(input.len() - 1) {
			output.push(input[i]);
			for (a, b, c) in &ins {
				if input[i] == *a && input[i + 1] == *b {
					output.push(*c);
					break;
				}
			}
		}
		output.push(input[input.len() - 1]);

		input = output;
		output = Vec::new();
	}

	let mut occ_count = [0; u8::MAX as usize + 1];

	input.iter().for_each(|v| occ_count[*v as usize] += 1);
	let (mut min, mut max) = (u64::MAX, 0);
	occ_count.iter().for_each(|v| {
		let v = *v;
		if v != 0 {
			if v > max {
				max = v;
			}
			if v < min {
				min = v;
			}
		}
	});

	Ok(max - min)
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let (input, ins) = parse_input()?;

	let pair_from_u8 = |pa: u8, pb: u8| -> u16 { ((pa as u16) << 8) + pb as u16 };

	let mut pairs: HashMap<u16, u64> = HashMap::new();
	for i in 0..(input.len() - 1) {
		let pair = pair_from_u8(input[i], input[i + 1]);
		*pairs.entry(pair).or_insert(0) += 1;
	}

	let pins: Vec<(u16, u16, u16)> = ins
		.iter()
		.map(|(a, b, c)| {
			let pair = pair_from_u8(*a, *b);
			let pair_a = pair_from_u8(*a, *c);
			let pair_b = pair_from_u8(*c, *b);
			(pair, pair_a, pair_b)
		})
		.collect();

	let mut new_pairs = pairs.clone();

	for _ in 0..40 {
		for p in &pins {
			if let Some(v) = pairs.get(&p.0) {
				if *v > 0 {
					*new_pairs.get_mut(&p.0).unwrap() -= v;
					*new_pairs.entry(p.1).or_insert(0) += v;
					*new_pairs.entry(p.2).or_insert(0) += v;
				}
			}
		}
		pairs = new_pairs.clone();
	}

	let mut occ_count = [0; u8::MAX as usize + 1];

	for p in &pairs {
		let p_a = (p.0 >> 8) as usize;
		let p_b = (p.0 & 0xFF) as usize;
		occ_count[p_a] += p.1;
		occ_count[p_b] += p.1;
	}

	for v in &mut occ_count {
		*v /= 2;
	}

	occ_count[input[0] as usize] += 1;
	occ_count[input[input.len() - 1] as usize] += 1;

	let (mut min, mut max) = (u64::MAX, 0);
	occ_count.iter().for_each(|v| {
		let v = *v;
		if v != 0 {
			if v > max {
				max = v;
			}
			if v < min {
				min = v;
			}
		}
	});

	Ok(max - min)
}
