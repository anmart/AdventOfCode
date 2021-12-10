mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod helper;

use std::time::Instant;
use helper::read_file_to_string;

fn main() {
	let _before_io = Instant::now();

	// read data
	let data = read_file_to_string("./data/day10").unwrap();

	// get current time for benchmarking
	let _before_run = Instant::now();

	println!("{}", day10::part2(data));

	// print benchmark results
	println!("Elapsed time: {:?}", _before_run.elapsed());
}
