pub fn get_boards(lines: std::str::Lines) -> Vec<[[i8;5];5]>{
	let mut boards = Vec::new();
	// parse remaining into boards
	let mut next_board: usize = 0; // usize can't go below 0 so I have to start at 1
	let mut current_row: usize = 0;
	for line in lines{
		if line == "" {
			next_board += 1;
			current_row = 0;
			let b: [[i8;5];5] = [[0;5];5];
			boards.push(b);
		} else if current_row < 5 {
			let mut column:usize = 0; // need to discard empty strings, so no normal iteration
			for num in line.split(" "){
				if num == ""{
					continue;
				}else{
					boards[next_board-1][current_row][column] = num.parse::<i8>().unwrap();
					column+=1;
				}
			}
			current_row += 1;
		} else {
			panic!("Too many rows in input somehow");
		}
	}
	return boards;
}

pub fn check_bingo(board: &[[i8;5];5], r: usize, c: usize) -> bool{
	// unroll loop manually because i hate myself	
	if 	board[r][0] < 0 &&
		board[r][1] < 0 &&
		board[r][2] < 0 &&
		board[r][3] < 0 &&
		board[r][4] < 0 {
		return true;
	}
	if 	board[0][c] < 0 &&
		board[1][c] < 0 &&
		board[2][c] < 0 &&
		board[3][c] < 0 &&
		board[4][c] < 0 {
		return true;
	}
	return false;
}

pub fn find_score(board: &[[i8;5];5], guess: i8) -> i32{
	// I wanted to do something fun with closures and stuff but not at 3am
	let mut sum: i32 = 0;
	for r in board.iter(){
		for cell in r.iter(){
			sum += std::cmp::max(0,*cell as i32);
		}
	}
	return sum * (guess as i32);
}

#[allow(dead_code)]
pub fn part1(lines: String) -> i32{
	let mut lines = lines.lines();
	let guesses = lines.next().unwrap().split(",");
	let mut boards = get_boards(lines);

	for g in guesses {
		let g: i8 = g.parse().unwrap();
		for b in &mut boards{
			for r in 0..5{
				for c in 0..5{
					if b[r][c] == g{
						// we have arrived at a guess. lots of indents in
						b[r][c] *= -1;
						// check for bingo
						if check_bingo(b,r,c){
							return find_score(b,g);
						}
					}
				}
			}
		}
	}

	return 696969;
}

#[allow(dead_code)]
pub fn part2(lines: String) -> i32{
	let mut lines = lines.lines();
	let guesses = lines.next().unwrap().split(",");
	let mut boards = get_boards(lines);

	for g in guesses {
		let mut finished_board_ids = Vec::new();
		let g: i8 = g.parse().unwrap();
		for (b_id, b) in boards.iter_mut().enumerate(){
			for r in 0..5{
				for c in 0..5{
					if b[r][c] == g{
						// we have arrived at a guess. lots of indents in
						b[r][c] = -1; // this was *= until I realized there were zeros
						// check for bingo
						if check_bingo(b,r,c){
							finished_board_ids.push(b_id);
						}
					}
				}
			}
		}
		// remove the boards that have won already
		for id in finished_board_ids.iter().rev(){
			if boards.len() == 1 {
				// this is the last board so away we go
				return find_score(&boards[0],g);
			}
			println!("removing {:?}", id);
			boards.remove(*id);
		}
	}

	return 696969;
}
