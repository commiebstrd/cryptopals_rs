use std::collections::HashMap;

/////////////////////
////// helpers //////
/////////////////////
pub fn println(s:String) {
	#[cfg(not(test))] println!("{}", s);
	#[cfg(test)] let _ = s;
}
#[allow(dead_code)]
pub fn char_hashmap(chars:&Vec<u8>) -> HashMap<&u8,usize> {
	// create hashmap of each char to number of times found
	let mut hexhash = HashMap::new();
	for chr in chars.iter() {
		let ichr = hexhash.entry(chr).or_insert(0);
		*ichr+=1;
	};
	println(format!("hexhash empty: {} size: {}", hexhash.is_empty(), hexhash.len()));
	assert!( !hexhash.is_empty() );
	hexhash
}
#[allow(dead_code)]
pub fn find_most(chars:HashMap<u8,usize>) -> (u8,usize) {
	// find highest counted char in hashmap
	let mut kv:(u8,usize) = (0,0);
	for (k,v) in chars.iter() {
		if kv.1 < *v {
			println(format!("k: {} v: {}", *k, *v));
			kv = (*k,*v);
		}
	}
	assert_ne!(kv, (0,0));
	kv
}
/////////////////////
//// challenge 2 ////
/////////////////////
pub fn eq_len_xor(pt:&Vec<u8>, key:&Vec<u8>) -> Vec<u8> {
	pt.iter()
		.zip( key.iter() )
		.map( |(h,k)| h^k )
		.collect()
}
/////////////////////
//// challenge 3 ////
/////////////////////
pub fn vec_byte_xor(chars:&Vec<u8>, key:u8) -> Vec<u8> {
	// xor a vec by a single byte key
	chars.iter().map(|c| c^key).collect()
}
pub fn chi_squared(chars:&Vec<u8>) -> f64 {
	// frequency map of each char in alphabet
	let engfreq:Vec<f64> =  vec![
	    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015,  // A-G
	    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749,  // H-N
	    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758,  // O-U
	    0.00978, 0.02360, 0.00150, 0.01974, 0.00074                     // V-Z
	];
	// walk string mapping chars to freq, ignore some
	let mut ignored = 0;
	let mut freqmap:HashMap<u8,f64> = HashMap::new();
	for c in chars.iter() {
		// if within our range, add char k, with freq v
		let _ = match *c {
			65...90 => freqmap.entry(*c-65).or_insert(engfreq[(*c-65) as usize]), // lower
			97...122 => freqmap.entry(*c-97).or_insert(engfreq[(*c-97) as usize]), // upper
			32...126 => { ignored+=1; continue; }, // numbers, puctuation
			9 => { ignored+=1; continue; }, // tab
			10 => { ignored+=1; continue; }, // cr
			13 => { ignored+=1; continue; }, // lf
			_ => return 0.0,
		};
	}
	let mut chi:f64 = 0.0;
	let len:f64 = (chars.len()-ignored) as f64;
	for (chr,freq) in freqmap.iter() {
		let observed:f64 = *freq;
		let expected:f64 = len * engfreq[*chr as usize];
		let difference:f64 = observed - expected;
		chi += difference * difference / expected;
	}
	chi
}
pub fn brute_single_byte_xor(chars:&Vec<u8>) -> u8 {
	// generate potential xor keys
	let xorrange:Vec<u8> = (0..255).map(|b| b).collect();
	// iter over pkeys, decode str, chi2(str), store (xor_key,chi2val)
	let chimap:HashMap<u8,f64> = xorrange.iter().map(|&k| {
		let decoded:Vec<u8> = vec_byte_xor(chars, k);
		let cs = chi_squared(&decoded);
		(k,cs)
	}).collect();
	// find highest rated key
	let mut key: u8 = 0;
	let mut hichi: f64 = 0.0;
	for (k,c) in chimap.iter() {
		if hichi < *c {
			key = *k;
			hichi = *c;
		}
	}
	key
}
/////////////////////
//// challenge 4 ////
/////////////////////
