use std::fs::File;
use std::io::prelude::*;
use std::io;

// https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
pub fn read_file_to_string(filename: &str) -> Result<String,io::Error>{
	let mut f = File::open(filename)?;
	let mut buffer = String::new();
	f.read_to_string(&mut buffer)?;
	Ok(buffer)
}

#[allow(dead_code)]
pub fn bit_string_to_u16(num: &str) -> u16{
	let mut val = 0;
	for c in num.chars(){
		val *= 2;
		if c == '1' {
			val += 1;
		}
	}
	return val;
}

#[allow(dead_code)]
pub fn get_surrounding_nodes(y: usize, x: usize, w: usize, h: usize, diag: bool) -> Vec<(usize,usize)>{
	let mut ret = vec!();
	if x > 0{
		ret.push((y,x-1));
	}
	if x < w-1{
		ret.push((y,x+1));
	}
	if y > 0{
		ret.push((y-1,x));
	}
	if y < h-1{
		ret.push((y+1,x));
	}
	if diag{
		panic!("OOPS haven't done diag yet sorry");
	}

	return ret;
}
