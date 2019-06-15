use regex::Regex;

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Address,
    AndEquals,
    Anonymous,
    Arrow,
    As,
    Assembly,
    Assignment,
    ASMAssign,
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
    CommentMulti,
    CommentSingle,
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
    EOF,
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
    Illegal,
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
    Parameter,
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
    ShiftLeft,
    ShiftLeftEquals,
    ShiftRight,
    ShiftRightEquals,
    StateVariable,
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
    UserDefinedTypeName,
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

trait LineMatch {
    fn match_idx(&self, idx: usize, val: char) -> bool;
    fn is_digit_at(&self, idx: usize) -> bool;
    fn is_rational_at(&self, idx: usize) -> bool;
    fn is_hex_digit_at(&self, idx: usize) -> bool;
    fn is_hex_delim_at(&self, idx: usize) -> bool;
    fn is_whitespace_at(&self, idx: usize) -> bool;
}

impl LineMatch for Vec<char> {
    fn match_idx(&self, idx: usize, val: char) -> bool {
        match self.get(idx) {
            Some(v) => v == &val,
            None => false
        }
    }

    fn is_digit_at(&self, idx: usize) -> bool {
        match self.get(idx) {
            Some(v) => v.is_digit(10),
            None => false
        }
    }

    fn is_hex_digit_at(&self, idx: usize) -> bool {
        match self.get(idx) {
            Some(v) => v.is_digit(16),
            None => false
        }
    }

    fn is_rational_at(&self, idx: usize) -> bool {
        match self.get(idx) {
            Some(v) => v.is_rational(),
            None => false
        }
    }

    fn is_hex_delim_at(&self, idx: usize) -> bool {
        match self.get(idx) {
            Some(v) => *v == 'x' || *v == 'X',
            None => false
        }
    }

    fn is_whitespace_at(&self, idx: usize) -> bool {
        match self.get(idx) {
            Some(v) => v.is_whitespace(),
            None => false
        }
    }
}

trait CharExt {
    fn starts_rational(&self) -> bool;
    fn starts_iden_or_keyword(&self) -> bool;
    fn is_iden_or_keyword_part(&self) -> bool;
    fn is_whitespace(&self) -> bool;
    fn is_rational(&self) -> bool;
}

impl CharExt for char {
    // Not allowed: Leading 0, leading 'e'
    fn starts_rational(&self) -> bool {
        return (self.is_digit(10) || *self == '.') && *self != '0';
    }

    // If self could be the first character of an identifier, returns true
    fn starts_iden_or_keyword(&self) -> bool {
        return *self == '_' || *self == '$' || self.is_ascii_alphabetic();
    }

    // If self could be a component of an identifier or keyword, returns true
    fn is_iden_or_keyword_part(&self) -> bool {
        return self.starts_iden_or_keyword() || self.is_digit(10);
    }

    // Returns true if the char could be a part of a rational literal
    fn is_rational(&self) -> bool {
        return 
            *self == 'e' || 
            *self == 'E' || 
            *self == '.' || 
            self.is_digit(10);
    }

    // If self is whitespace, returns true
    fn is_whitespace(&self) -> bool {
        return *self == ' ' || *self == '\n' || *self == '\t' || *self == '\r';
    }
}

trait AsString {
    fn as_string(&self) -> String;
}

impl AsString for Vec<char> {
    fn as_string(&self) -> String {
        return self.into_iter().collect();
    }
}

pub fn to_chars(string: &str) -> Vec<char> {
    return string.chars().collect::<Vec<char>>();
}

pub fn to_identifier(string: &str) -> Token {
    return Token::Identifier(string.to_string());
}

pub fn to_version(string: &str) -> Token {
    return Token::Version(string.to_string());
}

pub fn to_string_literal(string: &str) -> Token {
    return Token::StringLiteral(string.to_string());
}

pub fn to_decimal_number(string: &str) -> Token {
    return Token::DecimalNumber(string.to_string());
}

pub fn to_hex_number(string: &str) -> Token {
    return Token::HexNumber(string.to_string());
}

