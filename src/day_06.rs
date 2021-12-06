use std::fs::File;
use std::io::Read;

fn parse_input() -> Result<Vec<u64>, std::io::Error> {
	let mut file = File::open("input_06.txt")?;
	let mut str_file = String::new();
	file.read_to_string(&mut str_file)?;
	let res = str_file.trim().split(',').map(|v| v.parse().unwrap()).collect();
	Ok(res)
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let mut fishes = parse_input()?;

	for _ in 0..80 {
		let mut new_fishes = Vec::new();
		for fish in &mut fishes {
			if *fish == 0 {
				*fish = 6;
				new_fishes.push(8);
			} else {
				*fish -= 1;
			}
		}
		fishes.append(&mut new_fishes);
	}

	Ok(fishes.len() as u64)
}

fn get_num_children(days: i64, timer: i64) -> i64 {
	let mut num_children = 0;
	let mut new_timer = timer;
	let mut new_days = days;

	while new_days - new_timer >= 0 {
		new_days -= new_timer;
		num_children += get_num_children(new_days, 9) + 1;
		new_timer = 7;
	}

	num_children
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let fishes = parse_input()?;
	let mut num_fishes = fishes.len() as i64;
	let mut vec_threads = Vec::new();
	for fish in &fishes {
		let fish_life = (*fish + 1) as i64;
		vec_threads.push(std::thread::spawn(move || get_num_children(256, fish_life)));
	}

	for t in vec_threads {
		num_fishes += t.join().unwrap();
	}

	Ok(num_fishes as u64)
}
