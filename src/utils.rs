use super::lex_4_25;
use super::parse_4_25;

pub mod test_utils {
    use super::*;

    pub mod lexer {
        use super::lex_4_25;

        /**
         * Fail a lexer test, given some expected and actual token.
         */
        pub fn fail_test(expect: lex_4_25::Token, actual: lex_4_25::Token) {
            println!("Expected: {:?} | Actual: {:?}", expect, actual);
        }

        /**
         * Advance cur in s using next_token, and check that the return matches
         * the expected Token, t. If not, the test fails.
         */
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

        /**
         * Returns a Token as a ParseNode with no children
         */
        pub fn as_leaf(t: lex_4_25::Token) -> Box<parse_4_25::ParseNode> {
            Box::new(t.to_leaf())
        }

        /**
         * Returns a ParseNode with the given Token serving as the node, and
         * a passed-in array of children as the node's children.
         */
        pub fn as_node(node: lex_4_25::Token, children: Vec<Box<parse_4_25::ParseNode>>) -> Box<parse_4_25::ParseNode> {
            Box::new(parse_4_25::ParseNode {
                node,
                children
            })
        }

        /**
         * Returns a ParseNode with the given Token and children, but does
         * not wrap the ParseNode in a Box
         */
        pub fn as_node_raw(node: lex_4_25::Token, children: Vec<Box<parse_4_25::ParseNode>>) -> parse_4_25::ParseNode {
            parse_4_25::ParseNode {
                node,
                children
            }
        }

        /**
         * Given an array of children, returns their ParseTree
         */
        pub fn as_tree(children: Vec<parse_4_25::ParseNode>) -> parse_4_25::ParseTree {
            parse_4_25::ParseTree {
                children
            }
        }

        /**
         * Shorthand for parse(String::from("str-input")). Calls
         * parse_4_25::parse and returns the resulting ParseTree
         */
        pub fn parse_str(string: &str) -> parse_4_25::ParseTree {
            parse_4_25::parse(String::from(string))
        }

        /**
         * Given two ParseTrees, checks for equality. If unequal,
         * panics and prints the prettified trees.
         */
        pub fn expect_tree_eq(expect: parse_4_25::ParseTree, actual: parse_4_25::ParseTree) {
            match expect == actual {
                true => (),
                false => panic!("\nExpected: {:#?} \nActual: {:#?}\n", expect, actual)
            }
        }

        /**
         * Given two ParseNodes, checks for equality. If unequal,
         * panics and prints the prettified nodes.
         */
        pub fn expect_node_eq(expect: parse_4_25::ParseNode, actual: parse_4_25::ParseNode) {
            match expect == actual {
                true => (),
                false => panic!("\nExpected: {:#?} \nActual: {:#?}\n", expect, actual)
            }
        }
    }
}