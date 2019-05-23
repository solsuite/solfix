use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};

/* Lexer */

#[derive(Debug)]
enum Token {
    Address,
    Anonymous,
    As,
    Assembly,
    Bool,
    Break,
    Byte,
    Bytes,
    Bytes1,
    Bytes2,
    Bytes3,
    Bytes4,
    Bytes5,
    Bytes6,
    Bytes7,
    Bytes8,
    Bytes9,
    Bytes10,
    Bytes11,
    Bytes12,
    Bytes13,
    Bytes14,
    Bytes15,
    Bytes16,
    Bytes17,
    Bytes18,
    Bytes19,
    Bytes20,
    Bytes21,
    Bytes22,
    Bytes23,
    Bytes24,
    Bytes25,
    Bytes26,
    Bytes27,
    Bytes28,
    Bytes29,
    Bytes30,
    Bytes31,
    Bytes32,
    CloseBrace,
    CloseBracket,
    CloseParenthesis,
    Colon,
    Comma,
    Constant,
    Continue,
    Contract,
    Days,
    DecimalLiteral(String),
    Delete,
    Do,
    Else,
    Emit,
    Enum,
    Ether,
    Event,
    External,
    False,
    Finney,
    Fixed,
    For,
    From,
    Function,
    Hex,
    HexLiteral(String),
    Hours,
    Identifier(String),
    If,
    Import,
    Indexed,
    Int,
    Int8,
    Int16,
    Int24,
    Int32,
    Int40,
    Int48,
    Int56,
    Int64,
    Int72,
    Int80,
    Int88,
    Int96,
    Int104,
    Int112,
    Int120,
    Int128,
    Int136,
    Int144,
    Int152,
    Int160,
    Int168,
    Int176,
    Int184,
    Int192,
    Int200,
    Int208,
    Int216,
    Int224,
    Int232,
    Int240,
    Int248,
    Int256,
    Interface,
    Internal,
    Is,
    Let,
    Library, 
    Mapping,
    Memory,
    Minutes,
    Modifier,
    New,
    NoMatch,
    OpenBrace,
    OpenBracket,
    OpenParenthesis,
    Payable,
    Placeholder,
    Pragma,
    Private,
    Public,
    Pure,
    Return,
    Returns,
    Seconds,
    Semicolon,
    Storage,
    String,
    Struct,
    Szabo,
    Throw,
    True,
    Ufixed,
    Uint,
    Uint8,
    Uint16,
    Uint24,
    Uint32,
    Uint40,
    Uint48,
    Uint56,
    Uint64,
    Uint72,
    Uint80,
    Uint88,
    Uint96,
    Uint104,
    Uint112,
    Uint120,
    Uint128,
    Uint136,
    Uint144,
    Uint152,
    Uint160,
    Uint168,
    Uint176,
    Uint184,
    Uint192,
    Uint200,
    Uint208,
    Uint216,
    Uint224,
    Uint232,
    Uint240,
    Uint248,
    Uint256,
    Using,
    Var,
    Version(String),
    View,
    Weeks,
    Wei,
    While,
    Years,
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
            } else if line[*cur] == ':' {
                return Token::Colon;
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
               line[*cur] == ',' ||
               line[*cur] == ':'
            {
                *cur -= 1;
                return match collected.as_ref() {
                    "address" => Token::Address,
                    "anonymous" => Token::Anonymous, 
                    "as" => Token::As,
                    "assembly" => Token::Assembly,
                    "bool" => Token::Bool,
                    "break" => Token::Break,
                    "byte" => Token::Byte,
                    "bytes" => Token::Bytes,
                    "bytes1" => Token::Bytes1,
                    "bytes2" => Token::Bytes2,
                    "bytes3" => Token::Bytes3,
                    "bytes4" => Token::Bytes4,
                    "bytes5" => Token::Bytes5,
                    "bytes6" => Token::Bytes6,
                    "bytes7" => Token::Bytes7,
                    "bytes8" => Token::Bytes8,
                    "bytes9" => Token::Bytes9,
                    "bytes10" => Token::Bytes10,
                    "bytes11" => Token::Bytes11,
                    "bytes12" => Token::Bytes12,
                    "bytes13" => Token::Bytes13,
                    "bytes14" => Token::Bytes14,
                    "bytes15" => Token::Bytes15,
                    "bytes16" => Token::Bytes16,
                    "bytes17" => Token::Bytes17,
                    "bytes18" => Token::Bytes18,
                    "bytes19" => Token::Bytes19,
                    "bytes20" => Token::Bytes20,
                    "bytes21" => Token::Bytes21,
                    "bytes22" => Token::Bytes22,
                    "bytes23" => Token::Bytes23,
                    "bytes24" => Token::Bytes24,
                    "bytes25" => Token::Bytes25,
                    "bytes26" => Token::Bytes26,
                    "bytes27" => Token::Bytes27,
                    "bytes28" => Token::Bytes28,
                    "bytes29" => Token::Bytes29,
                    "bytes30" => Token::Bytes30,
                    "bytes31" => Token::Bytes31,
                    "bytes32" => Token::Bytes32,
                    "constant" => Token::Constant,
                    "continue" => Token::Continue,
                    "contract" => Token::Contract,
                    "days" => Token::Days,
                    "delete" => Token::Delete,
                    "do" => Token::Do,
                    "else" => Token::Else,
                    "emit" => Token::Emit,
                    "enum" => Token::Enum,
                    "ether" => Token::Ether,
                    "event" => Token::Event,
                    "external" => Token::External,
                    "false" => Token::False,
                    "finney" => Token::Finney,
                    "fixed" => Token::Fixed,
                    "for" => Token::For,
                    "from" => Token::From,
                    "function" => Token::Function,
                    "hex" => Token::Hex,
                    "hours" => Token::Hours,
                    "if" => Token::If,
                    "import" => Token::Import,
                    "indexed" => Token::Indexed,
                    "int" => Token::Int,
                    "int8" => Token::Int8,
                    "int16" => Token::Int16,
                    "int24" => Token::Int24,
                    "int32" => Token::Int32,
                    "int40" => Token::Int40,
                    "int48" => Token::Int48,
                    "int56" => Token::Int56,
                    "int64" => Token::Int64,
                    "int72" => Token::Int72,
                    "int80" => Token::Int80,
                    "int88" => Token::Int88,
                    "int96" => Token::Int96,
                    "int104" => Token::Int104,
                    "int112" => Token::Int112,
                    "int120" => Token::Int120,
                    "int128" => Token::Int128,
                    "int136" => Token::Int136,
                    "int144" => Token::Int144,
                    "int152" => Token::Int152,
                    "int160" => Token::Int160,
                    "int168" => Token::Int168,
                    "int176" => Token::Int176,
                    "int184" => Token::Int184,
                    "int192" => Token::Int192,
                    "int200" => Token::Int200,
                    "int208" => Token::Int208,
                    "int216" => Token::Int216,
                    "int224" => Token::Int224,
                    "int232" => Token::Int232,
                    "int240" => Token::Int240,
                    "int248" => Token::Int248,
                    "int256" => Token::Int256,
                    "interface" => Token::Interface,
                    "internal" => Token::Internal,
                    "is" => Token::Is,
                    "let" => Token::Let,
                    "library" => Token::Library,
                    "mapping" => Token::Mapping,
                    "memory" => Token::Memory,
                    "minutes" => Token::Minutes,
                    "modifier" => Token::Modifier,
                    "new" => Token::New,
                    "payable" => Token::Payable,
                    "pragma" => Token::Pragma,
                    "private" => Token::Private,
                    "public" => Token::Public,
                    "pure" => Token::Pure,
                    "return" => Token::Return,
                    "returns" => Token::Returns,
                    "seconds" => Token::Seconds,
                    "storage" => Token::Storage,
                    "string" => Token::String,
                    "struct" => Token::Struct,
                    "szabo" => Token::Szabo,
                    "throw" => Token::Throw,
                    "true" => Token::True,
                    "ufixed" => Token::Ufixed,
                    "uint" => Token::Uint,
                    "uint8" => Token::Uint8,
                    "uint16" => Token::Uint16,
                    "uint24" => Token::Uint24,
                    "uint32" => Token::Uint32,
                    "uint40" => Token::Uint40,
                    "uint48" => Token::Uint48,
                    "uint56" => Token::Uint56,
                    "uint64" => Token::Uint64,
                    "uint72" => Token::Uint72,
                    "uint80" => Token::Uint80,
                    "uint88" => Token::Uint88,
                    "uint96" => Token::Uint96,
                    "uint104" => Token::Uint104,
                    "uint112" => Token::Uint112,
                    "uint120" => Token::Uint120,
                    "uint128" => Token::Uint128,
                    "uint136" => Token::Uint136,
                    "uint144" => Token::Uint144,
                    "uint152" => Token::Uint152,
                    "uint160" => Token::Uint160,
                    "uint168" => Token::Uint168,
                    "uint176" => Token::Uint176,
                    "uint184" => Token::Uint184,
                    "uint192" => Token::Uint192,
                    "uint200" => Token::Uint200,
                    "uint208" => Token::Uint208,
                    "uint216" => Token::Uint216,
                    "uint224" => Token::Uint224,
                    "uint232" => Token::Uint232,
                    "uint240" => Token::Uint240,
                    "uint248" => Token::Uint248,
                    "uint256" => Token::Uint256,
                    "using" => Token::Using,
                    "var" => Token::Var,
                    "view" => Token::View,
                    "weeks" => Token::Weeks,
                    "wei" => Token::Wei,
                    "while" => Token::While,
                    "years" => Token::Years,
                    "_" => Token::Placeholder,
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
