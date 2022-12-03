#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut val = 0;
	for line in lines.lines(){
		// fast to write no lookup lol
		let halflen: usize = line.chars().count() / 2;
		let mut sub = "".to_string();
		let mut kill_list = "".to_string();
		for (i,c) in line.chars().enumerate(){
			if i < halflen {
				sub.push(c);
			} else if sub.contains(c) && !kill_list.contains(c){
				if c.is_lowercase() {
					val += c as u32 - 96;
				} else {
					val += c as u32 - 38;
				}
				kill_list.push(c);
			}
		}
	}
	return val.to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{ 
	let mut val = 0;
	let mut iter = lines.lines().into_iter().peekable();
	while iter.peek().is_some(){
		let a = iter.next().unwrap();
		let b = iter.next().unwrap();
		for c in iter.next().unwrap().chars(){

			if a.contains(c) && b.contains(c){
				val += if c.is_lowercase() {
					c as u32 - 96
				} else {
					c as u32 - 38
				};
				break;
			}
		}
	}

	return val.to_string();
}

