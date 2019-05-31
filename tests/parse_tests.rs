extern crate solfix;

#[cfg(test)]
mod parser_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{parse_expression, ParseNode};

    #[test]
    fn addition_parsing_test() {
        let addition = String::from("1500 + 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&addition, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        let left = String::from("1500");
        let right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::DecimalNumber(left) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(left), actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::HexNumber(right) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(right), actual)
        }
    }

    #[test]
    fn multiplication_parsing_test() {
        let multiplication = String::from("1500 * 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&multiplication, cur);
        match node.node {
            lex_4_25::Token::Multiply => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 0);
        assert_eq!(node.children[1].children.len(), 0);
        let left = String::from("1500");
        let right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::DecimalNumber(left) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(left), actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::HexNumber(right) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(right), actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test1() {
        let multiplication = String::from("(800 + 1500) * 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&multiplication, cur);
        println!("{:?}", node);
        match node.node {
            lex_4_25::Token::Multiply => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 2);
        assert_eq!(node.children[1].children.len(), 0);
        let inner_left = String::from("800");
        let inner_right = String::from("1500");
        let right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::Plus => {
                match &node.children[0].children[0].node {
                    lex_4_25::Token::DecimalNumber(inner_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(inner_left), actual)
                }
                match &node.children[0].children[1].node {
                    lex_4_25::Token::DecimalNumber(inner_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(inner_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::HexNumber(right) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(right), actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test2() {
        let multiplication = String::from("800 + 1500 * 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&multiplication, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 0);
        assert_eq!(node.children[1].children.len(), 2);
        let left = String::from("800");
        let inner_left = String::from("1500");
        let inner_right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::DecimalNumber(inner_left) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(left), actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::Multiply => {
                match &node.children[1].children[0].node {
                    lex_4_25::Token::DecimalNumber(inner_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(inner_left), actual)
                }
                match &node.children[1].children[1].node {
                    lex_4_25::Token::HexNumber(inner_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(inner_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test3() {
        let arithmetic = String::from("800 * 1500 + 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&arithmetic, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 2);
        assert_eq!(node.children[1].children.len(), 0);
        let inner_left = String::from("800");
        let inner_right = String::from("1500");
        let right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::Multiply => {
                match &node.children[0].children[0].node {
                    lex_4_25::Token::DecimalNumber(inner_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(inner_left), actual)
                }
                match &node.children[0].children[1].node {
                    lex_4_25::Token::DecimalNumber(inner_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(inner_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::HexNumber(right) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(right), actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test4() {
        let arithmetic = String::from("800 * 1500 + 0x000 * 0x800").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&arithmetic, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 2);
        assert_eq!(node.children[1].children.len(), 2);
        let left_left = String::from("800");
        let left_right = String::from("1500");
        let right_left = String::from("0x000");
        let right_right = String::from("0x800");
        match &node.children[0].node {
            lex_4_25::Token::Multiply => {
                match &node.children[0].children[0].node {
                    lex_4_25::Token::DecimalNumber(left_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(left_left), actual)
                }
                match &node.children[0].children[1].node {
                    lex_4_25::Token::DecimalNumber(left_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(left_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::Multiply => {
                match &node.children[1].children[0].node {
                    lex_4_25::Token::HexNumber(right_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(right_left), actual)
                }
                match &node.children[1].children[1].node {
                    lex_4_25::Token::HexNumber(right_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(right_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test5() {
        let arithmetic = String::from("1 - 800 * 1500 + 0x000 * 0x800 / 5 ** 1800").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&arithmetic, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 2);
        assert_eq!(node.children[1].children.len(), 2);
        match &node.children[0].node {
            lex_4_25::Token::Minus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::Divide => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Divide, actual)
        }
        let one = String::from("1");
        match &node.children[0].children[0].node {
            lex_4_25::Token::DecimalNumber(one) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(one), actual)
        }
        match &node.children[0].children[1].node {
            lex_4_25::Token::Multiply => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].children[0].node {
            lex_4_25::Token::Multiply => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].children[1].node {
            lex_4_25::Token::Power => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Power, actual)
        }
        assert_eq!(node.children[0].children[0].children.len(), 0);
        assert_eq!(node.children[0].children[1].children.len(), 2);
        assert_eq!(node.children[1].children[0].children.len(), 2);
        assert_eq!(node.children[1].children[1].children.len(), 2);
        let eight_hundred = String::from("800");
        match &node.children[0].children[1].children[0].node {
            lex_4_25::Token::DecimalNumber(eight_hundred) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(eight_hundred), actual)
        }
        let one_thousand_five_hundred = String::from("1500");
        match &node.children[0].children[1].children[1].node {
            lex_4_25::Token::DecimalNumber(one_thousand_five_hundred) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(one_thousand_five_hundred), actual)
        }
        let zero = String::from("0x000");
        match &node.children[1].children[0].children[0].node {
            lex_4_25::Token::HexNumber(zero) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(zero), actual)
        }
        let two_thousand_forty_eight = String::from("0x800");
        match &node.children[1].children[0].children[1].node {
            lex_4_25::Token::HexNumber(two_thousand_forty_eight) => (),
            actual => panic!("expected: {:?} | actual: {:?}", lex_4_25::Token::HexNumber(two_thousand_forty_eight), actual)
        }
        let five = String::from("5");
        match &node.children[1].children[1].children[0].node {
            lex_4_25::Token::DecimalNumber(five) => (),
            actual => panic!("expected: {:?} | actual: {:?}", lex_4_25::Token::DecimalNumber(five), actual)
        }
        let one_thousand_eight_hundred = String::from("1800");
        match &node.children[1].children[1].children[1].node {
            lex_4_25::Token::DecimalNumber(one_thousand_eight_hundred) => (),
            actual => panic!("expected: {:?} | actual: {:?}", lex_4_25::Token::DecimalNumber(one_thousand_eight_hundred), actual)
        }
    }
}
