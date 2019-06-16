extern crate solfix;

#[cfg(test)]
mod parse_expression_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::parse_expression;
    use solfix::utils::test_utils::parser::*;

    /*** Arithmetic ***/

    #[test]
    fn addition_parsing_test() {
        let s = lex_4_25::to_chars("1500 + 0x000");
        let cur = &mut 0;
        let actual = parse_expression(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Plus, vec![
            as_leaf(lex_4_25::to_decimal_number("1500")),
            as_leaf(lex_4_25::to_hex_number("0x000"))
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn multiplication_parsing_test() {
        let s = lex_4_25::to_chars("1500 * 0x000");
        let cur = &mut 0;
        let actual = parse_expression(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Multiply, vec![
            as_leaf(lex_4_25::to_decimal_number("1500")),
            as_leaf(lex_4_25::to_hex_number("0x000"))
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn arithmetic_parsing_test1() {
        let s = lex_4_25::to_chars("(800 + 1500) * 0x000");
        let cur = &mut 0;
        let actual = parse_expression(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Multiply, vec![
            as_node(lex_4_25::Token::Plus, vec![
                as_leaf(lex_4_25::to_decimal_number("800")),
                as_leaf(lex_4_25::to_decimal_number("1500"))
            ]),
            as_leaf(lex_4_25::to_hex_number("0x000"))
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn arithmetic_parsing_test2() {
        let s = lex_4_25::to_chars("800 + 1500 * 0x000");
        let cur = &mut 0;
        let actual = parse_expression(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Plus, vec![
            as_leaf(lex_4_25::to_decimal_number("800")),
            as_node(lex_4_25::Token::Multiply, vec![
                as_leaf(lex_4_25::to_decimal_number("1500")),
                as_leaf(lex_4_25::to_hex_number("0x000"))
            ])
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn arithmetic_parsing_test3() {
        let s = lex_4_25::to_chars("800 * 1500 + 0x000");
        let cur = &mut 0;
        let actual = parse_expression(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Plus, vec![
            as_node(lex_4_25::Token::Multiply, vec![
                as_leaf(lex_4_25::to_decimal_number("800")),
                as_leaf(lex_4_25::to_decimal_number("1500"))
            ]),
            as_leaf(lex_4_25::to_hex_number("0x000"))
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn arithmetic_parsing_test4() {
        let s = lex_4_25::to_chars("800 * 1500 + 0x000 * 0x800");
        let cur = &mut 0;
        let actual = parse_expression(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Plus, vec![
            as_node(lex_4_25::Token::Multiply, vec![
                as_leaf(lex_4_25::to_decimal_number("800")),
                as_leaf(lex_4_25::to_decimal_number("1500"))
            ]),
            as_node(lex_4_25::Token::Multiply, vec![
                as_leaf(lex_4_25::to_hex_number("0x000")),
                as_leaf(lex_4_25::to_hex_number("0x800"))
            ])
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn arithmetic_parsing_test5() {
        let s = lex_4_25::to_chars("1 - 800 * 1500 + 0x000 * 0x800 / 5 ** 1800");
        let cur = &mut 0;
        let actual = parse_expression(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::Plus, vec![
            as_node(lex_4_25::Token::Minus, vec![
                as_leaf(lex_4_25::to_decimal_number("1")),
                as_node(lex_4_25::Token::Multiply, vec![
                    as_leaf(lex_4_25::to_decimal_number("800")),
                    as_leaf(lex_4_25::to_decimal_number("1500"))
                ])
            ]),
            as_node(lex_4_25::Token::Divide, vec![
                as_node(lex_4_25::Token::Multiply, vec![
                    as_leaf(lex_4_25::to_hex_number("0x000")),
                    as_leaf(lex_4_25::to_hex_number("0x800"))
                ]),
                as_node(lex_4_25::Token::Power, vec![
                    as_leaf(lex_4_25::to_decimal_number("5")),
                    as_leaf(lex_4_25::to_decimal_number("1800"))
                ])
            ])
        ]);
        expect_node_eq(expect, actual);
    }

    /*** Function call ***/
    
    #[test]
    fn function_call_test1() {
        let s = lex_4_25::to_chars("Identifier()");
        let cur = &mut 0;
        let actual = parse_expression(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::OpenParenthesis, vec![
            as_leaf(lex_4_25::to_identifier("Identifier")),
            as_leaf(lex_4_25::Token::OpenParenthesis)
        ]);
        expect_node_eq(expect, actual);
    }

    #[test]
    fn function_call_test2() {
        let s = lex_4_25::to_chars("add(1, 2)");
        let cur = &mut 0;
        let actual = parse_expression(&s, cur);
        let expect = as_node_raw(lex_4_25::Token::OpenParenthesis, vec![
            as_leaf(lex_4_25::to_identifier("add")),
            as_node(lex_4_25::Token::OpenParenthesis, vec![
                as_leaf(lex_4_25::to_decimal_number("1")),
                as_leaf(lex_4_25::to_decimal_number("2"))
            ])
        ]);
        expect_node_eq(expect, actual);
    }
}
