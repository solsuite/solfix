extern crate solfix;

#[cfg(test)]
mod lexer_tests {
    use solfix::lex_4_25;
    use solfix::utils::test_utils::lexer;

    #[test]
    fn recognition_test1() {
        let s = lex_4_25::to_chars("contract A + B && { }{} () (A++)--;");
        let cur = &mut 0;
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Contract);
        lexer::expect_next_token(&s, cur, lex_4_25::to_identifier("A"));
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Plus);
        lexer::expect_next_token(&s, cur, lex_4_25::to_identifier("B"));
        lexer::expect_next_token(&s, cur, lex_4_25::Token::LogicalAnd);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::OpenBrace);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::CloseBrace);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::OpenBrace);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::CloseBrace);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::OpenParenthesis);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::CloseParenthesis);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::OpenParenthesis);
        lexer::expect_next_token(&s, cur, lex_4_25::to_identifier("A"));
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Increment);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::CloseParenthesis);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Decrement);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Semicolon);
    }

    #[test]
    fn recognition_test2() {
        let s = lex_4_25::to_chars("library A is B, C { }");
        let cur = &mut 0;
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Library);
        lexer::expect_next_token(&s, cur, lex_4_25::to_identifier("A"));
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Is);
        lexer::expect_next_token(&s, cur, lex_4_25::to_identifier("B"));
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Comma);
        lexer::expect_next_token(&s, cur, lex_4_25::to_identifier("C"));
        lexer::expect_next_token(&s, cur, lex_4_25::Token::OpenBrace);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::CloseBrace);
    }

    #[test]
    fn recognition_test3() {
        let s = lex_4_25::to_chars("event A(); var x = hex\"DEADBEEF\";");
        let cur = &mut 0;
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Event);
        lexer::expect_next_token(&s, cur, lex_4_25::to_identifier("A"));
        lexer::expect_next_token(&s, cur, lex_4_25::Token::OpenParenthesis);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::CloseParenthesis);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Semicolon);
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Var);
        lexer::expect_next_token(&s, cur, lex_4_25::to_identifier("x"));
        lexer::expect_next_token(&s, cur, lex_4_25::Token::Assignment);
        lexer::expect_next_token(&s, cur, lex_4_25::to_hex_literal("hex\"DEADBEEF\""));
    }
}
