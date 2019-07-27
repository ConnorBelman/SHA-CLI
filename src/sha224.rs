#[path = "preprocess.rs"]
mod preprocess;

use std::num::Wrapping;

const H0_0: u32 = 0xC1059ED8;
const H1_0: u32 = 0x367CD507;
const H2_0: u32 = 0x3070DD17;
const H3_0: u32 = 0xF70E5939;
const H4_0: u32 = 0xFFC00B31;
const H5_0: u32 = 0x68581511;
const H6_0: u32 = 0x64F98fA7;
const H7_0: u32 = 0xBEFA4FA4;

const K: [u32; 64] = [
	0x428A2F98, 0x71374491, 0xB5C0FBCF, 0xE9B5DBA5, 0x3956C25B, 0x59F111F1, 0x923F82A4, 0xAB1C5ED5,
   	0xD807AA98, 0x12835B01, 0x243185BE, 0x550C7DC3, 0x72BE5D74, 0x80DEB1FE, 0x9BDC06A7, 0xC19BF174,
   	0xE49B69C1, 0xEFBE4786, 0x0FC19DC6, 0x240CA1CC, 0x2DE92C6F, 0x4A7484AA, 0x5CB0A9DC, 0x76F988DA,
   	0x983E5152, 0xA831C66D, 0xB00327C8, 0xBf597FC7, 0xC6E00BF3, 0xD5A79147, 0x06CA6351, 0x14292967,
   	0x27B70A85, 0x2E1B2138, 0x4D2C6DFC, 0x53380D13, 0x650A7354, 0x766A0ABB, 0x81C2C92E, 0x92722C85,
   	0xA2BFE8A1, 0xA81A664B, 0xC24B8B70, 0xC76C51A3, 0xD192E819, 0xD6990624, 0xF40E3585, 0x106AA070,
   	0x19A4C116, 0x1E376C08, 0x2748774C, 0x34B0BCB5, 0x391C0CB3, 0x4ED8AA4A, 0x5B9CCA4F, 0x682E6FF3,
   	0x748F82EE, 0x78A5636F, 0x84C87814, 0x8CC70208, 0x90BEFFFA, 0xA4506CEB, 0xBEF9A3F7, 0xC67178F2
];

fn choose(x: u32, y: u32, z: u32) -> u32 {
	(x & y) ^ (!x & z)
}

fn major(x: u32, y: u32, z: u32) -> u32 {
	(x & y) ^ (x & z) ^ (y & z)
}

fn break_block_into_words(block: [u64; 8]) -> [u32; 64] {
	let mut w = [0u32; 64];
	for i in 0..8 {
		w[i * 2] = (block[i] >> 32) as u32;
		w[i * 2 + 1] = block[i] as u32;
	}
	w
}

fn process_blocks(blocks: Vec<[u64; 8]>) -> [u32; 8] {
	let mut hash = [H0_0, H1_0, H2_0, H3_0, H4_0, H5_0, H6_0, H7_0];
	for block in blocks {
		let mut w: [u32; 64] = break_block_into_words(block);
		for i in 16..64 {
			let s0: u32 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
			let s1: u32 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i-2] >> 10);
			w[i] = (Wrapping(w[i - 16]) + Wrapping(s0) + Wrapping(w[i - 7]) + Wrapping(s1)).0;
		}
		let mut a: u32 = hash[0];
		let mut b: u32 = hash[1];
		let mut c: u32 = hash[2];
		let mut d: u32 = hash[3];
		let mut e: u32 = hash[4];
		let mut f: u32 = hash[5];
		let mut g: u32 = hash[6];
		let mut h: u32 = hash[7];
		for i in 0..64 {
			let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
			let temp1 =(Wrapping(h) + Wrapping(s1) + Wrapping(choose(e, f, g)) + Wrapping(K[i]) + Wrapping(w[i])).0;
			let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
			let temp2 = (Wrapping(s0) + Wrapping(major(a, b, c))).0;
			h = g;
			g = f;
			f = e;
			e = (Wrapping(d) + Wrapping(temp1)).0;
			d = c;
			c = b;
			b = a;
			a = (Wrapping(temp1) + Wrapping(temp2)).0;
		}
		hash[0] = (Wrapping(hash[0]) + Wrapping(a)).0;
		hash[1] = (Wrapping(hash[1]) + Wrapping(b)).0;
		hash[2] = (Wrapping(hash[2]) + Wrapping(c)).0;
		hash[3] = (Wrapping(hash[3]) + Wrapping(d)).0;
		hash[4] = (Wrapping(hash[4]) + Wrapping(e)).0;
		hash[5] = (Wrapping(hash[5]) + Wrapping(f)).0;
		hash[6] = (Wrapping(hash[6]) + Wrapping(g)).0;
		hash[7] = (Wrapping(hash[7]) + Wrapping(h)).0;
	}
	hash
}

pub fn generate(message: String) -> String {
	let blocks = preprocess::pre_process_512(message);
	let h = process_blocks(blocks);
	let digest = format!("{:0>8X}{:0>8X}{:0>8X}{:0>8X}{:0>8X}{:0>8X}{:0>8X}", h[0], h[1], h[2], h[3], h[4], h[5], h[6]);
	digest
}