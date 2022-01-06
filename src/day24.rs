pub fn get_var_index(inp: &str) -> usize{
	return match inp{
		"w" => 0,
		"x" => 1,
		"y" => 2,
		"z" => 3,
		e => panic!("Bad variable: {:?}", e),
	}

}
pub fn get_val(vars: &Vec<i64>, inp: &str) -> i64{
	//println!("{:?}", inp);
	return match inp{
		"w" => vars[0],
		"x" => vars[1],
		"y" => vars[2],
		"z" => vars[3],
		e => e.parse::<i64>().unwrap(),
	}
}
pub fn part1(_: String) -> String{
	let m:[[i64;14];3] = [[1, 1, 1, 1, 1, 26, 26, 1, 26, 1, 26, 26, 26, 26],[14, 13, 12, 14, 13, -7, -13, 10, -7, 11, -9, -2, -9, -14],[12, 6, 4, 5, 0, 4, 15, 14, 6, 14, 8, 5, 14, 4]];
	for mut inp in (11111111111111u64..=99999999999999u64).rev(){
		if inp % 100000000 == 0{
			println!("We hit: {:?}", inp);
		}
		let mut inps: Vec<u8> = vec!();
		for _ in 0..14{
			inps.push((inp % 10) as u8);
			inp /= 10;
		}
		if inps.contains(&0u8){
			continue;
		}

		let mut w: i64;
		let mut x: i64;
//		let mut y: i32 = 0;
		let mut z: i64 = 0;
		for i in 0..=13{
			w = inps.pop().unwrap() as i64;
			x = if (( z % 26 ) + m[1][i]) == w { 0 } else { 1 };
			z /= m[0][i];
			z *= (25 * x) + 1;
			z +=(w + m[2][i])*x;

		}
		if z == 0{
			println!("FOUND IT!!!!! {:?}", inp);
			break;
		}
	}


	return String::from("");
}

#[allow(dead_code)]
pub fn part_1(lines: String) -> String{
	let program: Vec<Vec<&str>> = lines.lines().collect::<Vec<&str>>()
									.iter().map(|s| s.split(" ")
									.collect::<Vec<&str>>()).collect();
	for mut inp in (11111111111111u64..=99999999999999u64).rev(){
		let mut inps: Vec<u8> = vec!();
		for _ in 0..14{
			inps.push((inp % 10) as u8);
			inp /= 10;
		}
		inps = inps.into_iter().rev().collect();
		if inps.contains(&0u8){
			continue;
		}

		let mut vars: Vec<i64> = vec!(0,0,0,0); // w, x, y, z
		// run the program! 
		for l in program.iter(){
			//println!("program so far: {:?}", vars);
			match l[0]{
				"inp" => {
					println!();
					vars[get_var_index(l[1])] = inps.pop().unwrap() as i64;
				},
				"add" => {
					print!("{:?}, ", l[2]);
					let add_val = get_val(&vars,l[1]) + get_val(&vars, l[2]);
					vars[get_var_index(l[1])] = add_val;
				},
				"mul" => {
					print!("{:?}, ", l[2]);
					let mul_val = get_val(&vars,l[1]) * get_val(&vars, l[2]);
					vars[get_var_index(l[1])] = mul_val;
				},
				"div" => {
					print!("{:?}, ", l[2]);
					let div_val = get_val(&vars,l[1]) / get_val(&vars, l[2]);
					vars[get_var_index(l[1])] = div_val;
				},
				"mod" => {
					print!("{:?}, ", l[2]);
					let mod_val = get_val(&vars,l[1]) % get_val(&vars, l[2]);
					vars[get_var_index(l[1])] = mod_val;
				},
				"eql" => {
					print!("{:?}, ", l[2]);
					let eq_val = get_val(&vars,l[1]) == get_val(&vars, l[2]);
					vars[get_var_index(l[1])] = if eq_val { 1 } else { 0 };
				},
				e => panic!("Bad instruction, {:?}", e),
			}
		}
		if vars[3] == 0{
			println!("FOUND IT AT: {:?}", inp);
			break;
		}

		break;

	}

	return String::from("");
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	return lines;
}

