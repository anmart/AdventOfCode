#[allow(dead_code)]
pub fn part1(lines: String) -> i32{
	let mut tot = 0;
	for line in lines.lines(){
		let mut parse = vec!();
		for a in line.chars(){
			match a {
				'['|'('|'<'|'{' => parse.push(a),
				'}' => if parse.pop().unwrap() != '{' {tot += 1197},
				']' => if parse.pop().unwrap() != '[' {tot += 57},
				')' => if parse.pop().unwrap() != '(' {tot += 3},
				'>' => if parse.pop().unwrap() != '<' {tot += 25137},
				_ => println!("???"),
			}
		}
	}
	return tot;
}

#[allow(dead_code)]
pub fn part2(lines: String) -> u64{
	let mut woog = vec!();
	'outer: for line in lines.lines(){
		let mut parse = vec!();
		for a in line.chars(){
			match a {
				'['|'('|'<'|'{' => parse.push(a),
				'}' => if parse.pop().unwrap() != '{' {continue 'outer},
				']' => if parse.pop().unwrap() != '[' {continue 'outer},
				')' => if parse.pop().unwrap() != '(' {continue 'outer},
				'>' => if parse.pop().unwrap() != '<' {continue 'outer},
				_ => println!("???"),
			}

		}
		
		let mut score: u64 = 0;
		for c in parse.iter().rev(){
			score *= 5;
			score += match c{
				'(' => 1,
				'[' => 2,
				'{' => 3,
				'<' => 4,
				_ => 0,
			}
		}
		woog.push(score);
	}
	let z = woog.len() as f32 / 2.0;
	woog.sort();
	return woog[z as usize];
}

