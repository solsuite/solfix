use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

/* Lexer */

#[derive(Debug)]
enum Token {
    /* No Match */
    NoMatch,
    /* Misc */
    Semicolon,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    /* Keywords */
    Contract,
    Import,
    Interface,
    Library, 
    Pragma,
    Function,
    Event,
    Modifier,
    Identifier(String),
    HexLiteral(String),
    DecimalLiteral(String),
    Version(String)
}

fn next_token(line: &Vec<char>, cur: &mut usize) -> Token {
    let id_re = Regex::new(r"^[a-zA-Z\$_][a-zA-Z0-9\$_]*$").unwrap();
    let hex_re = Regex::new(r"^0x[0-9a-fA-F]*$").unwrap();
    let decimal_re = Regex::new(r"^[0-9]+(\.[0-9]*)?([eE][0-9]+)?$").unwrap();
    let version_re = Regex::new(r"^\^?[0-9]+\.[0-9]+\.[0-9]+").unwrap();
    let mut collected = String::new();
    while *cur < line.len() {
        if collected.len() == 0 {
            if line[*cur] == ';' {
                return Token::Semicolon;
            } else if line[*cur] == '{' {
                return Token::OpenBrace;
            } else if line[*cur] == '}' {
                return Token::CloseBrace;
            } else if line[*cur] == '(' {
                return Token::OpenParenthesis;
            } else if line[*cur] == ')' {
                return Token::CloseParenthesis;
            } else if line[*cur] == '[' {
                return Token::OpenBracket;
            } else if line[*cur] == ']' {
                return Token::CloseBracket;
            } else if line[*cur] == ',' {
                return Token::Comma;
            } else if !line[*cur].is_whitespace() {
                collected.push(line[*cur]);
            }
        } else {
            if *cur + 1 == line.len() || 
               line[*cur].is_whitespace() ||
               line[*cur] == ';' ||
               line[*cur] == '{' ||
               line[*cur] == '}' ||
               line[*cur] == '(' ||
               line[*cur] == ')' ||
               line[*cur] == '[' ||
               line[*cur] == ']' ||
               line[*cur] == ','
            {
                *cur -= 1;
                return match collected.as_ref() {
                    "contract" => Token::Contract,
                    "function" => Token::Function,
                    "library" => Token::Library,
                    "pragma" => Token::Pragma,
                    "import" => Token::Import,
                    "event" => Token::Event,
                    "modifier" => Token::Modifier,
                    id if id_re.is_match(id) => Token::Identifier(id.to_string()),
                    hex if hex_re.is_match(hex) => Token::HexLiteral(hex.to_string()),
                    num if decimal_re.is_match(num) => Token::DecimalLiteral(num.to_string()),
                    version if version_re.is_match(version) => Token::Version(version.to_string()),
                    none => panic!("No Token Matched {}", none)
                }
            } else {
                collected.push(line[*cur]);
            }
        }
        *cur += 1;
    }
    Token::NoMatch
}

/* Parser */
fn parse(input: BufReader<File>) {
    for line in input.lines() {
        let mut cur = 0;
        let chars = line.expect("Unable to read line").chars().collect::<Vec<char>>();
        while cur < chars.len() {
            println!("{:?}", next_token(&chars, &mut cur));
            cur += 1;
        }
    }
}

/* Main */

fn main() {
    let name = env::args().nth(1).expect("Usage: solidity-fix FILE_NAME");
    let input = File::open(name).expect("Unable to open input file");
    parse(BufReader::new(input));
}
