struct Location1 {
	hor: i32,
	depth: i32,
}

#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut loc = Location1{hor: 0, depth: 0};
	for line in lines.lines() {
		let str_len = line.len();
		let current_number = &line[str_len-1..].parse::<i32>().unwrap();
		if line.contains("forward"){
			loc.hor += current_number;
		} else if line.contains("up"){
			loc.depth -= current_number;
		} else if line.contains("down"){
			loc.depth += current_number;
		}
	}
	return (loc.hor * loc.depth).to_string();
}

struct Location2 {
	hor: i32,
	depth: i32,
	aim: i32,
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut loc = Location2{hor: 0, depth: 0, aim: 0};
	for line in lines.lines() {
		let str_len = line.len();
		let current_number = &line[str_len-1..].parse::<i32>().unwrap();
		if line.contains("forward"){
			loc.hor += current_number;
			loc.depth += loc.aim * current_number;
		} else if line.contains("up"){
			loc.aim -= current_number;
		} else if line.contains("down"){
			loc.aim += current_number;
		}
	}
	return (loc.hor * loc.depth).to_string();
}
