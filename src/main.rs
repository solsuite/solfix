use std::env;
use std::fs;
use solfix::parse_4_25::parse;
use solfix::style::stylize;

fn main() {
    let usage = "Usage: solfix style SOURCE [TARGET]\n              ast SOURCE [TARGET]";
    match env::args().nth(1) {
        Some(ref style) if style == "style" => {
            let source = env::args().nth(2).expect(usage);
            let input = fs::read_to_string(source).expect("Unable to find SOURCE");
            match env::args().nth(3) {
                Some(target) => fs::write(target, stylize(parse(input))).expect("Unable to write file to TARGET"),
                None => println!("{}", stylize(parse(input)))
            }
        }
        Some(ref ast) if ast == "ast" => {
            let source = env::args().nth(2).expect(usage);
            let input = fs::read_to_string(source).expect("Unable to find SOURCE");
            match env::args().nth(3) {
                _ => println!("{:?}", parse(input))
            }
        }
        _ => println!("{}", usage)
    }
}
