#[path = "preprocess.rs"]
mod preprocess;

use std::num::Wrapping;

const H0_0: u64 = 0xCBBB9D5DC1059ED8;
const H1_0: u64 = 0x629A292A367CD507; 
const H2_0: u64 = 0x9159015A3070DD17;
const H3_0: u64 = 0x152FECD8F70E5939;
const H4_0: u64 = 0x67332667FFC00B31;
const H5_0: u64 = 0x8EB44A8768581511;
const H6_0: u64 = 0xDB0C2E0D64F98FA7;
const H7_0: u64 = 0x47B5481DBEFA4FA4;

const K: [u64; 80] = [
	0x428A2F98D728AE22, 0x7137449123EF65CD, 0xB5C0FBCFEC4D3B2F, 0xE9B5DBA58189DBBC, 0x3956C25BF348B538, 
    0x59F111F1B605D019, 0x923F82A4AF194F9B, 0xAB1C5ED5DA6D8118, 0xD807AA98A3030242, 0x12835B0145706FBE, 
    0x243185BE4EE4B28C, 0x550C7DC3D5FFB4E2, 0x72BE5D74F27B896F, 0x80DEB1FE3B1696B1, 0x9BDC06A725C71235, 
    0xC19BF174CF692694, 0xE49B69C19EF14AD2, 0xEFBE4786384F25E3, 0x0FC19DC68B8CD5B5, 0x240CA1CC77AC9C65, 
    0x2DE92C6F592B0275, 0x4A7484AA6EA6E483, 0x5CB0A9DCBD41FBD4, 0x76F988DA831153B5, 0x983E5152EE66DFAB, 
    0xA831C66D2DB43210, 0xB00327C898FB213F, 0xBF597FC7BEEF0EE4, 0xC6E00BF33DA88FC2, 0xD5A79147930AA725, 
    0x06CA6351E003826F, 0x142929670A0E6E70, 0x27B70A8546D22FFC, 0x2E1B21385C26C926, 0x4D2C6DFC5AC42AED, 
    0x53380D139D95B3DF, 0x650A73548BAF63DE, 0x766A0ABB3C77B2A8, 0x81C2C92E47EDAEE6, 0x92722C851482353B, 
    0xA2BFE8A14CF10364, 0xA81A664BBC423001, 0xC24B8B70D0F89791, 0xC76C51A30654BE30, 0xD192E819D6EF5218, 
    0xD69906245565A910, 0xF40E35855771202A, 0x106AA07032BBD1B8, 0x19A4C116B8D2D0C8, 0x1E376C085141AB53, 
    0x2748774CDF8EEB99, 0x34B0BCB5E19B48A8, 0x391C0CB3C5C95A63, 0x4ED8AA4AE3418ACB, 0x5B9CCA4F7763E373, 
    0x682E6FF3D6B2B8A3, 0x748F82EE5DEFB2FC, 0x78A5636F43172F60, 0x84C87814A1F0AB72, 0x8CC702081A6439EC, 
    0x90BEFFFA23631E28, 0xA4506CEBDE82BDE9, 0xBEF9A3F7B2C67915, 0xC67178F2E372532B, 0xCA273ECEEA26619C, 
    0xD186B8C721C0C207, 0xEADA7DD6CDE0EB1E, 0xF57D4F7FEE6ED178, 0x06F067AA72176FBA, 0x0A637DC5A2C898A6, 
    0x113F9804BEF90DAE, 0x1B710B35131C471B, 0x28DB77F523047D84, 0x32CAAB7B40C72493, 0x3C9EBE0A15C9BEBC, 
    0x431D67C49C100D4C, 0x4CC5D4BECB3E42B6, 0x597F299CFC657E2A, 0x5FCB6FAB3AD6FAEC, 0x6C44198C4A475817
];

fn choose(x: u64, y: u64, z: u64) -> u64 {
	(x & y) ^ (!x & z)
}

fn major(x: u64, y: u64, z: u64) -> u64 {
	(x & y) ^ (x & z) ^ (y & z)
}

fn break_block_into_words(block: [u128; 8]) -> [u64; 80] {
	let mut w = [0u64; 80];
	for i in 0..8 {
		w[i * 2] = (block[i] >> 64) as u64;
		w[i * 2 + 1] = block[i] as u64;
	}
	w
}

fn process_blocks(blocks: Vec<[u128; 8]>) -> [u64; 8] {
	let mut hash = [H0_0, H1_0, H2_0, H3_0, H4_0, H5_0, H6_0, H7_0];
	for block in blocks {
		let mut w: [u64; 80] = break_block_into_words(block);
		for i in 16..80 {
			let s0: u64 = w[i - 15].rotate_right(1) ^ w[i - 15].rotate_right(8) ^ (w[i - 15] >> 7);
			let s1: u64 = w[i - 2].rotate_right(19) ^ w[i - 2].rotate_right(61) ^ (w[i-2] >> 6);
			w[i] = (Wrapping(w[i - 16]) + Wrapping(s0) + Wrapping(w[i - 7]) + Wrapping(s1)).0;
		}
		let mut a: u64 = hash[0];
		let mut b: u64 = hash[1];
		let mut c: u64 = hash[2];
		let mut d: u64 = hash[3];
		let mut e: u64 = hash[4];
		let mut f: u64 = hash[5];
		let mut g: u64 = hash[6];
		let mut h: u64 = hash[7];
		for i in 0..80 {
			let s1 = e.rotate_right(14) ^ e.rotate_right(18) ^ e.rotate_right(41);
			let temp1 =(Wrapping(h) + Wrapping(s1) + Wrapping(choose(e, f, g)) + Wrapping(K[i]) + Wrapping(w[i])).0;
			let s0 = a.rotate_right(28) ^ a.rotate_right(34) ^ a.rotate_right(39);
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
	let blocks = preprocess::pre_process_1024(message);
	let h = process_blocks(blocks);
	let digest = format!("{:0>16X}{:0>16X}{:0>16X}{:0>16X}{:0>16X}{:0>16X}", h[0], h[1], h[2], h[3], h[4], h[5]);
	digest
}