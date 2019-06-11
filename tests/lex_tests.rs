extern crate solfix;

#[cfg(test)]
mod lexer_tests {
    use solfix::lex_4_25;

    fn fail_test(expect: lex_4_25::Token, actual: lex_4_25::Token) {
        panic!("Expected: {:?} | Actual: {:?}", expect, actual);
    }

    fn expect_next_token(s: &Vec<char>, cur: &mut usize, t: lex_4_25::Token) {
        match lex_4_25::next_token(&s, cur) {
            ref next if *next == t => (),
            actual => fail_test(t, actual)
        };
    }

    #[test]
    fn recognition_test1() {
        let s = lex_4_25::to_chars("contract A + B && { }{} () (A++)--;");
        let cur = &mut 0;
        expect_next_token(&s, cur, lex_4_25::Token::Contract);
        expect_next_token(&s, cur, lex_4_25::to_identifier("A"));
        expect_next_token(&s, cur, lex_4_25::Token::Plus);
        expect_next_token(&s, cur, lex_4_25::to_identifier("B"));
        expect_next_token(&s, cur, lex_4_25::Token::LogicalAnd);
        expect_next_token(&s, cur, lex_4_25::Token::OpenBrace);
        expect_next_token(&s, cur, lex_4_25::Token::CloseBrace);
        expect_next_token(&s, cur, lex_4_25::Token::OpenBrace);
        expect_next_token(&s, cur, lex_4_25::Token::CloseBrace);
        expect_next_token(&s, cur, lex_4_25::Token::OpenParenthesis);
        expect_next_token(&s, cur, lex_4_25::Token::CloseParenthesis);
        expect_next_token(&s, cur, lex_4_25::Token::OpenParenthesis);
        expect_next_token(&s, cur, lex_4_25::to_identifier("A"));
        expect_next_token(&s, cur, lex_4_25::Token::Increment);
        expect_next_token(&s, cur, lex_4_25::Token::CloseParenthesis);
        expect_next_token(&s, cur, lex_4_25::Token::Decrement);
        expect_next_token(&s, cur, lex_4_25::Token::Semicolon);
    }

    #[test]
    fn recognition_test2() {
        let s = lex_4_25::to_chars("library A is B, C { }");
        let cur = &mut 0;
        expect_next_token(&s, cur, lex_4_25::Token::Library);
        expect_next_token(&s, cur, lex_4_25::to_identifier("A"));
        expect_next_token(&s, cur, lex_4_25::Token::Is);
        expect_next_token(&s, cur, lex_4_25::to_identifier("B"));
        expect_next_token(&s, cur, lex_4_25::Token::Comma);
        expect_next_token(&s, cur, lex_4_25::to_identifier("C"));
        expect_next_token(&s, cur, lex_4_25::Token::OpenBrace);
        expect_next_token(&s, cur, lex_4_25::Token::CloseBrace);
    }

    #[test]
    fn recognition_test3() {
        let s = lex_4_25::to_chars("event A(); var x = hex\"DEADBEEF\";");
        let cur = &mut 0;
        expect_next_token(&s, cur, lex_4_25::Token::Event);
        expect_next_token(&s, cur, lex_4_25::to_identifier("A"));
        expect_next_token(&s, cur, lex_4_25::Token::OpenParenthesis);
        expect_next_token(&s, cur, lex_4_25::Token::CloseParenthesis);
        expect_next_token(&s, cur, lex_4_25::Token::Semicolon);
        expect_next_token(&s, cur, lex_4_25::Token::Var);
        expect_next_token(&s, cur, lex_4_25::to_identifier("x"));
        expect_next_token(&s, cur, lex_4_25::Token::Assignment);
        expect_next_token(&s, cur, lex_4_25::to_hex_literal("hex\"DEADBEEF\""));
    }
}
