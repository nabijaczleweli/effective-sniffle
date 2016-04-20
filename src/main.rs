extern crate clap;
extern crate parse;

mod options;
pub mod par;

pub use options::Options;

use std::fs::File;
use std::io::Read;

fn main() {
	let opts = Options::parse();
	println!("{:?}", opts);

	let mut buf = Vec::new();
	File::open(opts.input).unwrap().read_to_end(&mut buf).unwrap();
	println!("{:?}", par::parse(&buf[..]));
}
