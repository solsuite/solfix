extern crate solfix;

#[cfg(test)]
mod style_integration_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{ parse, ParseNode, ParseTree };
    use solfix::style::stylize;
    use std::fs;

    #[test]
    fn empty_style_test() {
        let input = fs::read_to_string("./contracts/BadEmpty.sol")
            .expect("Test file not found: ./contracts/BadEmpty.sol");
        let expected = fs::read_to_string("./contracts/Empty.sol")
            .expect("Test file not found: ./contracts/Empty.sol");
        let actual = stylize(parse(input));
        assert_eq!(expected, actual);
    }

    #[test]
    fn value_style_test() {
        let input = fs::read_to_string("./contracts/BadValue.sol")
            .expect("Test file not found: ./contracts/BadValue.sol");
        let expected = fs::read_to_string("./contracts/Value.sol")
            .expect("Test file not found: ./contracts/Value.sol");
        let actual = stylize(parse(input));
        assert_eq!(expected, actual);
    }

    #[test]
    fn ownable_style_test() {
        let input = fs::read_to_string("./contracts/BadOwnable.sol")
            .expect("Test file not found: ./contracts/BadOwnable.sol");
        let expected = fs::read_to_string("./contracts/Ownable.sol")
            .expect("Test file not found: ./contracts/Ownable.sol");
        let actual = stylize(parse(input));
        assert_eq!(expected, actual);
    }
}
