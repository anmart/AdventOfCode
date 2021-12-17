
#[allow(dead_code)]
pub fn part1(lines: String) -> String{

	// this is bad lol
	let full_split = lines.split("=").collect::<Vec<&str>>();
	let x_split = full_split[1].split("..").collect::<Vec<&str>>();
	let min_x = String::from(x_split[0]).parse().unwrap();
	let max_x = String::from(x_split[1].split(", ").collect::<Vec<&str>>()[0]).parse().unwrap();

	let y_split = full_split[2].split("..").collect::<Vec<&str>>();
	let min_y = String::from(y_split[0]).parse().unwrap();
	let max_y = String::from(y_split[1]).parse().unwrap();
	println!("{:?} {:?} {:?} {:?}", min_x, min_y, max_x, max_y);

	let mut highest_y = 0;
	for x in 0..5000{
		for y in 0..5000{
			let mut x_vel = x;
			let mut y_vel = y;
			let mut x_pos = 0;
			let mut y_pos = 0;
			let mut highest_achieved = 0;
			while x_pos < max_x && y_pos > min_y{
				y_pos += y_vel;
				x_pos += x_vel;
				y_vel -= 1;
				if x_vel > 0{
					x_vel -= 1;
				}else if x_vel < 0{
					x_vel += 1;
				}
				if y_pos > highest_achieved{
					highest_achieved = y_pos;
				}
				if x_pos >= min_x && y_pos <= max_y && y_pos >= min_y && x_pos <= max_x{

					println!("hit! with: {:?},{:?},{:?},{:?},{:?}",x,y,x_pos,y_pos,highest_achieved);
					if highest_y < highest_achieved{
						highest_y = highest_achieved;
					}
					break;
				}
			}
		}
	}
	return highest_y.to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	
	// this is bad lol
	let full_split = lines.split("=").collect::<Vec<&str>>();
	let x_split = full_split[1].split("..").collect::<Vec<&str>>();
	let min_x = String::from(x_split[0]).parse().unwrap();
	let max_x = String::from(x_split[1].split(", ").collect::<Vec<&str>>()[0]).parse().unwrap();

	let y_split = full_split[2].split("..").collect::<Vec<&str>>();
	let min_y = String::from(y_split[0]).parse().unwrap();
	let max_y = String::from(y_split[1]).parse().unwrap();

	let mut hit_amt = 0;
	for x in 0..2000{
		for y in -1000..1000{
			let mut x_vel = x;
			let mut y_vel = y;
			let mut x_pos = 0;
			let mut y_pos = 0;
			let mut highest_achieved = 0;
			while x_pos < max_x && y_pos > min_y{
				y_pos += y_vel;
				x_pos += x_vel;
				y_vel -= 1;
				if x_vel > 0{
					x_vel -= 1;
				}else if x_vel < 0{
					x_vel += 1;
				}
				if y_pos > highest_achieved{
					highest_achieved = y_pos;
				}
				if x_pos >= min_x && y_pos <= max_y && y_pos >= min_y && x_pos <= max_x{
					hit_amt += 1;
					break;
				}
			}
		}
	}
	return hit_amt.to_string();
}

