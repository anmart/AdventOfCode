#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut tot = 0;
	for l in lines.lines(){
		let z:Vec<i32> = l.split(" | ").collect::<Vec<_>>()[1].split(" ").map(|x| x.len() as i32).collect();
		//tot += z.iter().fold(0, |a,&x| if x == 2 || x == 3 || x == 4 || x == 7{ a+x }else{a});
		for t in z{
			if t == 2 || t == 3 || t == 4 || t == 7{
				tot+=1;
			}
		}
	}
	return tot.to_string();
}

pub fn substr(str1:&str, str2:&str) -> Vec<u8>{
	let mut ret:Vec<char> = str1.chars().collect();
	ret.retain(|&x| !(str2.contains(x)));
	let ret:Vec<u8> = ret.iter().map(|&x| (x as u8) - _ASCII_START).collect();
	return ret;
}

const _ASCII_START:u8 = 'a' as u8;
#[allow(dead_code)]
pub fn part2(lines: String) -> String{	
	let mut tot = 0;
	for l in lines.lines(){
		let display_sets:Vec<_> = l.split(" | ").collect();
		let mut display:Vec<_> = display_sets[0].split(" ").collect();
		display.sort_by(|&a,&b| a.len().cmp(&b.len()));
		let mut indices: [usize;10] = [20;10];
		indices[1] = 0;
		indices[4] = 2;
		indices[7] = 1;
		indices[8] = 9;
		// [20, 1, 20, 3, 4, 5, 20, 7, 8, 6]
		// [1] [7] [4] [2,3,5] [2,3,5] [2,3,5] [0,6,9] [0,6,9] [0,6,9] [8]

		// figuring out 3
		for i in 3..=5{
			if substr(display[i], display[indices[1]]).len() == 3{
				indices[3] = i;
				break;
			}
		}

		// figuring out 9
		for i in 6..=8{
			if substr(display[i], display[indices[3]]).len() == 1{
				indices[9] = i;
				break;
			}
		}

		// figure out e display and 2 and 5
		let e_seg = substr(display[indices[8]], display[indices[9]]).pop().unwrap();
		for i in 3..=5{
			let b = substr(display[i], display[indices[3]]);
			if b.len() == 0{continue;}
			if b[0] == e_seg{
				// i is 2
				indices[2] = i;
			}else{
				indices[5] = i;
			}
		}


		// figure out 6 and 0
		for i in 6..=8{
			let b = substr(display[i], display[indices[7]]);
			if indices[9] == i{continue;}
			if b.len() == 4{
				// i is 6
				indices[6] = i;
			}else{
				indices[0] = i;
			}
		}



		// this is horrible
		let outs:Vec<_> = display_sets[1].split(" ").collect();
		let mut value = 0;
		for o in outs{
			let mut l2:Vec<_> = o.chars().collect();
			for i in 0..10{
				let mut l1:Vec<_> = display[indices[i]].chars().collect();
				l1.sort();
				l2.sort();
				if l1 == l2{
					value *= 10; 
					value += i as i32;
					break;
				}

			}

		}
		tot += value;
		
	}
	return tot.to_string();
}
/*
 *
  aaaa    0000 
 b    c  1    2
 b    c  1    2
  dddd    3333 
 e    f  4    5
 e    f  4    5
  gggg    6666
 *
 *
 *
 */

