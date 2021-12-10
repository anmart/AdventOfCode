pub fn is_smallest(inp: &Vec<Vec<u8>>, y:usize, x:usize) -> bool{	
	let width = inp[0].len();
	let height = inp.len();
	let val = inp[y][x];
	if y != 0 {
		if inp[y-1][x] <= val{
			return false;
		}
	}
	if y != height-1 {
		if inp[y+1][x] <= val{
			return false;
		}
	}
	if x != 0 {
		if inp[y][x-1] <= val{
			return false;
		}
	}
	if x != width-1 {
		if inp[y][x+1] <= val{
		return false;
		}
	}
	return true;
}

#[allow(dead_code)]
pub fn part1(lines: String) -> i32{
	let mut rows: Vec<Vec<u8>> = vec!();
	for line in lines.lines(){
		rows.push(line.as_bytes().iter().map(|&x| x - b'0').collect());
	}
	let mut risk: i32 = 0;
	for j in 0..rows.len(){
		for i in 0..rows[j].len(){
			if is_smallest(&rows,j,i){
				risk += 1 + (rows[j][i] as i32);
			}
		}
	}
	return risk;
}

#[allow(dead_code)]
pub fn part2(lines: String) -> i32{
	let mut rows: Vec<Vec<u8>> = vec!();
	for line in lines.lines(){
		rows.push(line.as_bytes().iter().map(|&x| x - b'0').collect());
	}

	let width = rows[0].len();
	let height = rows.len();

	let mut basins: Vec<i32> = vec!();
	for j in 0..rows.len(){
		for i in 0..rows[j].len(){
			if is_smallest(&rows,j,i){
				let mut open: Vec<(_,_)> = vec!((j,i));
				let mut closed: Vec<(usize,usize)> = vec!();
				let mut size: i32 = 0;
				while open.len() > 0{
					let (y,x) = open.pop().unwrap();
					//println!("looking at ({},{})", x,y);
					size += 1;

					if y != 0 && rows[y-1][x] != 9{
						let t = &(y-1,x);
						if !open.contains(t) && !closed.contains(t){
							//println!("was ({},{}) already in open or already in closed?", x,y);
							open.push(*t);
						}
					}
					if y != height-1 && rows[y+1][x] != 9{
						let t = &(y+1,x);
						if !open.contains(t) && !closed.contains(t){
							//println!("was ({},{}) already in open or already in closed?", x,y+1);
							open.push(*t);
						}
					}
					if x != 0 && rows[y][x-1] != 9{
						let t = &(y,x-1);
						if !open.contains(t) && !closed.contains(t){
							//println!("was ({},{}) already in open or already in closed?", x-1,y);
							open.push(*t);
						}
					}
					if x != width-1 && rows[y][x+1] != 9{
						let t = &(y,x+1);
						if !open.contains(t) && !closed.contains(t){
							//println!("was ({},{}) already in open or already in closed?", x+1,y);
							open.push(*t);
						}
					}

					closed.push((y,x));
				}
				//println!("{}",size);
				basins.push(size);
			}
		}
	}

	basins.sort();
	return basins.iter().rev().take(3).fold(1,|x,a| x*a);
}

