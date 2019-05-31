extern crate solfix;

#[cfg(test)]
mod lexer_tests {
    use solfix::lex_4_25;

    #[test]
    fn recognition_test1() {
        let cur = &mut 0;
        let chars = String::from("contract A + B && { }{} () (A++)--;").chars().collect::<Vec<char>>();
        let a = String::from("A");
        let b = String::from("B");
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::Contract => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::Contract, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::Identifier(a) => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::Identifier(a), actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::Plus, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::Identifier(b) => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::Identifier(b), actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::LogicalAnd => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::LogicalAnd, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::CloseBrace => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::CloseBrace, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::CloseBrace => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::CloseBrace, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::CloseParenthesis => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::CloseParenthesis, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::Identifier(a) => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::Identifier(a), actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::Increment => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::Increment, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::CloseParenthesis => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::CloseParenthesis, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::Decrement => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::Decrement, actual)
        }
        match lex_4_25::next_token(&chars, cur) {
            lex_4_25::Token::Semicolon => (),
            actual => panic!("Expected {:?}, Actual {:?}", lex_4_25::Token::Semicolon, actual)
        }
    }
}
