extern crate clap;
#[macro_use]
extern crate peggler;

mod options;
pub mod par;

pub use options::Options;

use std::fs::File;
use std::io::{self, Read, Write};

fn main() {
	let opts = Options::parse();
	println!("{:?}", opts);

	let mut buf = Vec::new();
	File::open(opts.input).unwrap().read_to_end(&mut buf).unwrap();
	match par::parse(&buf[..]) {
        Ok(tree) => println!("{:?}", tree),
		Err(error) => {
            let _ = io::stderr().write(error.as_bytes());
            return;
        }
	}
}
