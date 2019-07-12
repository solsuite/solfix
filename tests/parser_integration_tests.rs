extern crate solfix;

#[cfg(test)]
mod parser_integration_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{ parse, ParseNode, ParseTree };
    use solfix::utils::test_utils::parser::*;
    use std::fs;

    #[test]
    fn ownable_test() {
        let input = fs::read_to_string("./contracts/Ownable.sol")
            .expect("Test file not found: ./contracts/Ownable.sol");
        let actual = parse(input);
        let expect = as_tree(vec![
            as_node_raw(lex_4_25::Token::Pragma, vec![
                as_leaf(lex_4_25::to_identifier("solidity")),
                as_leaf(lex_4_25::Token::BitwiseXor),
                as_leaf(lex_4_25::to_version("0.4.25"))
            ]),
            as_node_raw(lex_4_25::Token::Contract, vec![
                as_leaf(lex_4_25::to_identifier("Ownable")),
                as_node(lex_4_25::Token::OpenBrace, vec![
                    as_node(lex_4_25::Token::StateVariable, vec![
                        as_leaf(lex_4_25::Token::Address),
                        as_leaf(lex_4_25::Token::Public),
                        as_leaf(lex_4_25::to_identifier("owner"))
                    ]),
                    as_node(lex_4_25::Token::Modifier, vec![
                        as_leaf(lex_4_25::to_identifier("onlyOwner")),
                        as_node(lex_4_25::Token::OpenBrace, vec![
                            as_node(lex_4_25::Token::OpenParenthesis, vec![
                                as_leaf(lex_4_25::to_identifier("require")),
                                as_node(lex_4_25::Token::OpenParenthesis, vec![
                                    as_node(lex_4_25::Token::Equals, vec![
                                        as_node(lex_4_25::Token::Dot, vec![
                                            as_leaf(lex_4_25::to_identifier("msg")),
                                            as_leaf(lex_4_25::to_identifier("sender"))
                                        ]),
                                        as_leaf(lex_4_25::to_identifier("owner"))
                                    ])
                                ])
                            ]),
                            as_leaf(lex_4_25::Token::Placeholder)
                        ])
                    ])
                ])
            ])
        ]);
        expect_tree_eq(expect, actual);
    }
} 
