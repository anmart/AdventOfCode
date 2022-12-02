
#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut score = 0;
	for line in lines.lines(){
		let mut line = line.chars();
		let opp = match line.next().unwrap(){
			'A' => 0,
			'B' => 1,
			'C' => 2,
			_ => std::process::exit(1),
		};
		line.next();
		let me = match line.next().unwrap(){
			'X' => 1,
			'Y' => 2,
			'Z' => 3,
			_ => std::process::exit(1),
		};
		score += me;
		if me == opp + 1 { 
			score += 3;	// tie
		// lord help me i wanna be fast
		} else if opp == 0 && me ==  2 || opp == 1 && me == 3 || opp == 2 && me == 1 {
			score += 6; // win
		}

	}
	return score.to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut score = 0;
	for line in lines.lines(){
		let mut line = line.chars();
		let opp = match line.next().unwrap(){
			'A' => 0,
			'B' => 1,
			'C' => 2,
			_ => std::process::exit(1),
		};
		line.next();
		let me = match line.next().unwrap(){
			'X' => (opp + 2) % 3 + 1,
			'Y' => opp + 1,
			'Z' => (opp + 1)%3 + 1,
			_ => std::process::exit(1),
		};
		score += me;
		if me == opp + 1 { 
			score += 3;	// tie
		// lord help me i wanna be fast
		} else if opp == 0 && me ==  2 || opp == 1 && me == 3 || opp == 2 && me == 1 {
			score += 6; // win
		}

	}
	return score.to_string();
}

