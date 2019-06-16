use super::lex_4_25;
use super::parse_4_25;

pub mod test_utils {
    use super::*;

    pub mod lexer {
        use super::lex_4_25;

        pub fn fail_test(expect: lex_4_25::Token, actual: lex_4_25::Token) {
            println!("Expected: {:?} | Actual: {:?}", expect, actual);
        }

        pub fn expect_next_token(s: &Vec<char>, cur: &mut usize, t: lex_4_25::Token) {
            match lex_4_25::next_token(&s, cur) {
                ref next if *next == t => (),
                actual => fail_test(t, actual)
            };
        }
    }

    pub mod parser {
        use super::lex_4_25;
        use super::parse_4_25;

        pub fn as_leaf(t: lex_4_25::Token) -> Box<parse_4_25::ParseNode> {
            Box::new(t.to_leaf())
        }

        pub fn as_node(node: lex_4_25::Token, children: Vec<Box<parse_4_25::ParseNode>>) -> Box<parse_4_25::ParseNode> {
            Box::new(parse_4_25::ParseNode {
                node,
                children
            })
        }

        pub fn expect_eq(expect: parse_4_25::ParseTree, actual: parse_4_25::ParseTree) {
            match expect == actual {
                true => (),
                false => panic!("\nExpected: {:#?} \nActual: {:#?}", expect, actual)
            }
        }
    }
}