use std::cmp::{min,max};

#[allow(dead_code)]
pub fn part1(lines: String) -> i32{
	let mut board: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
	for line in lines.lines() {
		let vals: Vec<usize> = line
			.split(" -> ")
			.map(|l| l.split(',').collect())
			.collect::<Vec<Vec<&str>>>()
			.into_iter()
			.flatten()
			.map(|v| v.parse::<usize>().unwrap())
			.collect();
		if vals[0] == vals[2]{
			// vertical
			for i in min(vals[1],vals[3])..=max(vals[1],vals[3]){
				board[vals[0]][i] += 1;
			}
		} else if vals[1] == vals[3]{
			// horizontal
			for i in min(vals[0],vals[2])..=max(vals[0],vals[2]){
				board[i][vals[1]] += 1;
			}
		} else {
			// we ignore diagonals rn
		}
		
	}

	let mut result = 0;
	for c in 0..1000{
		for r in 0..1000{
			if board[c][r] > 1{
				result+=1;
			}
		}
	}
	return result;
}

#[allow(dead_code)]
pub fn part2(lines: String) -> i32{
	let mut board: [[u8; 1000]; 1000] = [[0; 1000]; 1000];
	for line in lines.lines() {
		let vals: Vec<usize> = line
			.split(" -> ")
			.map(|l| l.split(',').collect())
			.collect::<Vec<Vec<&str>>>()
			.into_iter()
			.flatten()
			.map(|v| v.parse::<usize>().unwrap())
			.collect();
		if vals[0] == vals[2]{
			// vertical
			for i in min(vals[1],vals[3])..=max(vals[1],vals[3]){
				board[vals[0]][i] += 1;
			}
		} else if vals[1] == vals[3]{
			// horizontal
			for i in min(vals[0],vals[2])..=max(vals[0],vals[2]){
				board[i][vals[1]] += 1;
			}
		} else {
			//y = mx + b
			// step = y2 - y1, flip if x2 > x1
			let x_step: i16 = if vals[0] < vals[2] {1} else {-1};
			let y_step: i16 = if vals[1] < vals[3] {1} else {-1};
			let range = max(vals[2],vals[0]) - min(vals[2],vals[0]);
			for i in 0..= range as i16{
				let c = (vals[0] as i16) + i*x_step;
				let r = (vals[1] as i16) + i*y_step;
				board[c as usize][r as usize] += 1;
			}
		}
	}
	let mut result = 0;
	for c in 0..1000{
		for r in 0..1000{
			if board[c][r] > 1{
				result+=1;
			}
		}
	}
	return result;
}
