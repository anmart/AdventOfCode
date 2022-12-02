use std::collections::HashMap;

#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut map: Vec<Vec<u8>> = vec!();
	for l in lines.lines(){
		map.push(l.chars().collect::<Vec<char>>().into_iter().map(|x| (x as u8) - b'0').collect());
	}
	let goal = (map.len()-1,map[0].len()-1);
	let mut open: Vec<(usize,usize)> = vec!();
	let mut visited: Vec<(usize,usize)> = vec!();
	let mut came_from:HashMap<(usize,usize),(usize,usize)> = HashMap::new();
	let mut tentative_distance = vec![vec![u32::MAX; map.len()]; map.len()];
	tentative_distance[0][0] = 0;

	open.push((0,0));
	while open.len() > 0{
		let (y,x) = open.pop().unwrap();
		visited.push((y,x));
		let surroundings = super::get_surrounding_nodes(y,x,map.len(),map[0].len(),false);
		let dist_here = map[y][x] as u32 + tentative_distance[y][x]; // tent. of all surroundings
		for node in surroundings{
			if !visited.contains(&node){
				if !open.contains(&node){
					open.push(node);
				}
				let (n_y,n_x) = node;
				if dist_here < tentative_distance[n_y][n_x]{
					tentative_distance[n_y][n_x] = dist_here;
					if came_from.contains_key(&node){
						came_from.remove(&node);
					}
					came_from.insert(node,(y,x));
				}
			}
		}
		if (y,x) == goal{
			break;
		}
		open.sort_by_key(|k| {let (k_y,k_x) = *k; std::cmp::Reverse(tentative_distance[k_y][k_x])});
	}

	let mut current = goal;
	let mut tot: u32 = 0;
	while came_from.contains_key(&current){ // will skip 0,0 which is good
		let (y,x) = current;
		println!("{:?} is {:?}", current, map[y][x]);
		tot += map[y][x] as u32;
		current = came_from[&current];
	}

	return tot.to_string();
}
/*The above is an implementation of Dijkstra's using the wikipedia page for it
 *
 *
 *
 */

pub fn get_map_val(y: usize, x: usize, map: &Vec<Vec<u8>>) -> u8{
	let mod_val = (y/map.len()) + (x/map[0].len());
	//println!("loc: {:?}, base: {:?}, mod: {:?}", (y,x), map[y%map.len()][x%map[0].len()],mod_val);
	let val = map[y%map.len()][x%map[0].len()] + (mod_val as u8);
	return if val > 9 { val - 9 } else { val };
}
#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut map: Vec<Vec<u8>> = vec!();
	for l in lines.lines(){
		map.push(l.chars().collect::<Vec<char>>().into_iter().map(|x| (x as u8) - b'0').collect());
	}
	let goal = (5*map.len()-1,5*map[0].len()-1);
	let mut open: Vec<(usize,usize)> = vec!();
	let mut visited: Vec<(usize,usize)> = vec!();
	let mut came_from:HashMap<(usize,usize),(usize,usize)> = HashMap::new();
	let mut tentative_distance = vec![vec![u32::MAX; map.len()*5]; map.len()*5];
	tentative_distance[0][0] = 0;

	let loop_benchmark = std::time::Instant::now();
	let mut tot_checked = 0;
	open.push((0,0));
	while open.len() > 0{
		tot_checked += 1;
		if tot_checked % 10_000 == 0{
			println!("Checked: {:?}. Time elapsed so far: {:?}", tot_checked, loop_benchmark.elapsed());
		}
		let (y,x) = open.pop().unwrap();
		visited.push((y,x));
		let surroundings = super::get_surrounding_nodes(y,x,map.len()*5,map[0].len()*5,false);
		let dist_here = get_map_val(y,x,&map) as u32 + tentative_distance[y][x];
		for node in surroundings{
			if !visited.contains(&node){
				let (n_y,n_x) = node;
				if dist_here < tentative_distance[n_y][n_x]{
					tentative_distance[n_y][n_x] = dist_here;
					if came_from.contains_key(&node){
						came_from.remove(&node);
					}
					if !open.contains(&node){
						open.push(node);
					}
					came_from.insert(node,(y,x));
				}
			}
		}
		if (y,x) == goal{
			break;
		}
		open.sort_by_key(|k| {let (k_y,k_x) = *k; std::cmp::Reverse(tentative_distance[k_y][k_x])});
	}

	println!("Total Checked: {:?}. Total Nodes: {:?}", tot_checked, map.len()*map[0].len()*25);

	let mut current = goal;
	let mut tot: u32 = 0;
	while came_from.contains_key(&current){ // will skip 0,0 which is good
		let (y,x) = current;
		tot += get_map_val(y,x,&map)as u32;
		current = came_from[&current];
	}

	return tot.to_string();
}
