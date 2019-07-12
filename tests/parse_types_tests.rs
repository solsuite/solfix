extern crate solfix;

#[cfg(test)]
mod types_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::parse_type_name;
    use solfix::utils::test_utils::parser::*;

    #[test]
    fn elementary_type_test1() { 
        let s = lex_4_25::to_chars("address");
        let cur = &mut 0;
        let actual = parse_type_name(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Address, vec![]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn user_defined_type_test1() { 
        let s = lex_4_25::to_chars("Address.Enum");
        let cur = &mut 0;
        let actual = parse_type_name(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::UserDefinedTypeName, vec![
            as_leaf(lex_4_25::to_identifier("Address")),
            as_leaf(lex_4_25::to_identifier("Enum"))
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn mapping_test1() {
        let s = lex_4_25::to_chars("mapping (uint256 => uint256)");
        let cur = &mut 0;
        let actual = parse_type_name(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Mapping, vec![
            as_leaf(lex_4_25::Token::Uint256),
            as_leaf(lex_4_25::Token::Uint256)
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn array_type_name_test1() {
        let s = lex_4_25::to_chars("uint256[1 + 1]");
        let cur = &mut 0;
        let actual = parse_type_name(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::OpenBracket, vec![
            as_leaf(lex_4_25::Token::Uint256),
            as_node(lex_4_25::Token::Plus, vec![
                as_leaf(lex_4_25::to_decimal_number("1")),
                as_leaf(lex_4_25::to_decimal_number("1"))
            ])
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn function_type_name_test1() {
        let s = lex_4_25::to_chars("function () internal");
        let cur = &mut 0;
        let actual = parse_type_name(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Function, vec![
            as_leaf(lex_4_25::Token::OpenParenthesis),
            as_leaf(lex_4_25::Token::Internal)
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn function_type_name_test2() {
        let s = lex_4_25::to_chars("function () internal pure");
        let cur = &mut 0;
        let actual = parse_type_name(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Function, vec![
            as_leaf(lex_4_25::Token::OpenParenthesis),
            as_leaf(lex_4_25::Token::Internal),
            as_leaf(lex_4_25::Token::Pure)
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn function_type_name_test3() {
        let s = lex_4_25::to_chars("function (uint256, bytes32) internal pure returns (bool) ");
        let cur = &mut 0;
        let actual = parse_type_name(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Function, vec![
            as_node(lex_4_25::Token::OpenParenthesis, vec![
                as_leaf(lex_4_25::Token::Uint256),
                as_leaf(lex_4_25::Token::Bytes32)
            ]),
            as_leaf(lex_4_25::Token::Internal),
            as_leaf(lex_4_25::Token::Pure),
            as_node(lex_4_25::Token::Returns, vec![
                as_node(lex_4_25::Token::OpenParenthesis, vec![
                    as_leaf(lex_4_25::Token::Bool)
                ])
            ])
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn function_type_name_test4() {
        let s = lex_4_25::to_chars("function (function (uint256) internal) external payable returns (function (bool) external returns (uint256)) ");
        let cur = &mut 0;
        let actual = parse_type_name(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Function, vec![
            as_node(lex_4_25::Token::OpenParenthesis, vec![
                as_node(lex_4_25::Token::Function, vec![
                    as_node(lex_4_25::Token::OpenParenthesis, vec![
                        as_leaf(lex_4_25::Token::Uint256)
                    ]),
                    as_leaf(lex_4_25::Token::Internal)
                ])
            ]),
            as_leaf(lex_4_25::Token::External),
            as_leaf(lex_4_25::Token::Payable),
            as_node(lex_4_25::Token::Returns, vec![
                as_node(lex_4_25::Token::OpenParenthesis, vec![
                    as_node(lex_4_25::Token::Function, vec![
                        as_node(lex_4_25::Token::OpenParenthesis, vec![
                            as_leaf(lex_4_25::Token::Bool)
                        ]),
                        as_leaf(lex_4_25::Token::External),
                        as_node(lex_4_25::Token::Returns, vec![
                            as_node(lex_4_25::Token::OpenParenthesis, vec![
                                as_leaf(lex_4_25::Token::Uint256)
                            ])
                        ])
                    ])
                ])
            ])
        ]);
        expect_node_eq(expect, actual);
    }
}
