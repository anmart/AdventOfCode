/*enum cave_type{
	Start,
	End,
	Small,
	Big,
}

struct cave {
	ctype: cave_type,
	outs: Vec<&cave>,
}*/

#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let lines_strings:Vec<&str> = lines.lines().collect();


	let mut paths = vec!(vec!("start"));
	let mut tot:i32 = 0;
	'outer: loop{
		//println!("begining of loop: {:?}", paths);
		//let paths_to_add: Vec<Vec<&str>> = vec!();
		if paths.len() < 1{
			break 'outer;
		}
		let j = paths.pop().unwrap();
		//let paths_to_add: Vec<Vec<&str>> = vec!();
		let lines_strings_copy = lines_strings.to_vec();
		//println!("STRING: {:?}", lines_strings_copy);
		let new_caves_set = lines_strings_copy.into_iter().filter(|&x| {
			let z = x.split("-").collect::<Vec<&str>>(); 
			(z[0] ==  j[j.len()-1] && (z[1].to_uppercase() == z[1] || !j.contains(&z[1])) && z[1] != "start" && z[1] != "end") ||
				((z[1] ==  j[j.len()-1] && (z[0].to_uppercase() == z[0] || !j.contains(&z[0]))) && z[0] != "start" && z[0] != "end")
		}).collect::<Vec<&str>>();

		//println!("starting: {:?}\npossibles: {:?}", j, new_caves_set);
		if new_caves_set.len() < 1{
			tot +=1;
			println!("finished {:?}", j);
			if paths.len() < 1 && j[j.len()-1] == "end"{
				break 'outer;
			}
		} else{
			for c in new_caves_set{
				let dumb = c.split("-").collect::<Vec<&str>>();
				let dumbest = if dumb[0] == j[j.len()-1] {dumb[1]} else {dumb[0]};
				let mut new_j = j.to_vec();
				new_j.push(dumbest);
				paths.push(new_j);
			}
		}
		println!("big ending: {:?}", paths);

	}
	return tot.to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	return lines;
}

