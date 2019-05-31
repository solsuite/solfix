use std::env;
use std::fs;
use solfix::parse_4_25::parse;

fn main() {
    let name = env::args().nth(1).expect("Usage: solidity-fix FILE_NAME");
    let input = fs::read_to_string(name).expect("Unable to open input file");
    println!("{:?}", parse(input));
}
