use std::str;
use hex_d_hex::dhex;
use utils::{eq_len_xor,println};

static DECODED: &'static str = "1c0111001f010100061a024b53535009181c";
static KEY: &'static str = "686974207468652062756c6c277320657965";
static ENCODED: &'static str = "746865206b696420646f6e277420706c6179";

pub fn challenge2() -> bool {
	println(String::from("Encode string with equal length xor key"));
	println(format!("hex str: {}", DECODED));
	let hexvec = *dhex(DECODED);
	println(format!("encoded str: {}", str::from_utf8(&hexvec).unwrap()));
	let keyvec = *dhex(KEY);
	println(format!("encoded key: {}", str::from_utf8(&keyvec).unwrap()));
	assert_eq!(hexvec.len(), keyvec.len());
	// xor equal length plaintext vec with key vec
	let resvec = eq_len_xor(&hexvec, &keyvec);
	let resstr = str::from_utf8(resvec.as_slice()).unwrap();
	println(format!("xor result: {}", resstr));
	// ensure result
	let encvec = *dhex(ENCODED);
	let encstr = str::from_utf8(&encvec).unwrap();
	println(format!("encoded str: {}", encstr));
	resstr == encstr
}

#[test]
fn test2() {
	assert!(challenge2());
}
