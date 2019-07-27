#[path = "preprocess.rs"]
mod preprocess;

use std::num::Wrapping;

const H0_0: u32 = 0x67452301;
const H1_0: u32 = 0xEFCDAB89;
const H2_0: u32 = 0x98BADCFE;
const H3_0: u32 = 0x10325476;
const H4_0: u32 = 0xC3D2E1F0;

fn choose(x: u32, y: u32, z: u32) -> u32 {
	(x & y) | (!x & z)
}

fn parity(x: u32, y: u32, z: u32) -> u32 {
	x ^ y ^ z
}

fn major(x: u32, y: u32, z: u32) -> u32 {
	(x & y) | (x & z) | (y & z)
}

fn break_block_into_words(block: [u64; 8]) -> [u32; 80] {
	let mut w = [0u32; 80];
	for i in 0..8 {
		w[i * 2] = (block[i] >> 32) as u32;
		w[i * 2 + 1] = block[i] as u32;
	}
	w
}

fn process_blocks(blocks: Vec<[u64; 8]>) -> [u32; 5] {
	let mut h = [H0_0, H1_0, H2_0, H3_0, H4_0];
	for block in blocks {
		let mut w: [u32; 80] = break_block_into_words(block);
		for i in 16..80 {
			w[i] = w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]
		}
		let mut a: u32 = h[0];
		let mut b: u32 = h[1];
		let mut c: u32 = h[2];
		let mut d: u32 = h[3];
		let mut e: u32 = h[4];
		let mut f: u32 = 0;
		let mut k: u32 = 0;
		for i in 0..80 {
			if i <= 19 {
				f = choose(b, c, d);
				k = 0x5A827999;
			}
			else if 20 <= i && i <= 39 {
				f = parity(b, c, d);
				k = 0x6ED9EBA1;
			}
			else if 40 <= i && i <= 59 {
				f = major(b, c, d);
				k = 0x8F1BBCDC;
			}
			else if 60 <= i && i <= 79 {
				f = parity(b, c, d);
				k = 0xCA62C1D6;
			}
			let temp: u32 = (a.rotate_left(5) as u64 + f as u64 + e as u64 + k as u64 + w[i] as u64) as u32;
			e = d;
			d = c;
			c = b.rotate_left(30);
			b = a;
			a = temp;
		}
		h[0] = (Wrapping(h[0]) + Wrapping(a)).0;
		h[1] = (Wrapping(h[1]) + Wrapping(b)).0;
		h[2] = (Wrapping(h[2]) + Wrapping(c)).0;
		h[3] = (Wrapping(h[3]) + Wrapping(d)).0;
		h[4] = (Wrapping(h[4]) + Wrapping(e)).0;

	}
	h
}

pub fn generate(message: String) -> String {
	let blocks = preprocess::pre_process_512(message);
	let h = process_blocks(blocks);
	let digest = format!("{:0>8X}{:0>8X}{:0>8X}{:0>8X}{:0>8X}", h[0], h[1], h[2], h[3], h[4]);
	digest
}