pub fn to_hex_literal(string: &str) -> Token {
    return Token::HexLiteral(string.to_string());
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
 * Matches . at line[*cur] with Token::Dot
 */
fn match_period(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.is_digit_at(*cur + 1) {
        return match_rational(line, cur);
    } else {
        return Token::Dot;
    }
}

/**
 * Matches : at line[*cur] with its corresponding Token
 * :  | Colon
 * := | ASMAssign
 */
fn match_colon(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::ASMAssign;
    } else {
        return Token::Colon;
    }
}

/**
 * Matches = at line[*cur] with its corresponding Token
 * =  | Assignment
 * == | Equals
 * => | Arrow
 */
fn match_equals(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::Equals;
    } else if line.match_idx(*cur + 1, '>') {
        *cur += 1;
        return Token::Arrow;
    } else {
        return Token::Assignment;
    }
}

/**
 * Matches + at line[*cur] with its corresponding Token
 * +  | Plus
 * ++ | Increment
 * += | PlusEquals
 */
fn match_plus(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '+') {
        *cur += 1;
        return Token::Increment;
    } else if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::PlusEquals;
    } else {
        return Token::Plus;
    }
}

/**
 * Matches - at line[*cur] with its corresponding Token
 * -  | Minus
 * -- | Decrement
 * -= | MinusEquals
 */
fn match_minus(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '-') {
        *cur += 1;
        return Token::Decrement;
    } else if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::MinusEquals;
    } else {
        return Token::Minus;
    }
}

/**
 * Matches * at line[*cur] with its corresponding Token
 * *  | Multiply
 * ** | Power
 * *= | MultiplyEquals
 */
fn match_star(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '*') {
        *cur += 1;
        return Token::Power;
    } else if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::MultiplyEquals;
    } else {
        return Token::Multiply;
    }
}

/**
 * Matches / at line[*cur] with its corresponding Token
 * /  | Divide
 * // | CommentSingle
 * /* | CommentMulti */
 * /= | DivideEquals
 */
fn match_slash(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::DivideEquals;
    } else if line.match_idx(*cur + 1, '/') {
        *cur += 1;
        return Token::CommentSingle;
    } else if line.match_idx(*cur + 1, '*') {
        *cur += 1;
        return Token::CommentMulti;
    } else {
        return Token::Divide;
    }
}

/**
 * Matches > at line[*cur] with its corresponding Token
 * >    | GreaterThen
 * >=   | GreaterThanOrEquals
 * >>   | ShiftRight
 * >>=  | ShiftRightEquals
 * >>>  | TODO
 * >>>= | TODO
 */
fn match_rarrow(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::GreaterThanOrEquals;
    } else if line.match_idx(*cur + 1, '>') {
        if line.match_idx(*cur + 2, '=') {
            *cur += 2;
            return Token::ShiftRightEquals;
        } else if line.match_idx(*cur + 2, '>') {
            if line.match_idx(*cur + 3, '=') {
                *cur += 3;
                return Token::Illegal; // TODO
            } else {
                *cur += 2;
                return Token::Illegal; // TODO
            }
        } else {
            *cur += 1;
            return Token::ShiftRight;
        }
    } else {
        return Token::GreaterThan;
    }
}

/**
 * Matches < at line[*cur] with its corresponding Token
 * <    | LessThen
 * <=   | LessThanOrEquals
 * <<   | ShiftLeft
 * <<=  | ShiftLeftEquals
 */
fn match_larrow(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::LessThanOrEquals;
    } else if line.match_idx(*cur + 1, '<') {
        if line.match_idx(*cur + 2, '=') {
            *cur += 2;
            return Token::ShiftLeftEquals;
        } else {
            *cur += 1;
            return Token::ShiftLeft;
        }
    } else {
        return Token::LessThan;
    }
}

/**
 * Matches ! at line[*cur] with its corresponding Token
 * !  | Exclamation
 * != | NotEquals
 */
fn match_exclamation(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::NotEquals;
    } else {
        return Token::Exclamation;
    }
}

/**
 * Matches % at line[*cur] with its corresponding Token
 * %  | Modulus
 * %= | ModEquals
 */
