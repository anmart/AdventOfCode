const WIDTH:usize = 10;
const HEIGHT:usize = 10;

pub fn get_adjacent_positions(r:usize, c:usize) -> Vec<(usize,usize)>{
	let mut ret: Vec<(usize,usize)> = vec!();
	if c > 0{
		ret.push((r,c-1));
		if r > 0{
			ret.push((r-1,c-1));
		}
		if r < HEIGHT-1{
			ret.push((r+1,c-1));
		}
	}


	if c < WIDTH-1 {
		ret.push((r,c+1));
		if r > 0{
			ret.push((r-1,c+1));
		}
		if r < HEIGHT-1{
			ret.push((r+1,c+1));
		}
	}


	if r > 0{
		ret.push((r-1,c));
	}
	if r < HEIGHT-1{
		ret.push((r+1,c));
	}

	return ret;
}

pub fn octo_step(octs: &mut Vec<Vec<u8>>, r:usize, c:usize, flashed: &mut Vec<(usize,usize)>) -> i32{
	let mut ret = 0;
	octs[r][c] += 1;
	if octs[r][c] > 9{
		if !flashed.contains(&(r,c)){
			// FLASH!!!
			ret +=1;
			flashed.push((r,c));
			for (r_,c_) in get_adjacent_positions(r,c){
				ret += octo_step(octs,r_,c_, flashed);
			}
		}

	}
	return ret;
}

#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut octs: Vec<Vec<u8>> = vec!();
	for line in lines.lines(){
		octs.push(line.as_bytes().iter().map(|&x| x - b'0').collect());
	}

	let mut ret = 0;
	for _ in 0..100{
		let mut flashed:Vec<(usize,usize)> = vec!();
		for r in 0..HEIGHT{
			for c in 0..WIDTH{
				ret += octo_step(&mut octs,r,c, &mut flashed);
			}
		}
		for (r,c) in flashed{
			octs[r][c] = 0;
		}
	}
	return ret.to_string();

}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut octs: Vec<Vec<u8>> = vec!();
	for line in lines.lines(){
		octs.push(line.as_bytes().iter().map(|&x| x - b'0').collect());
	}

	for i in 0..1000{
		let mut flashed:Vec<(usize,usize)> = vec!();
		for r in 0..HEIGHT{
			for c in 0..WIDTH{
				octo_step(&mut octs,r,c, &mut flashed);
			}
		}

		if flashed.len() == WIDTH*HEIGHT{
			return ((i+1) as i32).to_string();
		}

		for (r,c) in flashed{
			octs[r][c] = 0;
		}
	}
	return String::from("Error, not found.");


}

