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
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod helper;

use std::time::Instant;
use std::process::exit;
use helper::read_file_to_string;

fn main() {	

	// get current time for processing benchmarking
	let benchmark_total = Instant::now();

	// get args
	let args: Vec<String> = std::env::args().collect();

	// check for and remove `-` string_args
	let (string_args,dash_args):(Vec<&String>,Vec<&String>) = args.iter().partition(|&x| !x.contains("-"));
	let flag_vocal = dash_args.contains(&&String::from("-v"));

	// print usage if wrong number of string args
	if string_args.len() < 3 || string_args.len() > 4{
		eprintln!("ERROR: usage is AdventOfCode2021 [dash_args] day part [data_mod]\n\ndash args:\n\t-v\tPrint extra benchmark info");
		exit(2);
	}

	// Verify Day and Part
	let day = match string_args[1].parse::<u8>(){
		Err(_) => {eprintln!("ERROR: arg `day` could not be parsed as u8, you put: {:?}", string_args[1] ); exit(2)},
		Ok(f) => f,
	};
	let part = match string_args[2].parse::<u8>() {
		Err(_) => {eprintln!("ERROR: arg `part` could not be parsed as u8, you put: {:?}", string_args[2] ); exit(2)},
		Ok(f) => f,
	};
	if part != 1 && part != 2{
		eprintln!("Error, part must be 1 or 2");
		exit(2);
	}

	// handle data file name and modifiers, such as _test, _test_test, etc
	let mut data_file_name = String::from("./data/day");
	if day < 10{
		data_file_name.push_str("0"); // maybe not the best way, but add 0 to days 1-9
	}
	data_file_name.push_str(&string_args[1]); // skip the cast, reuse day argument string
	if string_args.len() == 4{
		data_file_name.push_str(&string_args[3]);
	}

	// get current time for processing benchmarking
	let benchmark_data = Instant::now();

	// read data
	let data = match read_file_to_string(&data_file_name){
		Err(_) => {eprintln!("Error reading the following: {:?}", data_file_name); exit(2)},
		Ok(f) => f
	};

	// print data benchmark results
	if flag_vocal{
		println!("Elapsed data I/O time: {:?}", benchmark_data.elapsed());
	}

	// get current time for processing benchmarking
	let benchmark_processing = Instant::now();

	// run day::part
	let result_string = match day{
		1  => if part == 1 { day01::part1(data) } else { day01::part2(data) },
		2  => if part == 1 { day02::part1(data) } else { day02::part2(data) },
		3  => if part == 1 { day03::part1(data) } else { day03::part2(data) },
		4  => if part == 1 { day04::part1(data) } else { day04::part2(data) },
		5  => if part == 1 { day05::part1(data) } else { day05::part2(data) },
		6  => if part == 1 { day06::part1(data) } else { day06::part2(data) },
		7  => if part == 1 { day07::part1(data) } else { day07::part2(data) },
		8  => if part == 1 { day08::part1(data) } else { day08::part2(data) },
		9  => if part == 1 { day09::part1(data) } else { day09::part2(data) },
		10 => if part == 1 { day10::part1(data) } else { day10::part2(data) },
		11 => if part == 1 { day11::part1(data) } else { day11::part2(data) },
		12 => if part == 1 { day12::part1(data) } else { day12::part2(data) },
		13 => if part == 1 { day13::part1(data) } else { day13::part2(data) },
		14 => if part == 1 { day14::part1(data) } else { day14::part2(data) },
		15 => if part == 1 { day15::part1(data) } else { day15::part2(data) },
		16 => if part == 1 { day16::part1(data) } else { day16::part2(data) },
		17 => if part == 1 { day17::part1(data) } else { day17::part2(data) },
		18 => if part == 1 { day18::part1(data) } else { day18::part2(data) },
		19 => if part == 1 { day19::part1(data) } else { day19::part2(data) },
		20 => if part == 1 { day20::part1(data) } else { day20::part2(data) },
		21 => if part == 1 { day21::part1(data) } else { day21::part2(data) },
		22 => if part == 1 { day22::part1(data) } else { day22::part2(data) },
		23 => if part == 1 { day23::part1(data) } else { day23::part2(data) },
		24 => if part == 1 { day24::part1(data) } else { day24::part2(data) },
		25 => if part == 1 { day25::part1(data) } else { day25::part2(data) },
		_ =>String::from("Error: Day not found"),
		};


	// print processing benchmark results
	println!("Elapsed processing time: {:?}", benchmark_processing.elapsed());

	// print TOTAL benchmark results
	if flag_vocal{
		println!("Total elapsed time, including args: {:?}", benchmark_total.elapsed());
	}

	println!("\nOutput for Day {} Part {} is: {}", day, part, result_string);
}