fn match_percent(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::ModEquals;
    } else {
        return Token::Modulus;
    }
}

/**
 * Matches & at line[*cur] with its corresponding Token
 * &  | BitwiseAnd
 * && | LogicalAnd
 * &= | AndEquals
 */
fn match_and(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '&') {
        *cur += 1;
        return Token::LogicalAnd;
    } else if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::AndEquals;
    } else {
        return Token::BitwiseAnd;
    }
}

/**
 * Matches | at line[*cur] with its corresponding Token
 * |  | BitwiseOr
 * || | LogicalOr
 * |= | OrEquals
 */
fn match_or(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '|') {
        *cur += 1;
        return Token::LogicalOr;
    } else if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::OrEquals;
    } else {
        return Token::BitwiseOr;
    }
}

/**
 * Matches | at line[*cur] with its corresponding Token
 * ^  | BitwiseXor
 * ^= | XorEquals
 */
fn match_xor(line: &Vec<char>, cur: &mut usize) -> Token {
    if line.match_idx(*cur + 1, '=') {
        *cur += 1;
        return Token::XorEquals;
    } else {
        return Token::BitwiseXor;
    }
}

/**
 * Matches a string literal at line[*cur] with its corresponding Token
 */
fn match_string(line: &Vec<char>, cur: &mut usize) -> Token {
    let first_quote = line[*cur].to_string();
    let mut collected = String::from(first_quote.clone());
    *cur += 1;
    while *cur < line.len() {
        if line[*cur].to_string() == r"\".to_string() {
            return Token::Illegal; // TODO handle escapes
        } else if line[*cur].to_string() == first_quote {
            collected.push(line[*cur]);
            return Token::StringLiteral(collected);
        } else {
            collected.push(line[*cur]);
            *cur += 1;
        }
    }
    Token::EOF
}

/**
 * Increments cur until end of line or until a non-whitespace character
 * Returns Token::NoMatch, which is used in the match statement
 */
fn skip_whitespace(line: &Vec<char>, cur: &mut usize) -> Token {
    while *cur < line.len() && line.is_whitespace_at(*cur + 1) {
        *cur += 1;
    }
    Token::NoMatch
}

fn match_hex_literal(line: &Vec<char>, cur: &mut usize, collected: String) -> Token {
    let first_quote = line[*cur].to_string();
    let mut literal = collected.clone();
    literal.push(line[*cur]);
    *cur += 1;
    while *cur < line.len() {
        if line[*cur].to_string() == first_quote {
            literal.push(line[*cur]);
            *cur -= 1;
            return Token::HexLiteral(literal);
        } else if line.is_hex_digit_at(*cur) {
            literal.push(line[*cur]);
            *cur += 1;
        } else {
            return Token::Illegal;
        }
    }
    Token::EOF
}

/**
 * Matches an identifier or keyword at line[*cur]
 */
fn match_identifier_or_keyword(line: &Vec<char>, cur: &mut usize) -> Token {
    let mut collected = String::new();
    while *cur < line.len() && line[*cur].is_iden_or_keyword_part() {
        collected.push(line[*cur]);
        *cur += 1;
    }
    *cur -= 1;
    let mut result = match_collected(collected);
    // Special case - found "hex"
    if result == Token::Hex {
        if line.match_idx(*cur + 1, '"') || line.match_idx(*cur + 1, '\'') {
            *cur += 1;
            return match_hex_literal(line, cur, "hex".to_string());
        } else {
            return Token::Illegal;
        }
    }
    return result;
}

/**
 * Matches a leading '0'. If this does not correspond to a hex
 * number, returns Token::Illegal
 */
fn match_hex_number(line: &Vec<char>, cur: &mut usize) -> Token {
    let mut collected = String::new();
    if !line.is_hex_delim_at(*cur + 1) {
        return Token::Illegal;
    }

    collected.push(line[*cur]);
    collected.push(line[*cur + 1]);
    *cur += 2;

    while *cur < line.len() && line.is_hex_digit_at(*cur) {
        collected.push(line[*cur]);
        *cur += 1;
    }
    *cur -= 1;

    // Cannot only have '0x'
    if collected.len() <= 2 {
        return Token::Illegal;
    } else {
        return Token::HexNumber(collected);
    }
}

