extern crate solfix;

#[cfg(test)]
mod parser_integration_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{ NonTerminal, parse, ParseTree };
    use std::fs;

    /*** Helpers ***/

    fn boxed_parse_tree(root: NonTerminal, leaves: Vec<Box<ParseTree>>) -> Box<ParseTree> {
        Box::new(ParseTree {
            root: root,
            leaves: leaves,
        })
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

    #[test]
    fn ownable_test() {
        let input = fs::read_to_string("./contracts/Ownable.sol")
            .expect("Test file not found: ./contracts/Ownable.sol");
        let actual_tree = parse(input);
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::PragmaDirective,
                    vec![
                        NonTerminal::Identifier(String::from("solidity")).to_boxed_leaf(),
                        lex_4_25::Token::BitwiseXor.to_boxed_leaf(),
                        lex_4_25::Token::Version(String::from("0.4.25")).to_boxed_leaf()
                    ]
                ),
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Ownable")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::StateVariableDeclaration,
                                    vec![
                                        lex_4_25::Token::Address.to_boxed_leaf(),
                                        lex_4_25::Token::Public.to_boxed_leaf(),
                                        NonTerminal::Identifier(String::from("owner")).to_boxed_leaf()
                                    ]
                                ),
                                boxed_parse_tree(
                                    NonTerminal::ModifierDefinition,
                                    vec![
                                        NonTerminal::Identifier(String::from("onlyOwner")).to_boxed_leaf(),
                                        boxed_parse_tree(
                                            NonTerminal::Block,
                                            vec![
                                                boxed_parse_tree(
                                                    NonTerminal::FunctionCall,
                                                    vec![
                                                        NonTerminal::Identifier(String::from("require")).to_boxed_leaf(),
                                                        boxed_parse_tree(
                                                            NonTerminal::FunctionCallArguments,
                                                            vec![
                                                                boxed_parse_tree(
                                                                    NonTerminal::ExpressionList,
                                                                    vec![
                                                                        boxed_parse_tree(
                                                                            NonTerminal::Token(lex_4_25::Token::Equals),
                                                                            vec![
                                                                                boxed_parse_tree(
                                                                                    NonTerminal::MemberAccess,
                                                                                    vec![
                                                                                        NonTerminal::Identifier(String::from("msg")).to_boxed_leaf(),
                                                                                        NonTerminal::Identifier(String::from("sender")).to_boxed_leaf()
                                                                                    ]
                                                                                ),
                                                                                NonTerminal::Identifier(String::from("owner")).to_boxed_leaf()
                                                                            ]
                                                                        )
                                                                    ]
                                                                )
                                                            ]
                                                        )
                                                    ]
                                                ),
                                                lex_4_25::Token::Placeholder.to_boxed_leaf()
                                            ]
                                        )
                                    ]
                                ),
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    /*
    #[test]
    fn safe_math_test() {
        let input = fs::read_to_string("./contracts/SafeMath.sol")
            .expect("Test file not found: ./contracts/SafeMath.sol");
        let actual_tree = parse(input);
        let expected_tree = ParseTree {
            leaves: vec![
                ParseTree {
                    root: lex_4_25::Token::Pragma,
                    leaves: vec![
                        Box::new(lex_4_25::Token::Identifier("solidity".to_string()).to_leaf()),
                        Box::new(lex_4_25::Token::Version("^0.4.25".to_string()).to_leaf())
                    ]
                },
                ParseTree {
                    root: lex_4_25::Token::Library,
                    leaves: vec![
                        Box::new(lex_4_25::Token::Identifier("SafeMath".to_string()).to_leaf()),
                        Box::new(ParseTree {
                            root: lex_4_25::Token::OpenBrace,
                            leaves: vec![
                                Box::new(ParseTree {
                                    root: lex_4_25::Token::Function,
                                    leaves: vec![
                                        Box::new(lex_4_25::Token::Identifier("add".to_string()).to_leaf()),
                                        Box::new(ParseTree {
                                            root: lex_4_25::Token::OpenParenthesis,
                                            leaves: vec![
                                                Box::new(ParseTree {
                                                    root: lex_4_25::Token::Parameter,
                                                    leaves: vec![
                                                        Box::new(lex_4_25::Token::Uint256.to_leaf()),
                                                        Box::new(lex_4_25::Token::Identifier("a".to_string()).to_leaf())
                                                    ]
                                                }),
                                                Box::new(ParseTree {
                                                    root: lex_4_25::Token::Parameter,
                                                    leaves: vec![
                                                        Box::new(lex_4_25::Token::Uint256.to_leaf()),
                                                        Box::new(lex_4_25::Token::Identifier("b".to_string()).to_leaf())
                                                    ]
                                                })
                                            ]
                                        }),
                                        Box::new(lex_4_25::Token::Internal.to_leaf()),
                                        Box::new(lex_4_25::Token::Pure.to_leaf()),
                                        Box::new(ParseTree {
                                            root: lex_4_25::Token::Returns,
                                            leaves: vec![
                                                Box::new(ParseTree {
                                                    root: lex_4_25::Token::OpenParenthesis,
                                                    leaves: vec![
                                                        Box::new(lex_4_25::Token::Uint256.to_leaf())
                                                    ]
                                                })
                                            ]
                                        }),
                                        Box::new(ParseTree {
                                            root: lex_4_25::Token::OpenBrace,
                                            leaves: vec![
                                                Box::new(ParseTree {
                                                    root: lex_4_25::Token::Assignment,
                                                    leaves: vec![

                                                    ]
                                                }),
                                                Box::new(ParseTree {
                                                    root: lex_4_25::Token::OpenParenthesis,
                                                    leaves: vec![
                                                        Box::new(lex_4_25::Token::Identifier("require".to_string()).to_leaf()),
                                                        Box::new(ParseTree {
                                                            root: lex_4_25::Token::OpenParenthesis,
                                                            leaves: vec![

                                                            ]
                                                        })
                                                    ]
                                                }),
                                                Box::new(ParseTree {
                                                    root: lex_4_25::Token::Return,
                                                    leaves: vec![
                                                        Box::new(lex_4_25::Token::Identifier("c".to_string()).to_leaf())
                                                    ]
                                                })
                                            ]
                                        })
                                    ]
                                }),
                                Box::new(ParseTree {
                                    root: lex_4_25::Token::Function,
                                    leaves: vec![]
                                }),
                                Box::new(ParseTree {
                                    root: lex_4_25::Token::Function,
                                    leaves: vec![]
                                }),
                                Box::new(ParseTree {
                                    root: lex_4_25::Token::Function,
                                    leaves: vec![]
                                }),
                                Box::new(ParseTree {
                                    root: lex_4_25::Token::Function,
                                    leaves: vec![]
                                })
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }
    */
}
