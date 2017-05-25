use std::env;
use getopts::Options;
use regex::Regex;

#[allow(dead_code)]
#[derive(Default)]
pub struct Args {
	pub challenges: Vec<bool>,
}
#[allow(dead_code)]
impl Args {
	pub fn set_args(&mut self, max_chals:usize) -> Option<ArgErr> {
		let args: Vec<String> = env::args().collect();
		let ref program = args[0].clone();
		let mut opts = Options::new();
		opts.optopt("c", "challenges", "Comma separated list of challenges", "'1,3,5'");
		opts.optflag("h", "help", "Display this help and exit");
		// parse args provided into Matches struct
		let matches = match opts.parse(&args[1..]) {
			Ok(a) => a,
			Err(_) => return Some(ArgErr::GetOptsFailure),
		};
		// help output, catch so nothing futher happens
		if matches.opt_present("help") {
			let brief = format!("Usage: {} [options]\n", program);
			print!("{}", opts.usage(&brief));
			return Some(ArgErr::HelpSet);
		}
		self.challenges = vec![false; max_chals+1];
		// handle args
		if let Some(chals) = matches.opt_str("c") {
			let re = Regex::new(r"^(\d+,?)+$").unwrap();
			if ! re.is_match(&chals) {
				return Some(ArgErr::ReFailure);
			}
			for c in chals.split(",") {
				let d:usize = match usize::from_str_radix(c, 10) {
					Ok(d) => d,
					Err(_) => return Some(ArgErr::NonNumber),
				};
				if d>max_chals {
					return Some(ArgErr::InvalidChallenge);
				}
				self.challenges[d] = true;
			}
		}
		None
	}
}
#[allow(dead_code)]
pub enum ArgErr {
	GetOptsFailure,
	ReFailure,
	NonNumber,
	InvalidChallenge,
	InvalidArgs,
	HelpSet,
}
impl ArgErr {
	pub fn handle_error(self) -> i32 {
		match self {
			ArgErr::GetOptsFailure => write_err("Failed to capture arguments"),
			ArgErr::ReFailure => write_err("Challenge string did not match regex"),
			ArgErr::NonNumber => write_err("Challenge string contained a non-number"),
			ArgErr::InvalidChallenge => write_err("Challenge string contained an invalid challenge"),
			ArgErr::InvalidArgs => write_err("Provide the correct arguments"),
			ArgErr::HelpSet => return 0,
		};
		1 // return exit code 1
	}
}
fn write_err(err:&str) {
	println!("{}" , err);
}
