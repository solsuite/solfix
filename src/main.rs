use std::env;
use std::fs;
mod lex_4_25;
mod parse_4_25;
use crate::parse_4_25::parse;

fn main() {
    let name = env::args().nth(1).expect("Usage: solidity-fix FILE_NAME");
    let input = fs::read_to_string(name).expect("Unable to open input file");
    println!("{:?}", parse(input));
}
