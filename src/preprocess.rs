fn bytes_to_block_512(message: Vec<u8>) -> [u64; 8] {
	let mut block = [0u64; 8];
	for i in 0..8 {
		for j in 0..8 {
			block[i] = block[i] | ((message[i * 8 + j] as u64) << ((7 - j as u8) * 8));
		}
	}
	block
}

fn bytes_to_block_1024(message: Vec<u8>) -> [u128; 8] {
	let mut block = [0u128; 8];
	for i in 0..8 {
		for j in 0..16 {
			block[i] = block[i] | ((message[i * 16 + j] as u128) << ((15 - j as u8) * 8));
		}
	}
	block
}

fn u64_to_u8_vec(num: u64) -> Vec<u8> {
	let mut num_vec = Vec::new();
	for i in 0..8 {
		num_vec.push((num >> (8 * (7 - i))) as u8);
	}
	num_vec
}

fn u128_to_u8_vec(num: u128) -> Vec<u8> {
	let mut num_vec = Vec::new();
	for i in 0..16 {
		num_vec.push((num >> (8 * (15 - i))) as u8);
	}
	num_vec
}

fn pad_message_512(message: &[u8], message_length: u64) -> Vec<[u64; 8]> {
	let mut padded_message = Vec::new();
	let mut cur_message = message.to_vec();
	if (message.len() * 8) % 512 != 448 {
		cur_message.push(0x80);
	}
	while (cur_message.len() * 8) % 512 != 448 {
		cur_message.push(0x00);
	}
	cur_message.append(&mut u64_to_u8_vec(message_length));
	while !cur_message.is_empty() {
		padded_message.push(bytes_to_block_512(cur_message[0..64].to_vec()));
		cur_message = cur_message[64..].to_vec();
	}
	padded_message
}

fn pad_message_1024(message: &[u8], message_length: u128) -> Vec<[u128; 8]> {
	let mut padded_message = Vec::new();
	let mut cur_message = message.to_vec();
	if (message.len() * 8) % 1024 != 896 {
		cur_message.push(0x80);
	}
	while (cur_message.len() * 8) % 1024 != 896 {
		cur_message.push(0x00);
	}
	cur_message.append(&mut u128_to_u8_vec(message_length));
	while !cur_message.is_empty() {
		padded_message.push(bytes_to_block_1024(cur_message[0..128].to_vec()));
		cur_message = cur_message[128..].to_vec();
	}
	padded_message
}

pub fn pre_process_512(message: String) -> Vec<[u64; 8]> {
	let mut blocks = Vec::new();
	let mut message_bytes = message.as_bytes().to_vec();
	let message_length: u64 = message_bytes.len() as u64 * 8;
	while message_bytes.len() >= 64 {
		blocks.push(bytes_to_block_512(message_bytes[0..64].to_vec()));
		message_bytes = message_bytes[64..].to_vec();
	}
	blocks.append(&mut pad_message_512(&message_bytes, message_length));
	blocks
}

pub fn pre_process_1024(message: String) -> Vec<[u128; 8]> {
	let mut blocks = Vec::new();
	let mut message_bytes = message.as_bytes().to_vec();
	let message_length: u128 = message_bytes.len() as u128 * 8;
	while message_bytes.len() >= 128 {
		blocks.push(bytes_to_block_1024(message_bytes[0..128].to_vec()));
		message_bytes = message_bytes[128..].to_vec();
	}
	blocks.append(&mut pad_message_1024(&message_bytes, message_length));
	blocks
}