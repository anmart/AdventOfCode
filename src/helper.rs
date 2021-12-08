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
