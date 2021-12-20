use std::collections::HashMap;
/*
 *
 * Hashmap of points, coming from Scanner 0
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 */
// returns the first orientation/offset where at least 12 points match
pub fn get_matching_offset(scanner_base: &Vec<(i32,i32,i32)>, scanner_new: &Vec<(i32,i32,i32)>) 
	-> Result<(u8,i32,i32,i32),()>{

	// 24 orientations
	for i in 0..24{
		for b_i in 0..scanner_base.len(){
			for n_i in 0..scanner_new.len(){
				let (base_x,base_y,base_z) = scanner_base[b_i];
				let (x,y,z) = scanner_new[n_i];
				let (new_x,new_y,new_z) = get_oriented_point(i, x,y,z);
				let off_x = new_x - base_x;
				let off_y = new_y - base_y;
				//println!("{:?} {:?} {:?} {:?}", off_y, new_y, base_y, y);
				let off_z = new_z - base_z;
				let mut amt_in = 0; // idk if iterators have intersection, doing by hand
				for n in scanner_new{
					let (n_x,n_y,n_z) = *n;
					let (newest_x,newest_y,newest_z) = get_oriented_point(i,n_x, n_y, n_z);
					let shifted_x = newest_x - off_x;
					let shifted_y = newest_y - off_y;
					let shifted_z = newest_z - off_z;
					if scanner_base.contains(&(shifted_x,shifted_y,shifted_z)){
						amt_in += 1;
						if amt_in >= 12{
							return Ok((i,off_x,off_y,off_z));
						}
					}
				}
			}
		}
	}
	return Err(());
}
pub fn get_oriented_point(orientation: u8, x:i32, y:i32, z:i32) -> (i32,i32,i32){
	return match orientation{
		0 => (x,y,z),
		1 => (x,z,-y),
		2 => (x,-y,-z),
		3 => (x,-z,y),
		4 => (-x,-y,z),
		5 => (-x,z,y),
		6 => (-x,y,-z),
		7 => (-x,-z,y),

		8 => (y,-x,z),
		9 => (y,z,x),
		10 => (y,x,-z),
		11 => (y,-z,-x),
		12 => (-y,x,z),
		13 => (-y,z,-x),
		14 => (-y,-x,-z),
		15 => (-y,-z,x),

		16 => (z,-y,x),
		17 => (z,x,y),
		18 => (z,y,-x),
		19 => (z,-x,-y),
		20 => (-z,y,x),
		21 => (-z,x,-y),
		22 => (-z,-y,-x),
		23 => (-z,-x,y),
		_ => panic!("Invalid orientation"),
	}
}

pub fn vec_to_world_space(offsets: &HashMap<usize,(usize,u8,i32,i32,i32)>, i: usize, 
	points: &Vec<(i32,i32,i32)>) -> Vec<(i32,i32,i32)>{
	if i == 0{
		return points.to_vec(); // this is probably bad, should learn borrow stuff
	}
	let (sref, or, o_x,o_y,o_z) = offsets[&i];
	let points = points.into_iter().map(|(x,y,z)| { 
		let (nx,ny,nz) = get_oriented_point(or,*x,*y,*z);
		(nx-o_x,ny-o_y,nz-o_z)
	}).collect();
	if sref == 0{
		return points;
	}
	return vec_to_world_space(offsets,sref,&points);
}

