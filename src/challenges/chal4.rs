use hex_d_hex::dhex;

static DECODED: &'static str = "";
static KEY: &'static str = "";
static ENCODED: &'static str = "";

pub fn challenge4() -> bool {
	#[cfg(not(test))] println!("hex str: {}", DECODED);
	false
}

#[test]
fn test4() {
	assert!(challenge4());
}
