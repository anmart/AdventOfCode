#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut points: Vec<(u16,u16)> = vec!();
	let mut crossed_space = false;
	for line in lines.lines(){
		if line == ""{
			crossed_space = true;
			continue;
		}

		if !crossed_space{
			let sep = line.split(",").map(|s| s.parse::<u16>().unwrap()).collect::<Vec<u16>>();
			points.push((sep[0],sep[1]));
		}else{
			// parse just first fold
			let mod_line = line.split("=").collect::<Vec<&str>>();
			let val = mod_line[1].parse::<u16>().unwrap();
			let horizontal = if mod_line[0].chars().last().unwrap() == 'x' {true} else {false};
			points = points.iter().map(|x| {
				let (mut new_x,mut new_y) = x;
				if horizontal{
					if new_x > val{
						new_x =new_x - (2 * (new_x - val));
					}
				}else{
					if new_y > val{
						new_y = new_y - (2 * (new_y - val));
					}
				}
				(new_x,new_y)
			}).collect::<Vec<(u16,u16)>>();
			break;
		}
	}
	let mut used_points: Vec<(u16,u16)> = vec!();
	for z in points{
		if !used_points.contains(&z){
			used_points.push(z);
		}
	}
	return used_points.len().to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut points: Vec<(u16,u16)> = vec!();
	let mut crossed_space = false;
	for line in lines.lines(){
		if line == ""{
			crossed_space = true;
			continue;
		}

		if !crossed_space{
			let sep = line.split(",").map(|s| s.parse::<u16>().unwrap()).collect::<Vec<u16>>();
			points.push((sep[0],sep[1]));
		}else{
			let mod_line = line.split("=").collect::<Vec<&str>>();
			let val = mod_line[1].parse::<u16>().unwrap();
			let horizontal = if mod_line[0].chars().last().unwrap() == 'x' {true} else {false};
			points = points.iter().map(|x| {
				let (mut new_x,mut new_y) = x;
				if horizontal{
					if new_x > val{
						new_x =new_x - (2 * (new_x - val));
					}
				}else{
					if new_y > val{
						new_y = new_y - (2 * (new_y - val));
					}
				}
				(new_x,new_y)
			}).collect::<Vec<(u16,u16)>>();
		}
	}
	let mut ret = String::from("\n");
	for i in 0..6{
		for j in 0..40{
			if points.contains(&(j,i)){
				ret.push_str("#");
			}else{
				ret.push_str(" ");
			}

		}
		ret.push_str("\n");
	}
	return ret;
}

