use hex_d_hex::dhex;
use utils::println;
use base64::encode;

static DECODED: &'static str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
static ENCODED: &'static str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn hex2str64(input:&str) -> String {
	// convert string of hex chars to ascii string
	let hexvec = *dhex(input);
	// return b64 encoded string
	encode(&hexvec)
}

pub fn challenge1() -> bool {
	println(String::from("Convert string of hex chars to base64"));
	println(format!("hex str: {}" ,DECODED));
	let encoded = hex2str64(DECODED);
	println(format!("encoded str: {}", encoded));
	// ensure result
	encoded == ENCODED
}

#[test]
fn test1() {
	assert!(challenge1());
}
