use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut on_list: HashMap<(i8,i8,i8), bool> = HashMap::new();

	for l in lines.lines(){
		let set = if l.split(" ").collect::<Vec<&str>>()[0].len() == 2 { true } else { false };
		let mut inputs: Vec<i8> = vec!();
		for ll in l.split(",").collect::<Vec<&str>>(){
			let lll: Vec<&str> = ll.split("..").collect();
			let upper = lll[1].parse::<i8>().unwrap_or(-69);
			let lower = lll[0].split("=").collect::<Vec<&str>>()[1].parse::<i8>().unwrap_or(-69);
			if upper < -50 || upper > 50 || lower < -50 || lower > 50{
				break;
			}
			inputs.push(lower);
			inputs.push(upper);
		}
		if inputs.len() < 6{
			println!("??? how {:?}", l);
			continue;
		}
		println!("{:?}: {:?}", set, inputs);
		let xlo = inputs[0];
		let xhi = inputs[1];
		let ylo = inputs[2];
		let yhi = inputs[3];
		let zlo = inputs[4];
		let zhi = inputs[5];
		for x in xlo..=xhi{
			for y in ylo..=yhi{
				for z in zlo..=zhi{
					on_list.remove(&(x,y,z));
					if set{
						on_list.insert((x,y,z), true);
					}
				}
			}
		}
	}
	return on_list.len().to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut on_list: HashMap<(i32,i32,i32), bool> = HashMap::new();

	for l in lines.lines(){
		let set = if l.split(" ").collect::<Vec<&str>>()[0].len() == 2 { true } else { false };
		let mut inputs: Vec<i32> = vec!();
		for ll in l.split(",").collect::<Vec<&str>>(){
			let lll: Vec<&str> = ll.split("..").collect();
			let upper = lll[1].parse::<i32>().unwrap();
			let lower = lll[0].split("=").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
			inputs.push(lower);
			inputs.push(upper);
		}
		if inputs.len() < 6{
			println!("??? how {:?}", l);
			continue;
		}
		println!("{:?}: {:?}", set, inputs);
		let xlo = inputs[0];
		let xhi = inputs[1];
		let ylo = inputs[2];
		let yhi = inputs[3];
		let zlo = inputs[4];
		let zhi = inputs[5];
		for x in xlo..=xhi{
			for y in ylo..=yhi{
				for z in zlo..=zhi{
					on_list.remove(&(x,y,z));
					if set{
						on_list.insert((x,y,z), true);
					}
				}
			}
		}
	}
	return on_list.len().to_string();
}

