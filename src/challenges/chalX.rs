use hex_d_hex::dhex;

static DECODED: &'static str = "";
static KEY: &'static str = "";
static ENCODED: &'static str = "";

pub fn challengeX() -> bool {
	#[cfg(not(test))] println!("hex str: {}", DECODED);
	false
}

#[test]
fn testX() {
	assert!(challengeX());
}
