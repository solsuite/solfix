extern crate solfix;

#[cfg(test)]
mod parse_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{ parse, ParseNode, ParseTree };
    use solfix::utils::test_utils::parser::*;

    /*** Pragma ***/

    #[test]
    fn solidity_4_25_pragma_test1() {
        let actual = parse_str("pragma solidity 0.4.25;");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Pragma, vec![
                as_leaf(lex_4_25::to_identifier("solidity")),
                as_leaf(lex_4_25::to_version("0.4.25"))
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn solidity_4_25_pragma_test2() {
        let actual = parse_str("pragma solidity ^0.4.25;");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Pragma, vec![
                as_leaf(lex_4_25::to_identifier("solidity")),
                as_leaf(lex_4_25::Token::BitwiseXor),
                as_leaf(lex_4_25::to_version("0.4.25"))
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    /*** Contract ***/

    #[test]
    fn contract_test1() {
        let actual = parse_str("contract A {}");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("A")),
                as_leaf(lex_4_25::Token::OpenBrace)
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_test2() {
        let actual = parse_str("contract B is A {}");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("B")),
                as_node(lex_4_25::Token::Is, vec![
                    as_node(lex_4_25::Token::OpenParenthesis, vec![
                        as_node(lex_4_25::Token::UserDefinedTypeName, vec![
                            as_leaf(lex_4_25::to_identifier("A"))
                        ])
                    ])
                ]),
                as_leaf(lex_4_25::Token::OpenBrace)
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_enum_test1() { 
        let actual = parse_str("contract Enum { enum Foo { } }");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Enum")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::Enum, vec![
                        as_leaf(lex_4_25::to_identifier("Foo")),
                        as_leaf(lex_4_25::Token::OpenBrace)
                    ])
                ])
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_enum_test2() { 
        let actual = parse_str("contract Enum { enum Foo { Bar, Baz } }");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Enum")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::Enum, vec![
                        as_leaf(lex_4_25::to_identifier("Foo")),
                        as_node(lex_4_25::Token::OpenBrace, vec![
                            as_leaf(lex_4_25::to_identifier("Bar")),
                            as_leaf(lex_4_25::to_identifier("Baz"))
                        ])
                    ])
                ])
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_event_test1() { 
        let actual = parse_str("contract Event { event emptyEvent(); }");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Event")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::Event, vec![
                        as_leaf(lex_4_25::to_identifier("emptyEvent")),
                        as_leaf(lex_4_25::Token::OpenParenthesis)
                    ]) 
                ])
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_event_test2() { 
        let actual = parse_str("contract Event { event Transfer(address indexed from, address indexed to, uint256 indexed value); }");
        let expect = as_tree(vec![
           as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Event")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::Event, vec![
                        as_leaf(lex_4_25::to_identifier("Transfer")),
                        as_node(lex_4_25::Token::OpenParenthesis, vec![
                            as_node(lex_4_25::Token::EventParameter, vec![
                                as_leaf(lex_4_25::Token::Address),
                                as_leaf(lex_4_25::Token::Indexed),
                                as_leaf(lex_4_25::to_identifier("from"))
                            ]),
                            as_node(lex_4_25::Token::EventParameter, vec![
                                as_leaf(lex_4_25::Token::Address),
                                as_leaf(lex_4_25::Token::Indexed),
                                as_leaf(lex_4_25::to_identifier("to"))
                            ]),
                            as_node(lex_4_25::Token::EventParameter, vec![
                                as_leaf(lex_4_25::Token::Uint256),
                                as_leaf(lex_4_25::Token::Indexed),
                                as_leaf(lex_4_25::to_identifier("value"))
                            ])
                        ])
                    ])
                ])
            ]) 
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_function_test1() { 
        let actual = parse_str("contract Function { function doNothing() internal pure { } }");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Function")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::Function, vec![
                        as_leaf(lex_4_25::to_identifier("doNothing")),
                        as_leaf(lex_4_25::Token::OpenParenthesis),
                        as_leaf(lex_4_25::Token::Internal),
                        as_leaf(lex_4_25::Token::Pure),
                        as_leaf(lex_4_25::Token::OpenBrace)
                    ])
                ])
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_function_test2() { 
        let actual = parse_str("contract Function { function emitEvent() internal { emit someEvent(1 + 1); } }");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Function")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::Function, vec![
                        as_leaf(lex_4_25::to_identifier("emitEvent")),
                        as_leaf(lex_4_25::Token::OpenParenthesis),
                        as_leaf(lex_4_25::Token::Internal),
                        as_node(lex_4_25::Token::OpenBrace, vec![
                            as_node(lex_4_25::Token::Emit, vec![
                                as_node(lex_4_25::Token::OpenParenthesis, vec![
                                    as_leaf(lex_4_25::to_identifier("someEvent")),
                                    as_node(lex_4_25::Token::OpenParenthesis, vec![
                                        as_node(lex_4_25::Token::Plus, vec![
                                            as_leaf(lex_4_25::to_decimal_number("1")),
                                            as_leaf(lex_4_25::to_decimal_number("1"))
                                        ])
                                    ])
                                ])
                            ])
                        ])
                    ])
                ])
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_modifier_test1() { 
        let actual = parse_str("contract Modifier { modifier doNothing { _;} }");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Modifier")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::Modifier, vec![
                        as_leaf(lex_4_25::to_identifier("doNothing")),
                        as_node(lex_4_25::Token::OpenBrace, vec![ as_leaf(lex_4_25::Token::Placeholder) ])
                    ])
                ])
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_using_for_test1() { 
        let actual = parse_str("contract Using { using SafeMath for uint256; }");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Using")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::Using, vec![
                        as_leaf(lex_4_25::to_identifier("SafeMath")),
                        as_leaf(lex_4_25::Token::Uint256)
                    ])
                ])
            ])
        ]);
        expect_tree_eq(expect, actual);
    }

    #[test]
    fn contract_using_for_test2() {
        let actual = parse_str("contract Using { using SafeMath for *; }");
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Using")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::Using, vec![
                        as_leaf(lex_4_25::to_identifier("SafeMath")),
                        as_leaf(lex_4_25::Token::Multiply)
                    ])
                ])
            ])
        ]);
        expect_tree_eq(expect, actual);
    }
}
