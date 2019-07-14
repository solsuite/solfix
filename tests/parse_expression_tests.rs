extern crate solfix;

#[cfg(test)] mod parse_expression_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{ NonTerminal, parse_expression, ParseTree };

    /*** Helpers ***/

    fn boxed_parse_tree(root: NonTerminal, leaves: Vec<Box<ParseTree>>) -> Box<ParseTree> {
        Box::new(ParseTree {
            root: root,
            leaves: leaves,
        })
    }

    fn str_to_chars(string: &str) -> Vec<char> {
        let chars = String::from(string).chars().collect::<Vec<char>>();
        chars
    }

    trait TestUtils {
        fn to_boxed_leaf(self) -> Box<ParseTree>;
    }

    impl TestUtils for lex_4_25::Token {
        fn to_boxed_leaf(self) -> Box<ParseTree> {
            Box::new(NonTerminal::Token(self).to_leaf())
        }
    }

    impl TestUtils for NonTerminal {
        fn to_boxed_leaf(self) -> Box<ParseTree> {
            Box::new(self.to_leaf())
        }
    }

    trait PrettyAssertions<T> {
        fn assert_equal_pretty(self, other: T);
    }

    macro_rules! assert_eq_pretty {
        ($i1:ident, $i2:ident) => {
            assert_eq!($i1, $i2, "Expected\n========\n{:#?}\nActual\n======\n{:#?}", $i1, $i2);
        }
    }

    /*** Arithmetic ***/

    #[test]
    fn addition_parsing_test() {
        let actual_tree = parse_expression(&str_to_chars("1500 + 0x000"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::Token(lex_4_25::Token::Plus),
            leaves: vec![
                lex_4_25::Token::DecimalNumber(String::from("1500")).to_boxed_leaf(),
                lex_4_25::Token::HexNumber(String::from("0x000")).to_boxed_leaf(),
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn multiplication_parsing_test() {
        let actual_tree = parse_expression(&str_to_chars("1500 * 0x000"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::Token(lex_4_25::Token::Multiply),
            leaves: vec![
                lex_4_25::Token::DecimalNumber(String::from("1500")).to_boxed_leaf(),
                lex_4_25::Token::HexNumber(String::from("0x000")).to_boxed_leaf(),
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn arithmetic_parsing_test1() {
        let actual_tree = parse_expression(&str_to_chars("(800 + 1500) * 0x000"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::Token(lex_4_25::Token::Multiply),
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Plus),
                    vec![
                        lex_4_25::Token::DecimalNumber(String::from("800")).to_boxed_leaf(),
                        lex_4_25::Token::DecimalNumber(String::from("1500")).to_boxed_leaf(),
                    ]
                ),
                lex_4_25::Token::HexNumber(String::from("0x000")).to_boxed_leaf(),
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn arithmetic_parsing_test2() {
        let actual_tree = parse_expression(&str_to_chars("800 + 1500 * 0x000"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::Token(lex_4_25::Token::Plus),
            leaves: vec![
                lex_4_25::Token::DecimalNumber(String::from("800")).to_boxed_leaf(),
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Multiply),
                    vec![
                        lex_4_25::Token::DecimalNumber(String::from("1500")).to_boxed_leaf(),
                        lex_4_25::Token::HexNumber(String::from("0x000")).to_boxed_leaf(),
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn arithmetic_parsing_test3() {
        let actual_tree = parse_expression(&str_to_chars("800 * 1500 + 0x000"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::Token(lex_4_25::Token::Plus),
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Multiply),
                    vec![
                        lex_4_25::Token::DecimalNumber(String::from("800")).to_boxed_leaf(),
                        lex_4_25::Token::DecimalNumber(String::from("1500")).to_boxed_leaf(),
                    ]
                ),
                lex_4_25::Token::HexNumber(String::from("0x000")).to_boxed_leaf(),
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn arithmetic_parsing_test4() {
        let actual_tree = parse_expression(&str_to_chars("800 * 1500 + 0x000 * 0x800"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::Token(lex_4_25::Token::Plus),
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Multiply),
                    vec![
                        lex_4_25::Token::DecimalNumber(String::from("800")).to_boxed_leaf(),
                        lex_4_25::Token::DecimalNumber(String::from("1500")).to_boxed_leaf(),
                    ]
                ),
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Multiply),
                    vec![
                        lex_4_25::Token::HexNumber(String::from("0x000")).to_boxed_leaf(),
                        lex_4_25::Token::HexNumber(String::from("0x800")).to_boxed_leaf(),
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn arithmetic_parsing_test5() {
        let actual_tree = parse_expression(&str_to_chars("1 - 800 * 1500 + 0x000 * 0x800 / 5 ** 1800"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::Token(lex_4_25::Token::Plus),
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Minus),
                    vec![
                        lex_4_25::Token::DecimalNumber(String::from("1")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::Token(lex_4_25::Token::Multiply),
                            vec![
                                lex_4_25::Token::DecimalNumber(String::from("800")).to_boxed_leaf(),
                                lex_4_25::Token::DecimalNumber(String::from("1500")).to_boxed_leaf(),
                            ]
                        )
                    ]
                ),
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Divide),
                    vec![
                        boxed_parse_tree(
                            NonTerminal::Token(lex_4_25::Token::Multiply),
                            vec![
                                lex_4_25::Token::HexNumber(String::from("0x000")).to_boxed_leaf(),
                                lex_4_25::Token::HexNumber(String::from("0x800")).to_boxed_leaf()
                            ]
                        ),
                        boxed_parse_tree(
                            NonTerminal::Token(lex_4_25::Token::Power),
                            vec![
                                lex_4_25::Token::DecimalNumber(String::from("5")).to_boxed_leaf(),
                                lex_4_25::Token::DecimalNumber(String::from("1800")).to_boxed_leaf()
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    /*** Function call ***/

    #[test]
    fn function_call_test1() {
        let actual_tree = parse_expression(&str_to_chars("Identifier()"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::FunctionCall,
            leaves: vec![
                NonTerminal::Identifier(String::from("Identifier")).to_boxed_leaf(),
                boxed_parse_tree(
                    NonTerminal::FunctionCallArguments,
                    vec![
                        boxed_parse_tree(
                            NonTerminal::ExpressionList,
                            vec![
                                NonTerminal::Expression.to_boxed_leaf()
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn function_call_test2() {
        let actual_tree = parse_expression(&str_to_chars("add(1, 2)"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::FunctionCall,
            leaves: vec![
                NonTerminal::Identifier(String::from("add")).to_boxed_leaf(),
                boxed_parse_tree(
                    NonTerminal::FunctionCallArguments,
                    vec![
                        boxed_parse_tree(
                            NonTerminal::ExpressionList,
                            vec![
                                lex_4_25::Token::DecimalNumber(String::from("1")).to_boxed_leaf(),
                                lex_4_25::Token::DecimalNumber(String::from("2")).to_boxed_leaf()
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }
}
