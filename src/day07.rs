#[allow(dead_code)]
pub fn part1(lines: String) -> i32{
	let mut crab_rave:Vec<_> = lines.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

	let max_val = *crab_rave.iter().max().unwrap();

	let mut all_shuffles: Vec<i32> = Vec::new();
	for i in 0..max_val{
		all_shuffles.push(crab_rave.iter_mut().map(|x| (*x-i).abs()).sum());
	}

	return *all_shuffles.iter().min().unwrap();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> i32{
	let mut crab_rave:Vec<_> = lines.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect();

	let max_val = *crab_rave.iter().max().unwrap();

	let mut all_shuffles: Vec<i32> = Vec::new();
	for i in 0..max_val{
		all_shuffles.push(crab_rave.iter_mut().map(|x| {
			let b = (*x-i).abs() as f32;
			((b+1f32)/2f32 * b) as i32
		}).sum());
	}

	return *all_shuffles.iter().min().unwrap();
}

