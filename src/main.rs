use std::env;
use std::fs;
use solfix::parse_4_25::parse;
use solfix::codegen::generate;

fn main() {
    let usage = "Usage: solidity-fix SOURCE TARGET";
    let source = env::args().nth(1).expect(usage);
    let target = env::args().nth(2).expect(usage);
    let input = fs::read_to_string(source).expect("Unable to open input file");
    let result = generate(parse(input));
    fs::write(target, result).expect("Unable to write file");
}
