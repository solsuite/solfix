extern crate solfix;

#[cfg(test)]
mod parse_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{ parse, ParseNode, ParseTree };

    /*** Pragma ***/

    #[test]
    fn solidity_4_25_pragma_test1() {
        let actual_tree = parse(String::from("pragma solidity 0.4.25;"));
        let expected_tree = ParseTree { 
            children: vec![ 
                ParseNode {
                    node: lex_4_25::Token::Pragma,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("solidity".to_string()).to_leaf()),
                        Box::new(lex_4_25::Token::Version("0.4.25".to_string()).to_leaf())
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn solidity_4_25_pragma_test2() {
        let actual_tree = parse(String::from("pragma solidity ^0.4.25;"));
        let expected_tree = ParseTree { 
            children: vec![ 
                ParseNode {
                    node: lex_4_25::Token::Pragma,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("solidity".to_string()).to_leaf()),
                        Box::new(lex_4_25::Token::Version("^0.4.25".to_string()).to_leaf())
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    /*** Contract ***/

    #[test]
    fn contract_test1() {
        let actual_tree = parse(String::from("contract A {}"));
        let expected_tree = ParseTree { 
            children: vec![ 
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf()),
                        Box::new(lex_4_25::Token::OpenBrace.to_leaf())
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_test2() {
        let actual_tree = parse(String::from("contract B is A {}"));
        let expected_tree = ParseTree { 
            children: vec![ 
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("B".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::Is,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::OpenParenthesis,
                                    children: vec![
                                        Box::new(ParseNode {
                                            node: lex_4_25::Token::UserDefinedTypeName,
                                            children: vec![
                                                Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf())
                                            ]
                                        })
                                    ]
                                })
                            ]
                        }),
                        Box::new(lex_4_25::Token::OpenBrace.to_leaf())
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_enum_test1() { 
        let actual_tree = parse(String::from("contract Enum { enum Foo { } }"));
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Enum".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Enum,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("Foo".to_string()).to_leaf()),
                                        Box::new(lex_4_25::Token::OpenBrace.to_leaf())
                                    ]
                                })
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_enum_test2() { 
        let actual_tree = parse(String::from("contract Enum { enum Foo { Bar, Baz } }"));
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Enum".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Enum,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("Foo".to_string()).to_leaf()),
                                        Box::new(ParseNode {
                                            node: lex_4_25::Token::OpenBrace,
                                            children: vec![
                                                Box::new(lex_4_25::Token::Identifier("Bar".to_string()).to_leaf()),
                                                Box::new(lex_4_25::Token::Identifier("Baz".to_string()).to_leaf())
                                            ]
                                        })
                                    ]
                                })
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_event_test1() { 
        let actual_tree = parse(String::from("contract Event { event emptyEvent(); }"));
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Event".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Event,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("emptyEvent".to_string()).to_leaf()),
                                        Box::new(lex_4_25::Token::OpenParenthesis.to_leaf())
                                    ]
                                }) 
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_event_test2() { 
        let actual_tree = parse(String::from("contract Event { event Transfer(address indexed from, address indexed to, uint256 indexed value); }"));
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Event".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Event,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("Transfer".to_string()).to_leaf()),
                                        Box::new(ParseNode {
                                            node: lex_4_25::Token::OpenParenthesis,
                                            children: vec![
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::EventParameter,
                                                    children: vec![
                                                        Box::new(lex_4_25::Token::Address.to_leaf()),
                                                        Box::new(lex_4_25::Token::Indexed.to_leaf()),
                                                        Box::new(lex_4_25::Token::Identifier("from".to_string()).to_leaf())
                                                    ]
                                                }),
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::EventParameter,
                                                    children: vec![
                                                        Box::new(lex_4_25::Token::Address.to_leaf()),
                                                        Box::new(lex_4_25::Token::Indexed.to_leaf()),
                                                        Box::new(lex_4_25::Token::Identifier("to".to_string()).to_leaf())
                                                    ]
                                                }),
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::EventParameter,
                                                    children: vec![
                                                        Box::new(lex_4_25::Token::Uint256.to_leaf()),
                                                        Box::new(lex_4_25::Token::Indexed.to_leaf()),
                                                        Box::new(lex_4_25::Token::Identifier("value".to_string()).to_leaf())
                                                    ]
                                                })
                                            ]
                                        })
                                    ]
                                })
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_function_test1() { 
        let actual_tree = parse(String::from("contract Function { function doNothing() internal pure { } }"));
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Function".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Function,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("doNothing".to_string()).to_leaf()),
                                        Box::new(lex_4_25::Token::OpenParenthesis.to_leaf()),
                                        Box::new(lex_4_25::Token::Internal.to_leaf()),
                                        Box::new(lex_4_25::Token::Pure.to_leaf()),
                                        Box::new(lex_4_25::Token::OpenBrace.to_leaf())
                                    ]
                                })
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_function_test2() { 
        let actual_tree = parse(String::from("contract Function { function emitEvent() internal { emit someEvent(1 + 1); } }"));
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Function".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Function,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("emitEvent".to_string()).to_leaf()),
                                        Box::new(lex_4_25::Token::OpenParenthesis.to_leaf()),
                                        Box::new(lex_4_25::Token::Internal.to_leaf()),
                                        Box::new(ParseNode {
                                            node: lex_4_25::Token::OpenBrace,
                                            children: vec![
                                                Box::new(ParseNode {
                                                    node: lex_4_25::Token::Emit,
                                                    children: vec![
                                                        Box::new(ParseNode {
                                                            node: lex_4_25::Token::OpenParenthesis,
                                                            children: vec![
                                                                Box::new(lex_4_25::Token::Identifier("someEvent".to_string()).to_leaf()),
                                                                Box::new(ParseNode {
                                                                    node: lex_4_25::Token::OpenParenthesis,
                                                                    children: vec![
                                                                        Box::new(ParseNode {
                                                                            node: lex_4_25::Token::Plus,
                                                                            children: vec![
                                                                                Box::new(lex_4_25::Token::DecimalNumber("1".to_string()).to_leaf()),
                                                                                Box::new(lex_4_25::Token::DecimalNumber("1".to_string()).to_leaf())
                                                                            ]
                                                                        })
                                                                    ]
                                                                })
                                                            ]
                                                        })
                                                    ]
                                                }),
                                            ]
                                        })
                                    ]
                                })
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_modifier_test1() { 
        let actual_tree = parse(String::from("contract Modifier { modifier doNothing { _;} }"));
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Modifier".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Modifier,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("doNothing".to_string()).to_leaf()),
                                        Box::new(ParseNode {
                                            node: lex_4_25::Token::OpenBrace,
                                            children: vec![ Box::new(lex_4_25::Token::Placeholder.to_leaf()) ]
                                        })
                                    ]
                                })
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_using_for_test1() { 
        let actual_tree = parse(String::from("contract Using { using SafeMath for uint256; }"));
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Using".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Using,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("SafeMath".to_string()).to_leaf()),
                                        Box::new(lex_4_25::Token::Uint256.to_leaf())
                                    ]
                                })
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_using_for_test2() {
        let actual_tree = parse(String::from("contract Using { using SafeMath for *; }"));
        let expected_tree = ParseTree {
            children: vec![
                ParseNode {
                    node: lex_4_25::Token::Contract,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("Using".to_string()).to_leaf()),
                        Box::new(ParseNode {
                            node: lex_4_25::Token::OpenBrace,
                            children: vec![
                                Box::new(ParseNode {
                                    node: lex_4_25::Token::Using,
                                    children: vec![
                                        Box::new(lex_4_25::Token::Identifier("SafeMath".to_string()).to_leaf()),
                                        Box::new(lex_4_25::Token::Multiply.to_leaf())
                                    ]
                                })
                            ]
                        })
                    ]
                }
            ]
        };
        assert_eq!(expected_tree, actual_tree);
    }
}
