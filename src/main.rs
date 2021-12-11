use std::time::Instant;

mod day_11;

fn main() {
	let now = Instant::now();
	println!("Result of part A: {:?} in {} μs", day_11::run_a(), now.elapsed().as_micros());
	let now = Instant::now();
	println!("Result of part B: {:?} in {} μs", day_11::run_b(), now.elapsed().as_micros());
}
