extern crate solfix;

#[cfg(test)]
mod parser_integration_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{ parse, ParseNode, ParseTree };
    use std::fs;

    #[test]
    fn ownable_test() {
        let input = fs::read_to_string("./contracts/Ownable.sol")
            .expect("Test file not found: ./contracts/Ownable.sol");
        let actual_tree = parse(input);
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Pragma,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("solidity".to_string()).to_leaf()),
                        Box::new(lex_4_25::Token::BitwiseXor.to_leaf()),
                        Box::new(lex_4_25::Token::Version("0.4.25".to_string()).to_leaf())
                    ]
                },
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Ownable".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::StateVariable,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Address.to_leaf()),
                                        Box::new(lex_4_25::Token::Public.to_leaf()),
                                        Box::new(lex_4_25::Token::Identifier("owner".to_string()).to_leaf())
                                    ]
                                }),
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Modifier,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("onlyOwner".to_string()).to_leaf()),
                                        Box::new(ParseNode {
                                            node: lex_4_25::Token::OpenBrace,
                                            children: vec![
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::OpenParenthesis,
                                                    children: vec![
                                                        Box::new(lex_4_25::Token::Identifier("require".to_string()).to_leaf()),
                                                        Box::new(ParseNode {
                                                            node: lex_4_25::Token::OpenParenthesis,
                                                            children: vec![
                                                                Box::new(ParseNode {
                                                                    node: lex_4_25::Token::Equals,
                                                                    children: vec![
                                                                        Box::new(ParseNode {
                                                                            node: lex_4_25::Token::Dot,
                                                                            children: vec![
                                                                                Box::new(lex_4_25::Token::Identifier("msg".to_string()).to_leaf()),
                                                                                Box::new(lex_4_25::Token::Identifier("sender".to_string()).to_leaf())
                                                                            ]
                                                                        }),
                                                                        Box::new(lex_4_25::Token::Identifier("owner".to_string()).to_leaf())
                                                                    ]
                                                                })
                                                            ]
                                                        })
                                                    ]
                                                }),
                                                Box::new(lex_4_25::Token::Placeholder.to_leaf())
                                            ]
                                        })
                                    ]
                                }),
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    /*
    #[test]
    fn safe_math_test() {
        let input = fs::read_to_string("./contracts/SafeMath.sol")
            .expect("Test file not found: ./contracts/SafeMath.sol");
        let actual_tree = parse(input);
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Pragma,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("solidity".to_string()).to_leaf()),
                        Box::new(lex_4_25::Token::Version("^0.4.25".to_string()).to_leaf())
                    ]
                },
                ParseNode {
                    node: lex_4_25::Token::Library,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("SafeMath".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Function,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("add".to_string()).to_leaf()),
                                        Box::new(ParseNode {
                                            node: lex_4_25::Token::OpenParenthesis,
                                            children: vec![
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::Parameter,
                                                    children: vec![
                                                        Box::new(lex_4_25::Token::Uint256.to_leaf()),
                                                        Box::new(lex_4_25::Token::Identifier("a".to_string()).to_leaf())
                                                    ]
                                                }),
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::Parameter,
                                                    children: vec![
                                                        Box::new(lex_4_25::Token::Uint256.to_leaf()),
                                                        Box::new(lex_4_25::Token::Identifier("b".to_string()).to_leaf())
                                                    ]
                                                })
                                            ]
                                        }),
                                        Box::new(lex_4_25::Token::Internal.to_leaf()),
                                        Box::new(lex_4_25::Token::Pure.to_leaf()),
                                        Box::new(ParseNode {
                                            node: lex_4_25::Token::Returns,
                                            children: vec![
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::OpenParenthesis,
                                                    children: vec![
                                                        Box::new(lex_4_25::Token::Uint256.to_leaf())
                                                    ]
                                                })
                                            ]
                                        }),
                                        Box::new(ParseNode {
                                            node: lex_4_25::Token::OpenBrace,
                                            children: vec![
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::Assignment,
                                                    children: vec![

                                                    ]
                                                }),
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::OpenParenthesis,
                                                    children: vec![
                                                        Box::new(lex_4_25::Token::Identifier("require".to_string()).to_leaf()),
                                                        Box::new(ParseNode {
                                                            node: lex_4_25::Token::OpenParenthesis,
                                                            children: vec![

                                                            ]
                                                        })
                                                    ]
                                                }),
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::Return,
                                                    children: vec![
                                                        Box::new(lex_4_25::Token::Identifier("c".to_string()).to_leaf())
                                                    ]
                                                })
                                            ]
                                        })
                                    ]
                                }),
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Function,
                                    children: vec![]
                                }),
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Function,
                                    children: vec![]
                                }),
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Function,
                                    children: vec![]
                                }),
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Function,
                                    children: vec![]
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
