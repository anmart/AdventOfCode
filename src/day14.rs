
#[allow(dead_code)]
pub fn part1_naive(lines: String) -> String{
	let mut line_iter = lines.lines();
	let mut chars:Vec<char> = line_iter.next().unwrap().chars().collect();
	line_iter.next(); // skip empty line;

	let rules: Vec<(char,char,char)> = line_iter.map(|s| {
		let s_split: Vec<&str> = s.split(" -> ").collect();
		let s_left:Vec<char> = s_split[0].chars().collect();
		(s_left[0],s_left[1],s_split[1].chars().last().unwrap())
	}).collect();

	for _ in 0..10{
		chars = chars.into_iter().rev().collect();
		let mut next_char = chars.pop().unwrap();
		let mut new_chars:Vec<char> = vec!(next_char);
		let mut char_added = false; //dumb
		while chars.len() > 0{
			char_added = false;
			let cur_char = next_char;
			next_char = chars.pop().unwrap();
			for r in &rules{
				let (a,b,c) = *r;
				if cur_char == a && next_char == b{
					char_added = true;
					new_chars.push(c);
					new_chars.push(b);
					break;
				}
			}
			if !char_added{
				new_chars.push(cur_char);
			}
		}
		if !char_added{
			new_chars.push(next_char);
		}
		chars = new_chars;
	}

	let mut letters: [u64;26] = [0;26];
	for c in chars{
		letters[((c as u8) - b'A') as usize] += 1;
	}

	let mut max = 0;
	let mut min = 10000;
	for l in letters{
		if l == 0{
			continue;
		}
		if l < min{
			min = l;
		}
		if l > max{
			max = l;
		}
	}
	
	return (max - min).to_string();
}

pub fn cindex ( c: char) -> usize{
	return ((c as u8) - b'A') as usize;
}

#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut line_iter = lines.lines();
	let mut chars = line_iter.next().unwrap().chars().collect::<Vec<char>>().into_iter().peekable();
	line_iter.next(); // skip empty line;
	let mut polys: [[u64;26];26] = [[0;26];26];
	while chars.len() > 1{
		polys[cindex(chars.next().unwrap())][cindex(*chars.peek().unwrap())] += 1;

	}
	let extra_char = cindex(chars.next().unwrap());

	let rules: Vec<(char,char,char)> = line_iter.map(|s| {
		let s_split: Vec<&str> = s.split(" -> ").collect();
		let s_left:Vec<char> = s_split[0].chars().collect();
		(s_left[0],s_left[1],s_split[1].chars().last().unwrap())
	}).collect();

	for _ in 0..10{
		let mut to_add: Vec<(usize,usize,u64)> = vec!();
		for (r_a,r_b,r_c) in &rules{
			let amt = polys[cindex(*r_a)][cindex(*r_b)];
			polys[cindex(*r_a)][cindex(*r_b)] = 0;
			to_add.push((cindex(*r_a), cindex(*r_c), amt));
			to_add.push((cindex(*r_c), cindex(*r_b), amt));
		}
		for (r_a, r_b, amt) in to_add{
			polys[r_a][r_b] += amt;
		}
	}

	let mut max = 0;
	let mut min = 0;
	for (i, c) in polys.into_iter().enumerate(){
		let mut tot: u64 = c.into_iter().sum();
		tot += if extra_char == i {1} else {0}; // account for final character in original
		if tot != 0 && tot < min || min == 0{
			min = tot;
		}
		if tot != 0 && tot > max {
			max = tot;
		}

	}
	return (max - min).to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut line_iter = lines.lines();
	let mut chars = line_iter.next().unwrap().chars().collect::<Vec<char>>().into_iter().peekable();
	line_iter.next(); // skip empty line;
	let mut polys: [[u64;26];26] = [[0;26];26];
	while chars.len() > 1{
		polys[cindex(chars.next().unwrap())][cindex(*chars.peek().unwrap())] += 1;

	}
	let extra_char = cindex(chars.next().unwrap());

	let rules: Vec<(char,char,char)> = line_iter.map(|s| {
		let s_split: Vec<&str> = s.split(" -> ").collect();
		let s_left:Vec<char> = s_split[0].chars().collect();
		(s_left[0],s_left[1],s_split[1].chars().last().unwrap())
	}).collect();

	for _ in 0..40{
		let mut to_add: Vec<(usize,usize,u64)> = vec!();
		for (r_a,r_b,r_c) in &rules{
			let amt = polys[cindex(*r_a)][cindex(*r_b)];
			polys[cindex(*r_a)][cindex(*r_b)] = 0;
			to_add.push((cindex(*r_a), cindex(*r_c), amt));
			to_add.push((cindex(*r_c), cindex(*r_b), amt));
		}
		for (r_a, r_b, amt) in to_add{
			polys[r_a][r_b] += amt;
		}
	}

	let mut max = 0;
	let mut min = 0;
	for (i, c) in polys.into_iter().enumerate(){
		let mut tot: u64 = c.into_iter().sum();
		tot += if extra_char == i {1} else {0}; // account for final character in original
		if tot != 0 && tot < min || min == 0{
			min = tot;
		}
		if tot != 0 && tot > max {
			max = tot;
		}

	}
	return (max - min).to_string();
}