pub fn point_to_world_space(offsets: &HashMap<usize,(usize,u8,i32,i32,i32)>, i: usize, 
	point: (i32,i32,i32)) -> (i32,i32,i32){
	if i == 0{
		return point; // this is probably bad, should learn borrow stuff
	}
	let (sref, or, o_x,o_y,o_z) = offsets[&i];
	let (x,y,z) = point;
	let (nx,ny,nz) = get_oriented_point(or,x,y,z);
	let converted_point = (nx-o_x,ny-o_y,nz-o_z);
	if sref == 0{
		return converted_point;
	}
	return point_to_world_space(offsets,sref,converted_point);
}
#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut scanners: Vec<Vec<(i32,i32,i32)>> = vec!();

	// both for the following loop
	let mut scan_scanners: Vec<(i32,i32,i32)> = vec!();
	for l in lines.lines(){
		if l == ""{
			scanners.push(scan_scanners.to_vec());
			scan_scanners.clear();
			continue;
		}
		if l.contains("---"){continue;}
		let l_split:Vec<i32> = l.split(",").collect::<Vec<&str>>()
			.iter().map(|&s| s.parse().unwrap()).collect();
		scan_scanners.push((l_split[0],l_split[1],l_split[2]));
	}
	if !scan_scanners.is_empty(){
		// in case there's no trailing ""
		scanners.push(scan_scanners.to_vec());
	}

	let mut points: Vec<(i32,i32,i32)> = scanners[0].to_vec();

	// scanner: (scanner reference, orientation, off_x, off_y, off_z)
	let mut offsets: HashMap<usize,(usize,u8,i32,i32,i32)> = HashMap::new(); // s: (s,o,x,y,z)
	offsets.insert(0,(0,0,0,0,0));
	for i in 1..scanners.len(){
		// this is the hard part.
		for i_n in 0..scanners.len(){
			if i_n == i{
				continue;
			}
			// prevent circular reference
			if offsets.contains_key(&i_n){
				let (r,_,_,_,_) = offsets[&i_n];
				if r == i{
					continue;
				}

			}

			let (orientation,o_x,o_y,o_z) = match get_matching_offset(&scanners[i_n],&scanners[i]){
				Ok(r) => r,
				Err(_) => continue,
			};
			println!("{:?} {:?} {:?} {:?}, {:?}, {:?}",orientation, o_x,o_y,o_z, i, i_n);
			offsets.insert(i,(i_n,orientation, o_x,o_y,o_z));
			break;
		}
	}
	for (i,s) in scanners.iter().enumerate(){
		for p in vec_to_world_space(&offsets,i,s){
			if !points.contains(&p){
				points.push(p);
			}
		}
	}
	//println!("Scanner 1: {:?}",scanners[1]);
	//println!("{:?}", points);


	for i in 0..offsets.len(){
		println!("{:?}\t{:?}", i, offsets[&i]);
	}
	return points.len().to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut scanners: Vec<Vec<(i32,i32,i32)>> = vec!();

	// both for the following loop
	let mut scan_scanners: Vec<(i32,i32,i32)> = vec!();
	for l in lines.lines(){
		if l == ""{
			scanners.push(scan_scanners.to_vec());
			scan_scanners.clear();
			continue;
		}
		if l.contains("---"){continue;}
		let l_split:Vec<i32> = l.split(",").collect::<Vec<&str>>()
			.iter().map(|&s| s.parse().unwrap()).collect();
		scan_scanners.push((l_split[0],l_split[1],l_split[2]));
	}
	if !scan_scanners.is_empty(){
		// in case there's no trailing ""
		scanners.push(scan_scanners.to_vec());
	}

	// scanner: (scanner reference, orientation, off_x, off_y, off_z)
	let mut offsets: HashMap<usize,(usize,u8,i32,i32,i32)> = HashMap::new(); // s: (s,o,x,y,z)
	offsets.insert(0,(0,0,0,0,0));
	for i in 1..scanners.len(){
		// this is the hard part.
		for i_n in 0..scanners.len(){
			if i_n == i{
				continue;
			}
			// prevent circular reference
			if offsets.contains_key(&i_n){
				let (r,_,_,_,_) = offsets[&i_n];
				if r == i{
					continue;
				}

			}

			let (orientation,o_x,o_y,o_z) = match get_matching_offset(&scanners[i_n],&scanners[i]){
				Ok(r) => r,
				Err(_) => continue,
			};
			println!("{:?} {:?} {:?} {:?}, {:?}, {:?}",orientation, o_x,o_y,o_z, i, i_n);
			offsets.insert(i,(i_n,orientation, o_x,o_y,o_z));
			break;
		}
	}

	let mut scanner_locs: Vec<(i32,i32,i32)> = vec!();
	for i in 0..offsets.len(){
		scanner_locs.push(point_to_world_space(&offsets, i, (0,0,0))); 
	}

	let mut highest_diff = 0;
	for i in 0..scanner_locs.len(){
		for j in i..scanner_locs.len(){
			let (i_x,i_y,i_z) = scanner_locs[i];
			let (j_x,j_y,j_z) = scanner_locs[j];
			let diff = (i_x - j_x).abs() + (i_y - j_y).abs() + (i_z - j_z).abs();
			if diff > highest_diff{
				highest_diff = diff;
			}
		}
	}
	return highest_diff.to_string();

}

