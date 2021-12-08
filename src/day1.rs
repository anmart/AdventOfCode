#[allow(dead_code)]
pub fn part1(lines: String) -> i32{
	let mut increases = 0;
	let mut last_num = 99999;
	for line in lines.lines() {
		let current_number = line.parse::<i32>().unwrap();
		if current_number > last_num {
			increases += 1;
		}
		last_num = current_number;
	}
	return increases;
}

const INPUT_VALUE_AMOUNT: usize = 2000;
#[allow(dead_code)]
pub fn part2(lines: String) -> i32{
	let mut numbers: [i32; 2000] = [0;2000];	
	for (i, line) in lines.lines().enumerate() {
		numbers[i] = line.parse::<i32>().unwrap();
	}
	let mut increases = 0;
	for n in 0..(INPUT_VALUE_AMOUNT-3) {
		let current = numbers[n] + numbers[n+1] + numbers[n+2];
		let next = numbers[n+1] + numbers[n+2] + numbers[n+3];
		if next > current{
			increases+=1;
		}
	}
	return increases;
}
