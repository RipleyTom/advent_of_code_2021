use std::fs::File;
use std::io::Read;

fn parse_input() -> Result<Vec<u8>, std::io::Error> {
	let mut str_file = String::new();
	File::open("input_16.txt")?.read_to_string(&mut str_file)?;

	let char_to_bits = |c: char| -> [u8; 4] {
		let v = c.to_digit(16).unwrap() as u8;
		let mut res: [u8; 4] = [0; 4];
		for i in 0..4 {
			res[i] = (v >> (3 - i)) & 1;
		}
		res
	};

	Ok(str_file.trim().chars().map(char_to_bits).flatten().collect())
}

struct Packet {
	version: u8,
	type_id: u8,
	data: PacketType,
}

impl Packet {
	pub fn new(version: u8, type_id: u8, data: PacketType) -> Packet {
		Packet { version, type_id, data }
	}

	pub fn count_version(&self) -> u64 {
		match &self.data {
			PacketType::LiteralPacket(_) => self.version as u64,
			PacketType::OperatorPacket(packets) => {
				let mut sum = self.version as u64;
				for p in packets {
					sum += p.count_version();
				}
				sum
			}
		}
	}

	pub fn do_operation(&self) -> u64 {
		match &self.data {
			PacketType::LiteralPacket(v) => *v,
			PacketType::OperatorPacket(packets) => {
				let eval_packets: Vec<u64> = packets.iter().map(|p| p.do_operation()).collect();

				match self.type_id {
					0 => eval_packets.iter().sum(),
					1 => {
						let mut product = 1;
						eval_packets.iter().for_each(|p| product *= p);
						product
					}
					2 => *eval_packets.iter().min().unwrap(),
					3 => *eval_packets.iter().max().unwrap(),
					5 => {
						if eval_packets[0] > eval_packets[1] {
							1
						} else {
							0
						}
					}
					6 => {
						if eval_packets[0] < eval_packets[1] {
							1
						} else {
							0
						}
					}
					7 => {
						if eval_packets[0] == eval_packets[1] {
							1
						} else {
							0
						}
					}
					_ => panic!("Unexpected packet type!"),
				}
			}
		}
	}
}

enum PacketType {
	LiteralPacket(u64),
	OperatorPacket(Vec<Packet>),
}

fn array_to_value(arr: &[u8]) -> u64 {
	let mut res = 0;
	for v in arr {
		res <<= 1;
		res |= *v as u64;
	}

	res
}

fn read_packet(bits: &Vec<u8>, mut i: usize) -> (Packet, usize) {
	let version = array_to_value(&bits[i..i + 3]) as u8;
	let type_id = array_to_value(&bits[i + 3..i + 6]) as u8;
	i += 6;

	let data;

	if type_id == 4 {
		// Value
		let mut value = 0;
		let mut finished = 1;
		while finished == 1 {
			let tmp = array_to_value(&bits[i..i + 5]);
			i += 5;
			finished = tmp >> 4;
			value <<= 4;
			value |= tmp & 0xF;
		}
		println!("Literal packet with value {}", value);

		data = PacketType::LiteralPacket(value);
	} else {
		let mut sub_vec = Vec::new();
		// Operator
		let sub_type = bits[i];
		i += 1;

		if sub_type == 0 {
			let bits_size = array_to_value(&bits[i..i + 15]);
			i += 15;
			let end = i + bits_size as usize;
			while i < end {
				let (new_packet, new_i) = read_packet(&bits, i);
				sub_vec.push(new_packet);
				i = new_i;
			}
		} else {
			let num_packets = array_to_value(&bits[i..i + 11]);
			i += 11;

			for _ in 0..num_packets {
				let (new_packet, new_i) = read_packet(&bits, i);
				sub_vec.push(new_packet);
				i = new_i;
			}
		}

		println!("Operator packet with {} subpackets", sub_vec.len());
		data = PacketType::OperatorPacket(sub_vec);
	}

	(Packet::new(version, type_id, data), i)
}

pub fn run_a() -> Result<u64, std::io::Error> {
	let bits = parse_input()?;

	let num_bits = bits.len();
	let mut i = 0;

	let mut vec_packets = Vec::new();

	while i < (num_bits - 6) {
		let (new_packet, new_i) = read_packet(&bits, i);
		vec_packets.push(new_packet);
		i = new_i;
	}

	Ok(vec_packets.iter().map(|p| p.count_version()).sum())
}

pub fn run_b() -> Result<u64, std::io::Error> {
	let bits = parse_input()?;

	let num_bits = bits.len();
	let mut i = 0;

	let mut vec_packets = Vec::new();

	while i < (num_bits - 6) {
		let (new_packet, new_i) = read_packet(&bits, i);
		vec_packets.push(new_packet);
		i = new_i;
	}

	Ok(vec_packets[0].do_operation())
}
