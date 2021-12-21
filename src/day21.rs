use std::collections::HashMap;
#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	let mut player_positions: Vec<u16> = lines.lines().map(|s| s.split(" ").collect::<Vec<&str>>()[4].parse().unwrap()).collect();
	let mut scores= [0u16,0u16];
	let mut turn = 0;
	let mut die = 1;
	let mut rolls: u32 = 0;
	loop{
		rolls += 3;
		player_positions[turn] += die*3 + 3;
		die += 3;
		if die > 100{
			die -= 100;
		}
		while player_positions[turn] > 10{
			player_positions[turn] -= 10;
		}
		scores[turn] += player_positions[turn] as u16;

		if scores[turn] >= 1000{
			break;
		}

		turn = (turn + 1) % 2;
	}
	return ((scores[(turn + 1)%2] as u32)* rolls).to_string();
}

pub fn add_score(map: &mut HashMap<(u64,u64), HashMap<(u64,u64), u64>>,
				score: (u64,u64), pos: (u64,u64), amt: u64){
	match map.remove(&score){
		Some(mut scores) => {
			match scores.remove(&pos){
				Some(old_amt) => {
					scores.insert(pos, amt + old_amt);
				},
				None => {
					scores.insert(pos, amt);
				},
			}
			map.insert(score, scores);			
		},
		None => {
			let mut b: HashMap<(u64,u64), u64> = HashMap::new();
			b.insert(pos, amt);
			map.insert(score,b);
		}
	}
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let player_positions: Vec<u64> = lines.lines().map(|s| s.split(" ").collect::<Vec<&str>>()[4].parse().unwrap()).collect();

	// (p1 score, p2 score): hashmap of positions
	//              (p1 pos, p2 pos): number of hits
	//       omfg i couldve just done a HashMap<(u64,u64,u64,u64)> fuck me dude
	let mut scores: HashMap<(u64,u64), HashMap<(u64,u64), u64>> = HashMap::new();	
	add_score(&mut scores, (0,0),(player_positions[0],player_positions[1]),1);
	let mut p1_wins = 0;
	let mut p2_wins = 0;
	let occurences = [0,0,0,1,3,6,7,6,3,1];
	while scores.len() > 0{
		let mut new_scores: HashMap<(u64,u64), HashMap<(u64,u64), u64>> = HashMap::new();	
		for (scores, poses) in scores{
			for (pos, amt) in poses{
				let (p_1, p_2) = pos;
				let (s_1, s_2) = scores;
				'player1: for i in 3..=9{
					for j in 3..=9{
						let mut new_p_1 = p_1 + i;
						let mut new_p_2 = p_2 + j;
						if new_p_1 > 10 { new_p_1 -= 10 }
						if new_p_2 > 10 { new_p_2 -= 10 }
						let new_s_1 = s_1 + new_p_1;
						let new_s_2 = s_2 + new_p_2;
						//print!("Score: {:?} ", (new_s_1, new_s_2));
						if new_s_1 >= 21{
							// technically I should throw this above the for j
							// but a continue works just as well

							//println!("WIN FOR P1!");
							p1_wins += amt* occurences[i as usize];
							continue 'player1;
						}else if new_s_2 >= 21{
							//println!("WIN FOR P2!");
							p2_wins += amt * occurences[i as usize] * occurences[j as usize];
						}else{
							//println!("no win");
							add_score(&mut new_scores, (new_s_1,new_s_2), (new_p_1,new_p_2), amt * occurences[i as usize] * occurences[j as usize]);
						}
					}
				}
			}			
		}
		scores = new_scores;
	}
	return std::cmp::max(p1_wins,p2_wins).to_string();
}

