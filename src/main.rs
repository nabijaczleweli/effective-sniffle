extern crate clap;
#[macro_use]
extern crate nom;

mod options;
pub mod par;

pub use options::Options;

use std::fs::File;
use std::io::{/*self, */Read/*, Write*/};

fn main() {
    let opts = Options::parse();
    println!("{:?}", opts);

    let mut buf = Vec::new();
    File::open(opts.input).unwrap().read_to_end(&mut buf).unwrap();
    println!("{:?}", par::parse(&buf[..]));
    /*match par::parse(&buf[..]) {
        Ok(tree) => println!("{:?}", tree),
        Err(error) => {
            let _ = io::stderr().write_fmt(format_args!("{:?}", error));
            return;
        }
    }*/
}
