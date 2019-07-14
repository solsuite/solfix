extern crate solfix;

#[cfg(test)]
mod types_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{ NonTerminal, parse_type_name, ParseTree };

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

    /*** Parse Type Tests ***/

    #[test]
    fn elementary_type_test1() {
        let actual_tree = parse_type_name(&str_to_chars("address"), &mut 0);
        let expected_tree = *lex_4_25::Token::Address.to_boxed_leaf();
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn user_defined_type_test1() {
        let actual_tree = parse_type_name(&str_to_chars("Address.Enum"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::UserDefinedTypeName,
            leaves: vec![
                NonTerminal::Identifier(String::from("Address")).to_boxed_leaf(),
                NonTerminal::Identifier(String::from("Enum")).to_boxed_leaf()
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn mapping_test1() {
        let actual_tree = parse_type_name(&str_to_chars("mapping (uint256 => uint256)"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::Mapping,
            leaves: vec! [
                lex_4_25::Token::Uint256.to_boxed_leaf(),
                lex_4_25::Token::Uint256.to_boxed_leaf()
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn array_type_name_test1() {
        let actual_tree = parse_type_name(&str_to_chars("uint256[1 + 1]"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::ArrayTypeName,
            leaves: vec! [
                lex_4_25::Token::Uint256.to_boxed_leaf(),
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Plus),
                    vec![
                        lex_4_25::Token::DecimalNumber(String::from("1")).to_boxed_leaf(),
                        lex_4_25::Token::DecimalNumber(String::from("1")).to_boxed_leaf()
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn function_type_name_test1() {
        let actual_tree = parse_type_name(&str_to_chars("function () internal"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::FunctionTypeName,
            leaves: vec! [
                NonTerminal::FunctionTypeParameterList.to_boxed_leaf(),
                lex_4_25::Token::Internal.to_boxed_leaf()
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn function_type_name_test2() {
        let actual_tree = parse_type_name(&str_to_chars("function () internal pure"), &mut 0);
        let expected_tree = ParseTree {
            root: NonTerminal::FunctionTypeName,
            leaves: vec! [
                NonTerminal::FunctionTypeParameterList.to_boxed_leaf(),
                lex_4_25::Token::Internal.to_boxed_leaf(),
                NonTerminal::StateMutability(lex_4_25::Token::Pure).to_boxed_leaf()
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn function_type_name_test3() {
        let actual_tree = parse_type_name(
            &str_to_chars("function (uint256, bytes32) internal pure returns (bool) "),
            &mut 0
        );
        let expected_tree = ParseTree {
            root: NonTerminal::FunctionTypeName,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::FunctionTypeParameterList,
                    vec![
                        boxed_parse_tree(
                            NonTerminal::FunctionTypeParameter,
                            vec![
                                lex_4_25::Token::Uint256.to_boxed_leaf()
                            ]
                        ),
                        boxed_parse_tree(
                            NonTerminal::FunctionTypeParameter,
                            vec![
                                lex_4_25::Token::Bytes32.to_boxed_leaf()
                            ]
                        )
                    ]
                ),
                lex_4_25::Token::Internal.to_boxed_leaf(),
                NonTerminal::StateMutability(lex_4_25::Token::Pure).to_boxed_leaf(),
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Returns),
                    vec![
                        boxed_parse_tree(
                            NonTerminal::FunctionTypeParameterList,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::FunctionTypeParameter,
                                    vec![
                                        lex_4_25::Token::Bool.to_boxed_leaf()
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn function_type_name_test4() {
        let actual_tree = parse_type_name(
            &str_to_chars(
                "function (function (uint256) internal) external payable returns \
                (function (bool) external returns (uint256)) "
            ),
            &mut 0
        );
        let expected_tree = ParseTree {
            root: NonTerminal::FunctionTypeName,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::FunctionTypeParameterList,
                    vec![
                        boxed_parse_tree(
                            NonTerminal::FunctionTypeParameter,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::FunctionTypeName,
                                    vec! [
                                        boxed_parse_tree(
                                            NonTerminal::FunctionTypeParameterList,
                                            vec![
                                                boxed_parse_tree(
                                                    NonTerminal::FunctionTypeParameter,
                                                    vec![
                                                        lex_4_25::Token::Uint256.to_boxed_leaf()
                                                    ]
                                                )
                                            ]
                                        ),
                                        lex_4_25::Token::Internal.to_boxed_leaf()
                                    ]
                                )
                            ]
                        )
                    ]
                ),
                lex_4_25::Token::External.to_boxed_leaf(),
                NonTerminal::StateMutability(lex_4_25::Token::Payable).to_boxed_leaf(),
                boxed_parse_tree(
                    NonTerminal::Token(lex_4_25::Token::Returns),
                    vec![
                        boxed_parse_tree(
                            NonTerminal::FunctionTypeParameterList,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::FunctionTypeParameter,
                                    vec![
                                        boxed_parse_tree(
                                            NonTerminal::FunctionTypeName,
                                            vec! [
                                                boxed_parse_tree(
                                                    NonTerminal::FunctionTypeParameterList,
                                                    vec![
                                                        boxed_parse_tree(
                                                            NonTerminal::FunctionTypeParameter,
                                                            vec![
                                                                lex_4_25::Token::Bool.to_boxed_leaf()
                                                            ]
                                                        )
                                                    ]
                                                ),
                                                lex_4_25::Token::External.to_boxed_leaf(),
                                                boxed_parse_tree(
                                                    NonTerminal::Token(lex_4_25::Token::Returns),
                                                    vec![
                                                        boxed_parse_tree(
                                                            NonTerminal::FunctionTypeParameterList,
                                                            vec![
                                                                boxed_parse_tree(
                                                                    NonTerminal::FunctionTypeParameter,
                                                                    vec![
                                                                        lex_4_25::Token::Uint256.to_boxed_leaf()
                                                                    ]
                                                                )
                                                            ]
                                                        )
                                                    ]
                                                )
                                            ]
                                        )
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }
}
