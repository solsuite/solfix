use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Address,
    AndEquals,
    Anonymous,
    Arrow,
    As,
    Assembly,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
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
    DecimalNumber(String),
    Decrement,
    Delete,
    Divide,
    DivideEquals,
    Do,
    Dot,
    Else,
    Emit,
    Enum,
    Equals,
    Ether,
    Event,
    EventParameter,
    Exclamation,
    External,
    False,
    Finney,
    Fixed,
    For,
    From,
    Function,
    GreaterThan,
    GreaterThanOrEquals,
    Hex,
    HexLiteral(String),
    HexNumber(String),
    Hours,
    Identifier(String),
    If,
    Import,
    Increment,
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
    LessThan,
    LessThanOrEquals,
    Let,
    Library, 
    LogicalAnd,
    LogicalOr,
    Mapping,
    Memory,
    Minus,
    MinusEquals,
    Minutes,
    ModEquals,
    Modifier,
    Modulus,
    Multiply,
    MultiplyEquals,
    New,
    NoMatch,
    NotEquals,
    OpenBrace,
    OpenBracket,
    OpenParenthesis,
    OrEquals,
    Payable,
    Placeholder,
    Plus,
    PlusEquals,
    Power,
    Pragma,
    Private,
    Public,
    Pure,
    Question,
    Return,
    Returns,
    Seconds,
    Semicolon,
    Set,
    ShiftLeft,
    ShiftLeftEquals,
    ShiftRight,
    ShiftRightEquals,
    Storage,
    String,
    StringLiteral(String),
    Struct,
    Szabo,
    Throw,
    Tilda,
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
    XorEquals,
    Years,
}

impl Token {
    // Returns whether the Token is a unit
    pub fn is_number_unit(&self) -> bool {
        return match self {
            Token::Days => true,
            Token::Ether => true,
            Token::Finney => true,
            Token::Hours => true,
            Token::Minutes => true,
            Token::Seconds => true,
            Token::Szabo => true,
            Token::Weeks => true,
            Token::Wei => true,
            Token::Years => true,
            _ => false
        }
    }

    // Returns whether the Token is an int
    pub fn is_int(&self) -> bool {
        return match self {
            Token::Int => true,
            Token::Int8 => true,
            Token::Int16 => true,
            Token::Int24 => true,
            Token::Int32 => true,
            Token::Int40 => true,
            Token::Int48 => true,
            Token::Int56 => true,
            Token::Int64 => true,
            Token::Int72 => true,
            Token::Int80 => true,
            Token::Int88 => true,
            Token::Int96 => true,
            Token::Int104 => true,
            Token::Int112 => true,
            Token::Int120 => true,
            Token::Int128 => true,
            Token::Int136 => true,
            Token::Int144 => true,
            Token::Int152 => true,
            Token::Int160 => true,
            Token::Int168 => true,
            Token::Int176 => true,
            Token::Int184 => true,
            Token::Int192 => true,
            Token::Int200 => true,
            Token::Int208 => true,
            Token::Int216 => true,
            Token::Int224 => true,
            Token::Int232 => true,
            Token::Int240 => true,
            Token::Int248 => true,
            Token::Int256 => true,
            _ => false
        }
    }

    // Returns whether the token is an unsigned integer
    pub fn is_uint(&self) -> bool {
        return match self {
            Token::Uint => true,
            Token::Uint8 => true,
            Token::Uint16 => true,
            Token::Uint24 => true,
            Token::Uint32 => true,
            Token::Uint40 => true,
            Token::Uint48 => true,
            Token::Uint56 => true,
            Token::Uint64 => true,
            Token::Uint72 => true,
            Token::Uint80 => true,
            Token::Uint88 => true,
            Token::Uint96 => true,
            Token::Uint104 => true,
            Token::Uint112 => true,
            Token::Uint120 => true,
            Token::Uint128 => true,
            Token::Uint136 => true,
            Token::Uint144 => true,
            Token::Uint152 => true,
            Token::Uint160 => true,
            Token::Uint168 => true,
            Token::Uint176 => true,
            Token::Uint184 => true,
            Token::Uint192 => true,
            Token::Uint200 => true,
            Token::Uint208 => true,
            Token::Uint216 => true,
            Token::Uint224 => true,
            Token::Uint232 => true,
            Token::Uint240 => true,
            Token::Uint248 => true,
            Token::Uint256 => true,
            _ => false
        }
    }

