use std::str;
use hex_d_hex::dhex;
use utils::{println,brute_single_byte_xor,vec_byte_xor};

static DECODED: &'static str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

pub fn challenge3() -> bool {
	println(String::from("Bruteforce single byte xor key from encrypted text"));
	println(format!("hex str: {}", DECODED));
	let hexvec = *dhex(DECODED);
	let key = brute_single_byte_xor(&hexvec);
	println(format!("xor key: {}", key));
	let decvec = vec_byte_xor(&hexvec, key);
	let decstr = str::from_utf8(&decvec).unwrap();
	println(format!("decoded str: {}", decstr));
	let checkvec:Vec<bool> = decvec.iter().map(|c| {
		match *c {
			65...90 => true, // lower
			97...122 => true, // upper
			32...126 => true, // numbers, puctuation
			9 => true, // tab
			10 => true, // cr
			13 => true, // lf
			_ => false,
		}
	}).collect();
	checkvec.contains(&false) == false
}

#[test]
fn test3() {
	assert!(challenge3());
}
