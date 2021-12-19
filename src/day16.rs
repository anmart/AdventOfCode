use std::collections::VecDeque;

#[allow(dead_code)]
pub struct LiteralPacket{
	version: u8,
	type_id: u8,
	value: u64,
}

impl LiteralPacket{
	fn new(version_: u8, type_id_: u8, value_: u64) -> LiteralPacket{
		LiteralPacket{ 
			version: version_,
			type_id: type_id_,
			value: value_,
		}
	}

}
#[allow(dead_code)]
pub struct OperatorPacket{
	version: u8,
	type_id: u8,
	length_type_id: u8, // probably not necessary to store
	length_data: u16, // see length_type_id for what this actually means
	sub_packets: Vec<Packet>,

}

impl OperatorPacket{
	fn new(version_: u8, type_id_: u8, length_type_id_: u8, length_data_: u16) -> OperatorPacket{
		OperatorPacket{ 
			version: version_,
			type_id: type_id_,
			length_type_id: length_type_id_,
			length_data: length_data_,
			sub_packets: vec!(),
		}
	}

}
pub enum Packet{
	LiteralPacket(LiteralPacket),
	OperatorPacket(OperatorPacket),
}

pub struct BitQueue{
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
	fn dequeue_bits(&mut self, amt: u8) -> u64{
		let mut ret: u64 = 0;
		for _ in 0..amt{
			if self.bit_storage.is_empty(){
				self.enqueue_bits();
			}
			ret = ret << 1;
			ret = ret + (self.bit_storage.pop_front().unwrap() as u64);
		}
		return ret;
	}
	
	// returns amount of bits discarded
	// I misunderstood the prompt and thought this was necessary.
	// it is not at all :(
	#[allow(dead_code)]
	fn discard_storage(&mut self) -> u16{ 
		let length = self.bit_storage.len();
		self.bit_storage.clear();
		return length as u16;
	}
}
impl Packet{
	// parses a packet, returns how many bits were used in the process
	pub fn parse_packet(bits: &mut BitQueue) -> (Packet,u16){
		let version = bits.dequeue_bits(3) as u8;
		let type_id = bits.dequeue_bits(3) as u8;
		let mut bits_used: u16 = 6; // 3 from version, 3 from type_id
		if type_id == 4{
			let mut value: u64 = 0;
			loop{
				let command_bit = bits.dequeue_bits(1);
				value = value << 4;
				value += bits.dequeue_bits(4);
				bits_used += 5;
				if command_bit == 0{
					//bits.discard_storage();
					//println!("finishing literal packet with value: {:?}", value);
					return (Packet::LiteralPacket(LiteralPacket::new(version,type_id,value)), bits_used);
				}
			}

		} else{
			let length_type_id = bits.dequeue_bits(1) as u8;
			bits_used += 1;
			let length_data;
			let mut sub_packets: Vec<Packet> = vec!();
			if length_type_id == 0{
				// type 0 is 15 bits long and represents bits
				length_data = bits.dequeue_bits(15) as u16;
				bits_used += 15;
				let mut sub_bits_used = 0;
				//println!("Processing {:?} bits of sub-packets",length_data);
				while sub_bits_used < length_data{
					let (p,b) = Packet::parse_packet(bits);
					bits_used += b;
					sub_packets.push(p);
					sub_bits_used += b as u16;
				}
			}else{
				length_data = bits.dequeue_bits(11) as u16;
				bits_used += 11;
				// type 1 is 11 bits long and represents packets
				//println!("Processing {:?} sub-packets", length_data);
				for _ in 0..length_data{
					let (p,b) = Packet::parse_packet(bits);
					bits_used += b;
					sub_packets.push(p);
				}
			}
			let mut ret_pack = OperatorPacket::new(version,type_id,length_type_id,length_data);
			ret_pack.sub_packets = sub_packets;
			return (Packet::OperatorPacket(ret_pack),bits_used);
		}
	}

	pub fn get_packet_versions(p: Packet) -> u64{
		return match p{
			Packet::LiteralPacket(l) => l.version as u64,
			Packet::OperatorPacket(o) => {
				let mut v = 0;
				for sp in o.sub_packets{
					v += Packet::get_packet_versions(sp);
				}
				v + o.version as u64
			},

		}

	}

	pub fn evaluate_packet(p: &Packet) -> i64{
		return match p{
			Packet::LiteralPacket(l) => l.value as i64,
			Packet::OperatorPacket(o) => match o.type_id{
				0 => o.sub_packets.iter()
					.map(|s| Packet::evaluate_packet(s))
					.collect::<Vec<i64>>().iter().sum::<i64>(),
				1 => o.sub_packets.iter()
					.map(|s| Packet::evaluate_packet(s))
					.collect::<Vec<i64>>().iter().product(),
				2 => *o.sub_packets.iter()
					.map(|s| Packet::evaluate_packet(s))
					.collect::<Vec<i64>>().iter().min().unwrap(),
				3 => *o.sub_packets.iter()
					.map(|s| Packet::evaluate_packet(s))
					.collect::<Vec<i64>>().iter().max().unwrap(),
				5 => if Packet::evaluate_packet(&o.sub_packets[0]) 
					> Packet::evaluate_packet(&o.sub_packets[1]) {1} else {0},
				6 => if Packet::evaluate_packet(&o.sub_packets[0]) 
					< Packet::evaluate_packet(&o.sub_packets[1]) {1} else {0},
				7 => if Packet::evaluate_packet(&o.sub_packets[0]) 
					== Packet::evaluate_packet(&o.sub_packets[1]) {1} else {0},
				_ => panic!("error: incorrect packet type"),

			}

		}
	}
}

#[allow(dead_code)]
pub fn part1(lines: String) -> String{
	
	let mut b = BitQueue::new(lines.chars().collect::<VecDeque<char>>());
	let (p,_) = Packet::parse_packet(&mut b);


	return Packet::get_packet_versions(p).to_string();
}

#[allow(dead_code)]
pub fn part2(lines: String) -> String{
	let mut b = BitQueue::new(lines.chars().collect::<VecDeque<char>>());
	let (p,_) = Packet::parse_packet(&mut b);


	return Packet::evaluate_packet(&p).to_string();
}

