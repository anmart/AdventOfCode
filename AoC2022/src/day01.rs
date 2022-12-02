#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut highest_val = 0;
	let mut curr_val = 0;
	for line in lines.lines() {
		if line.len() <= 0 {
			if curr_val > highest_val {
				highest_val = curr_val;
			}
			curr_val = 0;
		} else {
			curr_val += line.parse::<i32>().unwrap();
		}

	}
	return highest_val.to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut calories = Vec::new();
	let mut curr_val = 0;
	for line in lines.lines() {
		if line.len() <= 0 {
			calories.push(curr_val);
			curr_val = 0;
		} else {
			curr_val += line.parse::<i32>().unwrap();
		}

	}
	calories.sort();
	return calories.iter().rev().take(3).sum::<i32>().to_string();
}

