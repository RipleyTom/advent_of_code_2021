use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Result<Vec<[u64; 12]>, std::io::Error> {
	let file = File::open("input_03.txt")?;

	let mut num_lines = 0;

	let result = BufReader::new(file)
		.lines()
		.filter_map(|line| {
			let line = line.unwrap().trim().to_string();
			if line.len() != 12 {
				return None;
			}

			num_lines += 1;

			let mut value = [0; 12];
			let mut num = 0;

			for c in line.chars() {
				match c {
					'0' => {}
					'1' => value[num] = 1,
					_ => {}
				}
				num += 1;
			}
			Some(value)
		})
		.collect();

	Ok(result)
}

fn count_1s(v_inp: &Vec<[u64; 12]>) -> [u64; 12] {
	let mut num_1s = [0; 12];

	for v in v_inp {
		for i in 0..12 {
			if v[i] == 1 {
				num_1s[i] += 1;
			}
		}
	}

	num_1s
}

fn array_to_number(arr: &[u64; 12]) -> u64 {
	let mut res = 0;
	for i in 0..12 {
		if arr[i] == 1 {
			res |= 1 << (11 - i);
		}
	}

	res
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let v_inp = parse_input()?;

	let num_1s = count_1s(&v_inp);

	let mut arr_result = [0; 12];
	for i in 0..12 {
		if num_1s[i] > (v_inp.len() / 2) as u64 {
			arr_result[i] = 1;
		}
	}

	let final_result = array_to_number(&arr_result);

	Ok(final_result * (!final_result & 0b111111111111))
}

fn get_most_popular_bit(v_inp: &Vec<&[u64; 12]>, depth: u64) -> u64 {
	let mut num_1s = 0;

	for v in v_inp {
		if v[depth as usize] == 1 {
			num_1s += 1;
		}
	}

	if num_1s >= (v_inp.len() / 2) {
		1
	} else {
		0
	}
}

fn search_for(v_inp: &Vec<[u64; 12]>, keep_pop: bool) -> u64 {
	let mut old_vec: Vec<&[u64; 12]> = v_inp.iter().collect();

	for i in 0..12 {
		let most_pop = get_most_popular_bit(&old_vec, i);
		let to_keep = {
			if !keep_pop {
				if most_pop == 1 {
					0
				} else {
					1
				}
			} else {
				most_pop
			}
		};

		old_vec = old_vec
			.iter()
			.filter_map(|v| {
				if v[i as usize] == to_keep {
					Some(*v)
				} else {
					None
				}
			})
			.collect();

		if old_vec.len() == 1 {
			break;
		}
	}

	array_to_number(&old_vec[0])
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let v_inp = parse_input()?;

	let a = search_for(&v_inp, true);
	let b = search_for(&v_inp, false);

	Ok(a * b)
}
