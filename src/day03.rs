#[allow(dead_code)]
pub fn part1(lines: String) -> i32{
	let mut commonness: [i32; 12] = [0;12];
	for line in lines.lines() {
		for (i, c) in line.chars().enumerate(){
			commonness[i] += if c == '1' {1} else {-1};
		}
	}
	let mut gamma = 0;
	let mut epsilon = 0;
	for v in commonness {
		gamma *= 2;
		epsilon *= 2;
		if v > 0 {
			gamma += 1;
		} else {
			epsilon += 1;
		}
	}
	return gamma * epsilon;
}

#[allow(dead_code)]
pub fn part2(lines: String) -> i32{
	
	let mut commonness_oxygen: [i32; 12] = [0;12];
	let mut commonness_co2: [i32; 12] = [0;12];
	let mut all_values: [u16; 1000] = [1;1000];

	for (i,line) in lines.lines().enumerate() {
		all_values[i] = super::helper::bit_string_to_u16(line);
	}

	let mut filter_oxygen: u16 = 0;
	let mut filter_co2: u16 = 0;
	let mut found_final_co2 = false;
	for i in 0..12 {
		let shift = 12 - i;
		let mut last_found_co2 = 0;
		let mut times_found_co2 = 0;
		for val in all_values{
			let current_bit = if (1 & (val >> shift-1)) == 1 {1} else {-1};
			if val >> shift == filter_oxygen >> shift{
				// narrowing down to 1 left will give correct result regardless
				commonness_oxygen[i] += current_bit;
			}
			if val >> shift == filter_co2 >> shift{
				last_found_co2 = val;
				times_found_co2 += 1;
				commonness_co2[i] += current_bit;
			}

		}

		// we've found the final item
		if times_found_co2 == 1{
			filter_co2 = last_found_co2;
			found_final_co2 = true;
		}

		if commonness_oxygen[i] >= 0{
			filter_oxygen += 1 << shift-1;
		}
		if !found_final_co2 && commonness_co2[i] < 0{
			filter_co2 += 1 << shift-1;
		}
	}

	return (filter_oxygen as i32) * (filter_co2 as i32);
}
