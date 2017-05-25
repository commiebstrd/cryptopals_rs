extern crate getopts;
extern crate regex;
extern crate base64;
extern crate hex_d_hex;

mod args;
mod utils;
mod challenges;

use args::Args;
use challenges::chal1::challenge1;
use challenges::chal2::challenge2;
use challenges::chal3::challenge3;
use challenges::chal4::challenge4;

const MAXCHALS:usize = 4;

fn main() {
	// accept args
	let mut args = Args::default();
	match args.set_args(MAXCHALS) {
		Some(e) => { e.handle_error(); return },
		None => { },
	};
	if args.challenges.contains(&true) == false {
		println!("No challenges selected, exiting");
		return;
	}
  for (c, b) in args.challenges.iter().enumerate() {
		if !b { continue; }
		println!("Running challenge {}", c);
		let res:bool = match c {
			0 => { println!("challenge 0 is invalid"); continue; },
			1 => challenge1(),
			2 => challenge2(),
			3 => challenge3(),
			3 => challenge3(),
			_ => { println!("Found invalid challenge"); return; }
		};
		if res { println!("Success!\n"); }
		else { println!("Failure!\n") }
	}
}
