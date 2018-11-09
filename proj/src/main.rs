extern crate clap;
extern crate colored;
extern crate regex;

use clap::{App, Arg};

pub mod assert;
pub mod env;
pub mod parser;
pub mod test_result;
pub mod tools;

fn main() -> std::io::Result<()> {
    let matches = App::new("Inline Rust Testing for Native Javascript")
        .version("0.1")
        .author("Pontus L. <pontus.laestadius@gmail.com>")
        .about("Executes Rust-like testing on JavaScript files.")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("INPUT").unwrap();
    env::read(filename)?;
    Ok(())
}