/**
 * Matches a decimal literal at line[*cur] and returns its Token
 */
fn match_rational(line: &Vec<char>, cur: &mut usize) -> Token {
    let mut collected = String::new();
    let mut decimal_found = false;
    let mut version_found = false;
    let mut exponent_found = false;

    while *cur < line.len() {
        if line.match_idx(*cur, '.') {
            // Cannot have a decimal after an exponent
            // If we find 2 decimals, we are parsing a Version
            // Allowed: { var a = 14.4; } || { var a = 1.4e5; }
            // Not allowed: { var a = 1e4.5; }
            if decimal_found {
                if version_found {
                    return Token::Illegal;
                } else {
                    version_found = true;
                }
            } else if exponent_found {
                return Token::Illegal;
            } else {
                decimal_found = true;
            }
        } else if line.match_idx(*cur, 'e') || line.match_idx(*cur, 'E') {
            // Cannot have 2 exponents, or an exponent in a version
            if exponent_found || version_found {
                return Token::Illegal;
            } else {
                // If we find an exponent, there must be at least 1 more digit
                // (Trailing decimals are allowed, though!)
                // Allowed: { var a = 14.; }
                // Not allowed: { var a = 14e; }
                if *cur + 1 == line.len() || !line[*cur + 1].is_digit(10) {
                    return Token::Illegal;
                } else {
                    exponent_found = true;
                }
            }
        } else if !line.is_digit_at(*cur) {
            *cur -= 1;
            if version_found {
                return Token::Version(collected);
            }
            return Token::DecimalNumber(collected);
        }
        collected.push(line[*cur]);
        *cur += 1;
    }
    *cur -= 1;
    Token::DecimalNumber(collected)
}

/**
 * Returns the next Token found in the line and increments cur
 * to the end of the Token in the parsed line
 */