    // Returns whether the Token is a byte, bytes, or bytesXX
    pub fn is_byte(&self) -> bool {
        return match self {
            Token::Byte => true,
            Token::Bytes => true,
            Token::Bytes1 => true,
            Token::Bytes2 => true,
            Token::Bytes3 => true,
            Token::Bytes4 => true,
            Token::Bytes5 => true,
            Token::Bytes6 => true,
            Token::Bytes7 => true,
            Token::Bytes8 => true,
            Token::Bytes9 => true,
            Token::Bytes10 => true,
            Token::Bytes11 => true,
            Token::Bytes12 => true,
            Token::Bytes13 => true,
            Token::Bytes14 => true,
            Token::Bytes15 => true,
            Token::Bytes16 => true,
            Token::Bytes17 => true,
            Token::Bytes18 => true,
            Token::Bytes19 => true,
            Token::Bytes20 => true,
            Token::Bytes21 => true,
            Token::Bytes22 => true,
            Token::Bytes23 => true,
            Token::Bytes24 => true,
            Token::Bytes25 => true,
            Token::Bytes26 => true,
            Token::Bytes27 => true,
            Token::Bytes28 => true,
            Token::Bytes29 => true,
            Token::Bytes30 => true,
            Token::Bytes31 => true,
            Token::Bytes32 => true,
            _ => false
        }
    }

    // Returns whether the token represents an elementary type
    // (address, bool, string, var, int, uint, byte)
    pub fn is_elementary_type(&self) -> bool {
        return match self {
            Token::Address => true,
            Token::Bool => true,
            Token::String => true,
            Token::Var => true,
            int if int.is_int() => true,
            uint if uint.is_uint() => true,
            byte if byte.is_byte() => true,
            _ => false
        }
    }
}

trait StopToken {
    fn is_stop_token(self) -> bool;
}

impl StopToken for char {
    // If true, the lexer will stop reading at this Token
    fn is_stop_token(self) -> bool {
        if self.is_whitespace() ||
           self == ';' ||
           self == '{' ||
           self == '}' ||
           self == '(' ||
           self == ')' ||
           self == '[' ||
           self == ']' ||
           self == ',' ||
           self == ':' ||
           self == '!' ||
           self == '~' ||
           self == '*' ||
           self == '/' ||
           self == '+' ||
           self == '-' ||
           self == '=' ||
           self == '>' ||
           self == '<' ||
           self == '!' ||
           self == '.' {
            return true;
       }
       return false; 
    }
}

/**
 * Given a collected string, returns the matching Token
 * Returns Token::NoMatch if no match is found
 */
fn match_collected(collected: String) -> Token {
    let decimal_re = Regex::new(r"^[0-9]+(\.[0-9]*)?([eE][0-9]+)?$").unwrap();
    let id_re = Regex::new(r"^[a-zA-Z\$_][a-zA-Z0-9\$_]*$").unwrap();
    let hex_re = Regex::new(r"^0x[0-9a-fA-F]*$").unwrap();
    let hex_literal_re = Regex::new(r#"^hex(\\"([0-9a-fA-F]{2})*\\"|'([0-9a-fA-F]{2})*')$"#).unwrap();
    let version_re = Regex::new(r"^\^?[0-9]+\.[0-9]+\.[0-9]+").unwrap();
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
        hex if hex_re.is_match(hex) => Token::HexNumber(hex.to_string()),
        num if decimal_re.is_match(num) => Token::DecimalNumber(num.to_string()),
        hex if hex_literal_re.is_match(hex) => Token::HexLiteral(hex.to_string()),
        version if version_re.is_match(version) => Token::Version(version.to_string()),
        _ => Token::NoMatch
    }
}

/**
 * Returns the next Token found in the line and increments cur
 * to the end of the Token in the parsed line
 */
