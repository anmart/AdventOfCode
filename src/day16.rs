use std::collections::VecDeque;
#[allow(dead_code)]
struct LiteralPacket{
	version: u8,
	type_id: u8,
	value: u32,
}
#[allow(dead_code)]
struct OperatorPacket{
	version: u8,
	type_id: u8,
	length_type_id: u8, // probably not necessary to store
	length_data: u16, // see length_type_id for what this actually means
	sub_packets: Vec<Packet>,

}

enum Packet{
	LiteralPacket,
	OperatorPacket,
}

struct BitQueue{
	bit_storage: VecDeque<u8>, // one u8 per bit
	chars: VecDeque<char>,
}
impl BitQueue{
	fn new(in_chars: VecDeque<char>)->BitQueue{
		return BitQueue{ chars: in_chars, bit_storage: VecDeque::new()};
	}

	#[allow(dead_code)] // ???? it's used below
	fn enqueue_bits(&mut self){
		let hex_val = self.chars.pop_front().unwrap().to_digit(16).unwrap() as u8;
		self.bit_storage.push_back((hex_val >> 3) & 0x1);
		self.bit_storage.push_back((hex_val >> 2) & 0x1);
		self.bit_storage.push_back((hex_val >> 1) & 0x1);
		self.bit_storage.push_back(hex_val & 0x1);
	}
	fn dequeue_bits(&mut self, amt: u8) -> u32{
		let mut ret: u32 = 0;
		for _ in 0..amt{
			if self.bit_storage.is_empty(){
				self.enqueue_bits();
			}
			ret = ret << 1;
			ret = ret + (self.bit_storage.pop_front().unwrap() as u32);
		}
		return ret;
	}
	
	// returns amount of bits discarded
	fn discard_storage(&mut self) -> u16{ 
		let length = self.bit_storage.len();
		self.bit_storage.clear();
		return length as u16;
	}
}

// parses a packet, returns how many bits were used in the process
//pub fn parse_packer() -> (Packet,u8){
	
//	return (Packet::LiteralPacket{version: 0,type_id: 0, value: 0},8);
//}

#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	
	let mut b = BitQueue::new(lines.chars().collect::<VecDeque<char>>());
	let mut tot_ver = 0;

//	let mut curr = (bit_iter.next().unwrap() << 2) + (bit_iter.next().unwrap() << 1) + bit_iter.next().unwrap();

	// get 3 bits for ver


	return lines;
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	return lines;
}

