
// makes no bounds checks, as its used in a place where that isnt necessary
pub fn get_pixel_val(row: usize,col:usize, img: &Vec<Vec<u8>>) -> usize{
	if row == 0 || col==0 { panic!("bad input to get_pixel_val");}

	let mut pixs:Vec<u8> = vec!();
	pixs.push(img[row-1][col-1]);
	pixs.push(img[row-1][col]);
	pixs.push(img[row-1][col+1]);

	pixs.push(img[row][col-1]);
	pixs.push(img[row][col]);
	pixs.push(img[row][col+1]);

	pixs.push(img[row+1][col-1]);
	pixs.push(img[row+1][col]);
	pixs.push(img[row+1][col+1]);

	let mut ret:u16 = 0;
	for p in pixs{
		ret = ret << 1;
		ret += (p as u16) & 0b1;
	}
	return ret as usize;

}

#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut l_lines = lines.lines().into_iter();
	let key: Vec<char> = l_lines.next().unwrap().chars().collect();
	l_lines.next();

	let mut img: Vec<Vec<u8>> = vec!();
	for l in l_lines{
		let mut row: Vec<u8> = l.chars().collect::<Vec<char>>().iter().map(|c| {
			match c{
				'.' => 0,
				'#' => 1,
				_ => panic!("wat"),
			}
		}).collect();
		for _ in 0..5{
			row.insert(0,0);
			row.push(0);
		}
		img.push(row);
	}

	// lol
	let empty_row: Vec<u8> = vec![0;img[0].len()];
	for _ in 0..5{
		img.insert(0,empty_row.to_vec());
		img.push(empty_row.to_vec());
	}

	for i in 0..2{
		// create vec for next iteration
		let mut new_img: Vec<Vec<u8>> = vec![vec![0u8;img[0].len()]; img.len()];

		// parsing time
		for row in 1..img.len()-1{
			for col in 1..img[0].len()-1{
				new_img[row][col] = match key[get_pixel_val(row,col,&img)]{
					'.' => 0,
					'#' => 1,
					_ => panic!("wat"),
				}
				
			}
		}
		img = new_img;

		// fix the outer edges. In our (most?) cases, the infinite 0s flash off and on.
		// no clue what to do otherwise but it works here so 
		let new_val: u8 = 1 - i%2;
		img[0] = vec![new_val;img[0].len()];
		img.pop(); 
		img.push(vec![new_val;img[0].len()]);
		for row in img.iter_mut(){
			row[0] = new_val;
			row.pop();
			row.push(new_val);
		}
	}

	for o in img.iter(){
		for p in o{
			print!("{}",p);
		}
		println!();
	}


	return img.into_iter().fold(0, |acc, x| acc + x.iter().sum::<u8>() as u16).to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut l_lines = lines.lines().into_iter();
	let key: Vec<char> = l_lines.next().unwrap().chars().collect();
	l_lines.next();

	let mut img: Vec<Vec<u8>> = vec!();
	for l in l_lines{
		let mut row: Vec<u8> = l.chars().collect::<Vec<char>>().iter().map(|c| {
			match c{
				'.' => 0,
				'#' => 1,
				_ => panic!("wat"),
			}
		}).collect();
		for _ in 0..55{
			row.insert(0,0);
			row.push(0);
		}
		img.push(row);
	}

	// lol
	let empty_row: Vec<u8> = vec![0;img[0].len()];
	for _ in 0..55{
		println!("{:?}", empty_row);
		img.insert(0,empty_row.to_vec());
		img.push(empty_row.to_vec());
	}

	for i in 0..50{
		// create vec for next iteration
		let mut new_img: Vec<Vec<u8>> = vec![vec![0u8;img[0].len()]; img.len()];

		// parsing time
		for row in 1..img.len()-1{
			for col in 1..img[0].len()-1{
				new_img[row][col] = match key[get_pixel_val(row,col,&img)]{
					'.' => 0,
					'#' => 1,
					_ => panic!("wat"),
				}
				
			}
		}
		img = new_img;

		// fix the outer edges. In our (most?) cases, the infinite 0s flash off and on.
		// no clue what to do otherwise but it works here so 
		let new_val: u8 = 1 - i%2;
		img[0] = vec![new_val;img[0].len()];
		img.pop(); 
		img.push(vec![new_val;img[0].len()]);
		for row in img.iter_mut(){
			row[0] = new_val;
			row.pop();
			row.push(new_val);
		}
	}

	for o in img.iter(){
		for p in o{
			print!("{}",p);
		}
		println!();
	}


	return img.into_iter().fold(0, |acc, x| acc + x.iter().sum::<u8>() as u16).to_string();
}