pub fn next_token(line: &Vec<char>, cur: &mut usize) -> Token {
    let mut next = Token::NoMatch;
    
    loop {
        if *cur >= line.len() {
            return Token::EOF;
        }

        let t = match line[*cur] {
            ';' => Token::Semicolon,
            '{' => Token::OpenBrace,
            '}' => Token::CloseBrace,
            '(' => Token::OpenParenthesis,
            ')' => Token::CloseParenthesis,
            '[' => Token::OpenBracket,
            ']' => Token::CloseBracket,
            '?' => Token::Question,
            ',' => Token::Comma,
            '~' => Token::Tilda,
            '.' => match_period(line, cur),
            ':' => match_colon(line, cur),          // : :=
            '=' => match_equals(line, cur),         // = == =>
            '+' => match_plus(line, cur),           // + ++ +=
            '-' => match_minus(line, cur),          // - -- -=
            '*' => match_star(line, cur),           // * ** *=
            '/' => match_slash(line, cur),          // / // /* /=
            '>' => match_rarrow(line, cur),         // > >= >> >>= >>> >>>=
            '<' => match_larrow(line, cur),         // < <= << <<=
            '!' => match_exclamation(line, cur),    // ! !=
            '%' => match_percent(line, cur),        // % %=
            '&' => match_and(line, cur),            // & && &=
            '|' => match_or(line, cur),             // | || |=
            '^' => match_xor(line, cur),            // ^ ^=
            '"' | '\'' => match_string(line, cur),
            '0' => {
                if line.match_idx(*cur + 1, ' ') {
                    to_decimal_number("0")
                } else if line.is_hex_delim_at(*cur + 1) {
                    match_hex_number(line, cur)
                } else if line.match_idx(*cur + 1, '.') {
                    match_rational(line, cur)
                } else {
                    Token::Illegal
                }
            },
            non if non.is_whitespace() => skip_whitespace(line, cur),
            num if num.starts_rational() => match_rational(line, cur),
            chr if chr.starts_iden_or_keyword() => match_identifier_or_keyword(line, cur),
            _ => Token::Illegal
        };

        if t == Token::Illegal {
            return t;
        } else {
            *cur += 1;
            if t != Token::NoMatch {
                return t;
            } else if *cur >= line.len() {
                return Token::EOF;
            }
        }
    }
    next
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

    fn fail_test(expect: Token, actual: Token) {
        panic!("Expected: {:?} | Actual: {:?}", expect, actual);
    }

    fn expect_next_token(s: &Vec<char>, cur: &mut usize, t: Token) {
        match next_token(&s, cur) {
            ref next if *next == t => (),
            actual => fail_test(t, actual)
        };
    }

    /* Colon */

    #[test]
    fn test_pragma1() {
        let s = to_chars("^0.4.25;");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::BitwiseXor);
        expect_next_token(&s, cur, Token::Version(String::from("0.4.25")));
        expect_next_token(&s, cur, Token::Semicolon);
    }

    #[test]
    fn test_colon() {
        let s = to_chars(":");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Colon);
    }

    #[test]
    fn test_asmassign() {
        let s = to_chars(":=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::ASMAssign);
    }

    /* Equals  */

    #[test]
    fn test_assignment() {
        let s = to_chars("=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Assignment);
    }

    #[test]
    fn test_equals() {
        let s = to_chars("==");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Equals);
    }

    #[test]
    fn test_arrow() {
        let s = to_chars("=>");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Arrow);
    }

    /* Plus */

    #[test]
    fn test_plus() {
        let s = to_chars("+");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Plus);
    }

    #[test]
    fn test_increment() {
        let s = to_chars("++");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Increment);
    }

    #[test]
    fn test_plus_equals() {
        let s = to_chars("+=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::PlusEquals);
    }

    /* Minus */

    #[test]
    fn test_minus() {
        let s = to_chars("-");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Minus);
    }

    #[test]
    fn test_decrement() {
        let s = to_chars("--");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Decrement);
    }

    #[test]
    fn test_minus_equals() {
        let s = to_chars("-=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::MinusEquals);
    }

    /* Star */

    #[test]
    fn test_multiply() {
        let s = to_chars("*");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Multiply);
    }

    #[test]
    fn test_power() {
        let s = to_chars("**");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Power);
    }

    #[test]
    fn test_multiply_equals() {
        let s = to_chars("*=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::MultiplyEquals);
    }
    
    /* Slash */

    #[test]
    fn test_divide() {
        let s = to_chars("/");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Divide);
    }

    #[test]
    fn test_comment_single() {
        let s = to_chars("//");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::CommentSingle);
    }

    #[test]
    fn test_comment_multi() {
        let s = to_chars("/*");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::CommentMulti);
    }

    #[test]
    fn test_divide_equals() {
        let s = to_chars("/=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::DivideEquals);
    }

    /* RArrow */

    #[test]
    fn test_greater_than() {
        let s = to_chars(">");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::GreaterThan);
    }

    #[test]
    fn test_greater_than_or_equals() {
        let s = to_chars(">=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::GreaterThanOrEquals);
    }

    #[test]
    fn test_shift_right() {
        let s = to_chars(">>");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::ShiftRight);
    }

    #[test]
    fn test_shift_right_equals() {
        let s = to_chars(">>=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::ShiftRightEquals);
    }

    #[test]
    fn test_thing_0() { // TODO
        let s = to_chars(">>>");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Illegal);
    }

    #[test]
    fn test_thing_1() { // TODO
        let s = to_chars(">>>=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Illegal);
    }

    /* LArrow */

    #[test]
    fn test_less_than() {
        let s = to_chars("<");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::LessThan);
    }

    #[test]
    fn test_less_than_or_equals() {
        let s = to_chars("<=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::LessThanOrEquals);
    }

    #[test]
    fn test_shift_left() {
        let s = to_chars("<<");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::ShiftLeft);
    }

    #[test]
    fn test_shift_left_equals() {
        let s = to_chars("<<=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::ShiftLeftEquals);
    }

    /* Exclamation */

    #[test]
    fn test_exclamation() {
        let s = to_chars("!");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Exclamation);
    }

    #[test]
    fn test_not_equals() {
        let s = to_chars("!=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::NotEquals);
    }

    /* Percent */

    #[test]
    fn test_modulus() {
        let s = to_chars("%");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Modulus);
    }

    #[test]
    fn test_mod_equals() {
        let s = to_chars("%=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::ModEquals);
    }

    /* And */

    #[test]
    fn test_bitwise_and() {
        let s = to_chars("&");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::BitwiseAnd);
    }

    #[test]
    fn test_logical_and() {
        let s = to_chars("&&");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::LogicalAnd);
    }

    #[test]
    fn test_and_equals() {
        let s = to_chars("&=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::AndEquals);
    }

    /* Or */

    #[test]
    fn test_bitwise_or() {
        let s = to_chars("|");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::BitwiseOr);
    }

    #[test]
    fn test_logical_or() {
        let s = to_chars("||");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::LogicalOr);
    }

    #[test]
    fn test_or_equals() {
        let s = to_chars("|=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::OrEquals);
    }

    /* Xor */

    #[test]
    fn test_bitwise_xor() {
        let s = to_chars("^");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::BitwiseXor);
    }

    #[test]
    fn test_xor_equals() {
        let s = to_chars("^=");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::XorEquals);
    }

    /* StringLiteral */

    #[test]
    fn test_string_literal_0() {
        let s = to_chars("\"\"");
        let cur = &mut 0; 
        expect_next_token(&s, cur, Token::StringLiteral(s.as_string()));
    }

    #[test]
    fn test_string_literal_1() {
        let s = to_chars("''");
        let cur = &mut 0; 
        expect_next_token(&s, cur, Token::StringLiteral(s.as_string()));
    }

    #[test]
    fn test_string_literal_2() {
        let s = to_chars("\"test.sol\"");
        let cur = &mut 0; 
        expect_next_token(&s, cur, Token::StringLiteral(s.as_string()));
    }

    /* Whitespace */

    #[test]
    fn test_whitespace_0() {
        let s = to_chars(" ++");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Increment);
    }

    #[test]
    fn test_whitespace_1() {
        let s = to_chars("  ++  +");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Increment);
        expect_next_token(&s, cur, Token::Plus);
    }

    #[test]
    fn test_whitespace_3() {
        let s = to_chars(" ++  -- / \"literal\"");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Increment);
        expect_next_token(&s, cur, Token::Decrement);
        expect_next_token(&s, cur, Token::Divide);
        expect_next_token(&s, cur, to_string_literal("\"literal\""));
    }

    /* Number literals */

    #[test]
    fn test_numbers_0() {
        let s = to_chars("1 12+123");
        let cur = &mut 0;
        expect_next_token(&s, cur, to_decimal_number("1"));
        expect_next_token(&s, cur, to_decimal_number("12"));
        expect_next_token(&s, cur, Token::Plus);
        expect_next_token(&s, cur, to_decimal_number("123"));
    }

    #[test]
    fn test_numbers_1() {
        let s = to_chars("0 0.1");
        let cur = &mut 0;
        expect_next_token(&s, cur, to_decimal_number("0"));
        expect_next_token(&s, cur, to_decimal_number("0.1"));
    }

    #[test]
    fn test_numbers_2() {
        let s = to_chars("01234");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Illegal);
    }

    #[test]
    fn test_numbers_3() {
        let s = to_chars("1.2e3 4E5");
        let cur = &mut 0;
        expect_next_token(&s, cur, to_decimal_number("1.2e3"));
        expect_next_token(&s, cur, to_decimal_number("4E5"));
    }

    #[test]
    fn test_numbers_4() {
        let s = to_chars(".14 .Iden Iden.Iden");
        let cur = &mut 0;
        expect_next_token(&s, cur, to_decimal_number(".14"));
        expect_next_token(&s, cur, Token::Dot);
        expect_next_token(&s, cur, to_identifier("Iden"));
        expect_next_token(&s, cur, to_identifier("Iden"));
        expect_next_token(&s, cur, Token::Dot);
        expect_next_token(&s, cur, to_identifier("Iden"));
    }

    #[test]
    fn test_hex_numbers_0() {
        let s = to_chars("0x");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Illegal);
    }

    #[test]
    fn test_hex_numbers_1() {
        let s = to_chars("0xFF 0 0xF");
        let cur = &mut 0;
        expect_next_token(&s, cur, to_hex_number("0xFF"));
        expect_next_token(&s, cur, to_decimal_number("0"));
        expect_next_token(&s, cur, to_hex_number("0xF"));
    }

    #[test]
    fn test_hex_numbers_2() {
        let s = to_chars("0xdf 0xZZ");
        let cur = &mut 0;
        expect_next_token(&s, cur, to_hex_number("0xdf"));
        expect_next_token(&s, cur, Token::Illegal);
    }

    /* Keywords */

    #[test]
    fn test_address() {
        let s = to_chars("address");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Address);
    }

    #[test]
    fn test_anonymous() {
        let s = to_chars("anonymous");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Anonymous);
    }

    #[test]
    fn test_as() {
        let s = to_chars("as");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::As);
    }

    #[test]
    fn test_assembly() {
        let s = to_chars("assembly");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Assembly);
    }

    #[test]
    fn test_bool() {
        let s = to_chars("bool");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Bool);
    }

    #[test]
    fn test_break() {
        let s = to_chars("break");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Break);
    }

    #[test]
    fn test_byte() {
        let s = to_chars("byte");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Byte);
    }

    #[test]
    fn test_bytes() {
        let s = to_chars("bytes");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Bytes);
    }

    #[test]
    fn test_bytes1() {
        let s = to_chars("bytes1");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Bytes1);
    }

    #[test]
    fn test_bytes32() {
        let s = to_chars("bytes32");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Bytes32);
    }

    #[test]
    fn test_constant() {
        let s = to_chars("constant");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Constant);
    }

    #[test]
    fn test_continue() {
        let s = to_chars("continue");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Continue);
    }

    #[test]
    fn test_contract() {
        let s = to_chars("contract");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Contract);
    }

    #[test]
    fn test_days() {
        let s = to_chars("days");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Days);
    }

    #[test]
    fn test_delete() {
        let s = to_chars("delete");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Delete);
    }

    #[test]
    fn test_do() {
        let s = to_chars("do");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Do);
    }

    #[test]
    fn test_else() {
        let s = to_chars("else");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Else);
    }

    #[test]
    fn test_emit() {
        let s = to_chars("emit");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Emit);
    }

    #[test]
    fn test_enum() {
        let s = to_chars("enum");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Enum);
    }

    #[test]
    fn test_ether() {
        let s = to_chars("ether");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Ether);
    }

    #[test]
    fn test_event() {
        let s = to_chars("event");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Event);
    }

    #[test]
    fn test_external() {
        let s = to_chars("external");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::External);
    }

    #[test]
    fn test_false() {
        let s = to_chars("false");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::False);
    }

    #[test]
    fn test_finney() {
        let s = to_chars("finney");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Finney);
    }

    #[test]
    fn test_fixed() {
        let s = to_chars("fixed");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Fixed);
    }

    #[test]
    fn test_for() {
        let s = to_chars("for");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::For);
    }

    #[test]
    fn test_from() {
        let s = to_chars("from");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::From);
    }

    #[test]
    fn test_function() {
        let s = to_chars("function");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Function);
    }

    #[test]
    fn test_hex() {
        let s = to_chars("hex");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Illegal);
    }

    #[test]
    fn test_hex_literal1() {
        let s = to_chars("hex\"DEADBEEF\"");
        let cur = &mut 0;
        expect_next_token(&s, cur, to_hex_literal("hex\"DEADBEEF\""));
    }

    #[test]
    fn test_hex_literal2() {
        let s = to_chars("hex\"ZZZZ\"");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Illegal);
    }

    #[test]
    fn test_hours() {
        let s = to_chars("hours");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Hours);
    }

    #[test]
    fn test_if() {
        let s = to_chars("if");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::If);
    }

    #[test]
    fn test_import() {
        let s = to_chars("import");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Import);
    }

    #[test]
    fn test_indexed() {
        let s = to_chars("indexed");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Indexed);
    }

    #[test]
    fn test_int() {
        let s = to_chars("int");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Int);
    }

    #[test]
    fn test_int8() {
        let s = to_chars("int8");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Int8);
    }

    #[test]
    fn test_int16() {
        let s = to_chars("int16");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Int16);
    }

    #[test]
    fn test_int256() {
        let s = to_chars("int256");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Int256);
    }

    #[test]
    fn test_interface() {
        let s = to_chars("interface");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Interface);
    }

    #[test]
    fn test_internal() {
        let s = to_chars("internal");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Internal);
    }

    #[test]
    fn test_is() {
        let s = to_chars("is");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Is);
    }

    #[test]
    fn test_let() {
        let s = to_chars("let");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Let);
    }

    #[test]
    fn test_library() {
        let s = to_chars("library");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Library);
    }

    #[test]
    fn test_mapping() {
        let s = to_chars("mapping");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Mapping);
    }

    #[test]
    fn test_memory() {
        let s = to_chars("memory");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Memory);
    }

    #[test]
    fn test_minutes() {
        let s = to_chars("minutes");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Minutes);
    }

    #[test]
    fn test_modifier() {
        let s = to_chars("modifier");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Modifier);
    }

    #[test]
    fn test_new() {
        let s = to_chars("new");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::New);
    }

    #[test]
    fn test_payable() {
        let s = to_chars("payable");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Payable);
    }

    #[test]
    fn test_pragma() {
        let s = to_chars("pragma");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Pragma);
    }

    #[test]
    fn test_private() {
        let s = to_chars("private");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Private);
    }

    #[test]
    fn test_public() {
        let s = to_chars("public");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Public);
    }
    
    #[test]
    fn test_pure() {
        let s = to_chars("pure");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Pure);
    }

    #[test]
    fn test_return() {
        let s = to_chars("return");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Return);
    }

    #[test]
    fn test_returns() {
        let s = to_chars("returns");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Returns);
    }

    #[test]
    fn test_seconds() {
        let s = to_chars("seconds");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Seconds);
    }

    #[test]
    fn test_storage() {
        let s = to_chars("storage");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Storage);
    }

    #[test]
    fn test_string() {
        let s = to_chars("string");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::String);
    }

    #[test]
    fn test_struct() {
        let s = to_chars("struct");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Struct);
    }

    #[test]
    fn test_szabo() {
        let s = to_chars("szabo");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Szabo);
    }

    #[test]
    fn test_throw() {
        let s = to_chars("throw");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Throw);
    }

    #[test]
    fn test_true() {
        let s = to_chars("true");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::True);
    }

    #[test]
    fn test_ufixed() {
        let s = to_chars("ufixed");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Ufixed);
    }

    #[test]
    fn test_uint() {
        let s = to_chars("uint");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Uint);
    }

    #[test]
    fn test_uint8() {
        let s = to_chars("uint8");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Uint8);
    }

    #[test]
    fn test_uint16() {
        let s = to_chars("uint16");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Uint16);
    }

    #[test]
    fn test_uint256() {
        let s = to_chars("uint256");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Uint256);
    }

    #[test]
    fn test_using() {
        let s = to_chars("using");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Using);
    }

    #[test]
    fn test_var() {
        let s = to_chars("var");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Var);
    }

    #[test]
    fn test_view() {
        let s = to_chars("view");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::View);
    }

    #[test]
    fn test_weeks() {
        let s = to_chars("weeks");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Weeks);
    }

    #[test]
    fn test_wei() {
        let s = to_chars("wei");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Wei);
    }

    #[test]
    fn test_while() {
        let s = to_chars("while");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::While);
    }

    #[test]
    fn test_years() {
        let s = to_chars("years");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Years);
    }

    #[test]
    fn test_placeholder() {
        let s = to_chars("_");
        let cur = &mut 0;
        expect_next_token(&s, cur, Token::Placeholder);
    }
}
