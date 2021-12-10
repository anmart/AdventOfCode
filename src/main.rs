mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
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