pub fn next_token(line: &Vec<char>, cur: &mut usize) -> Token {
    let mut string = false;
    let mut collected = String::new();
    while *cur < line.len() {
        // If no characters have been collected, we are reading a new Token
        if collected.len() == 0 {
            if line[*cur] == ';' {
                *cur += 1;
                return Token::Semicolon;
            } else if line[*cur] == '{' {
                *cur += 1;
                return Token::OpenBrace;
            } else if line[*cur] == '}' {
                *cur += 1;
                return Token::CloseBrace;
            } else if line[*cur] == '(' {
                *cur += 1;
                return Token::OpenParenthesis;
            } else if line[*cur] == ')' {
                *cur += 1;
                return Token::CloseParenthesis;
            } else if line[*cur] == '[' {
                *cur += 1;
                return Token::OpenBracket;
            } else if line[*cur] == ']' {
                *cur += 1;
                return Token::CloseBracket;
            } else if line[*cur] == '?' {
                *cur += 1;
                return Token::Question;
            } else if line[*cur] == ',' {
                *cur += 1;
                return Token::Comma;
            } else if line[*cur] == ':' {
                *cur += 1;
                return Token::Colon;
            }  else if line[*cur] == '~' {
                *cur += 1;
                return Token::Tilda;
            } else if line[*cur] == '/' {
                *cur += 1;
                return Token::Divide;
            } else if line[*cur] == '.' {
                *cur += 1;
                return Token::Dot;
            } else if line[*cur] == '\"' {
                collected.push(line[*cur]);
                string = true;
            } else if !line[*cur].is_whitespace() {
                collected.push(line[*cur]);
            } // else: do nothing, increment cur by 1
        } else {
            if string {
                collected.push(line[*cur]);
                if line[*cur] == '\"' {
                    return Token::StringLiteral(collected);
                }
            } else if line[*cur] == '.' && (collected == "^0" || collected == "0" || collected == "^0.4" || collected == "0.4") {
               collected.push('.');
            } else if line[*cur] == '0' && collected == "^" {
               collected.push('0');
            } else if line[*cur] == '>' && collected == "=" {
                *cur += 1;
                return Token::Arrow;
            } else if line[*cur] == '*' && collected == "*" {
                *cur += 1;
                return Token::Power;
            } else if line[*cur] == '=' && collected == "=" {
                *cur += 1;
                return Token::Equals;
            } else if line[*cur] == '=' && collected == "<" {
                *cur += 1;
                return Token::LessThanOrEquals;
            } else if line[*cur] == '=' && collected == ">" {
                *cur += 1;
                return Token::GreaterThanOrEquals;
            } else if line[*cur] == '=' && collected == "!" {
                *cur += 1;
                return Token::NotEquals;
            } else if line[*cur] == '=' && collected == "|" {
                *cur += 1;
                return Token::OrEquals;
            } else if line[*cur] == '=' && collected == "+" {
                *cur += 1;
                return Token::PlusEquals;
            } else if line[*cur] == '=' && collected == "-" {
                *cur += 1;
                return Token::MinusEquals;
            } else if line[*cur] == '=' && collected == "*" {
                *cur += 1;
                return Token::MultiplyEquals;
            } else if line[*cur] == '=' && collected == "/" {
                *cur += 1;
                return Token::DivideEquals;
            } else if line[*cur] == '=' && collected == "%" {
                *cur += 1;
                return Token::ModEquals;
            } else if line[*cur] == '=' && collected == "<<" {
                *cur += 1;
                return Token::ShiftLeftEquals;
            } else if line[*cur] == '=' && collected == ">>" {
                *cur += 1;
                return Token::ShiftRightEquals;
            } else if line[*cur] == '+' && collected == "+" {
                *cur += 1;
                return Token::Increment;
            } else if line[*cur] == '-' && collected == "-" {
                *cur += 1;
                return Token::Decrement;
            } else if line[*cur] == '>' && collected == ">" {
                *cur += 1;
                return Token::ShiftRight;
            } else if line[*cur] == '|' && collected == "|" {
                *cur += 1;
                return Token::LogicalOr; 
            } else if line[*cur] == '&' && collected == "&" {
                *cur += 1;
                return Token::LogicalAnd;
            } else if collected == "^" {
                return Token::BitwiseXor;
            } else if collected == "<<" {
                return Token::ShiftLeft;
            } else if collected == "-" {
                return Token::Minus;
            } else if collected == "+" {
                return Token::Plus;
            } else if collected == "|" {
                return Token::BitwiseOr;
            } else if collected == "&" {
                return Token::BitwiseAnd;
            } else if collected == "<" {
                return Token::LessThan;
            } else if collected == "=" {
                return Token::Set;
            } else if collected == "!" {
                return Token::Exclamation;
            } else if collected == ">" {
                return Token::GreaterThan;
            } else if collected == "*" {
                return Token::Multiply;
            } else if line[*cur].is_stop_token() {
                return match_collected(collected);
            } else {
                collected.push(line[*cur]);
            }
        }
        *cur += 1;
    }
    // If the end of the line is reached, match the collected characters and return the result
    match_collected(collected)
}

// Return the next token in the line, without incrementing cur
pub fn peek_token(line: &Vec<char>, cur: &mut usize) -> Token {
    let old = *cur;
    let next = next_token(line, cur);
    *cur = old;
    next
}

#[cfg(test)] 
mod tests {
    use super::*;

    /* String Literal */

    #[test]
    fn string_literal_test1() {
        let path = String::from("\"\"");
        let chars = path.chars().collect::<Vec<char>>();
        let cur = &mut 0; 
        let actual = next_token(&chars, cur);
        match actual {
            Token::StringLiteral(path) => assert_eq!(&path, "\"\""), 
            actual => panic!("Expected: {:?} | Actual: {:?}", Token::StringLiteral("\"\"".to_string()), actual)
        }
    }

    #[test]
    fn string_literal_test2() {
        let chars = String::from("\"test_file\"").chars().collect::<Vec<char>>();
        let cur = &mut 0; 
        let actual = next_token(&chars, cur);
        match actual {
            Token::StringLiteral(path) => assert_eq!(&path, "\"test_file\""), 
            actual => panic!("Expected: {:?} | Actual: {:?}", Token::StringLiteral("\"test_file\"".to_string()), actual)
        }
    }

    #[test]
    fn string_literal_test3() {
        let path = String::from("\"test_file.sol\"");
        let chars = path.chars().collect::<Vec<char>>();
        let cur = &mut 0; 
        let actual = next_token(&chars, cur);
        match actual {
            Token::StringLiteral(path) => assert_eq!(&path, "\"test_file.sol\""), 
            actual => panic!("Expected: {:?} | Actual: {:?}", Token::StringLiteral("\"test_file.sol\"".to_string()), actual)
        }
    }
}
