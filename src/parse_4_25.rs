use super::lex_4_25;

// TODO(jalextowle): Add proper documentation to this enum list. It may be appropriate to add this
// and the lex_4_25::Token enum to a file called `enumerations.rs` or something similar.
#[derive(Clone, Debug, PartialEq)]
pub enum NonTerminal {
    // Solidity Nonterminals
    SourceUnit,
    PragmaDirective,
    ImportDirective,
    ContractDefinition(lex_4_25::Token),
    ContractPart,
    InheritanceList,
    InheritanceSpecifier,
    StateVariableDeclaration,
    UsingForDeclaration,
    StructDefinition,
    ModifierDefinition,
    ModifierInvocation,
    FunctionDefinition,
    EventDefinition,
    EnumValue(String),
    EnumValueList,
    EnumDefinition,
    ParameterList,
    Parameter,
    EventParameterList,
    EventParameter,
    FunctionTypeParameterList,
    FunctionTypeParameter,
    VariableDeclaration,
    TypeName,
    UserDefinedTypeName,
    Mapping,
    ArrayTypeName,
    FunctionTypeName,
    StorageLocation(lex_4_25::Token),
    StateMutability(lex_4_25::Token),
    Block,
    Statement,
    ExpressionStatement,
    IfStatement,
    WhileStatement,
    PlaceholderStatement,
    SimpleStatement,
    ForStatement,
    InlineAssemblyStatement,
    DoWhileStatement,
    Continue,
    Break,
    Return,
    Throw,
    EmitStatement,
    VariableDefinition,
    IdentifierList,
    Expression,
    PrimaryExpression,
    ExpressionList,
    NameValueList,
    FunctionCall,
    FunctionCallArguments,
    NewExpression,
    MemberAccess,
    IndexAccess,
    BooleanLiteral(String),
    NumberLiteral(String),
    NumberUnit,
    HexLiteral(String),
    StringLiteral(String),
    Identifier(String),
    HexNumber,
    DecimalNumber,
    TupleExpression,
    ElementaryTypeNameExpression,
    ElementaryTypeName,
    Int,
    Uint,
    Byte,
    Fixed,
    Ufixed,
    InlineAssemblyBlock,
    AssemblyItem,
    AssemblyLocalBinding,
    AssemblyAssignment,
    AssemblyLabel,
    FunctionalAssemblyExpression,
    // Catch-All for Tokens that do not fit cleanly into this hierarchy (yet)
    Token(lex_4_25::Token),
    // Error NonTerminals
    Invalid(Box<NonTerminal>),
    InvalidPair(Box<NonTerminal>, Box<NonTerminal>),
    Empty,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParseTree {
    pub root: NonTerminal,
    pub leaves: Vec<Box<ParseTree>>
}

impl ParseTree {
    fn add_leaf(&mut self, nonterminal: NonTerminal) {
        self.leaves.push(Box::new(ParseTree{ root: nonterminal, leaves: vec![] }));
    }

    fn add_invalid(&mut self, expected: NonTerminal) {
        self.add_leaf(NonTerminal::Invalid(Box::new(expected)));
    }

    // TODO(jalextowle): It would be better to have `expected` and `actual` for debugging
    fn add_invalid_token(&mut self, expected: lex_4_25::Token) {
        self.add_leaf(NonTerminal::Invalid(Box::new(NonTerminal::Token(expected))));
    }

    fn add_token(&mut self, token: lex_4_25::Token) {
        self.add_leaf(NonTerminal::Token(token));
    }

    fn add_tree(&mut self, other: ParseTree) {
        self.leaves.push(Box::new(other));
    }

    fn remove_left_leaf(&mut self) -> Box<ParseTree> { self.leaves.remove(0) }

    /**
     * Merge other with self, returning the new parent node
     * self recieves other.children[0], which is replaced by a pointer to self
     */
    fn merge_expressions(mut self, mut other: ParseTree) -> ParseTree {
        self.leaves.push(other.remove_left_leaf());
        other.leaves.push(Box::new(self));
        other.leaves.swap(0, 1);
        other
    }

    // Returns a new, empty parse tree
    fn empty() -> ParseTree {
        ParseTree { root: NonTerminal::Empty, leaves: vec![] }
    }
}

impl lex_4_25::Token {
    pub fn next_identifier_name(input: &Vec<char>, current_ptr: &mut usize) -> String {
        return match lex_4_25::next_token(input, current_ptr) {
            lex_4_25::Token::Identifier(name) => name,
            _ => String::from("")
        }
    }

    pub fn to_token(self) -> NonTerminal {
        NonTerminal::Token(self)
    }

    pub fn to_invalid(self) -> NonTerminal {
        NonTerminal::Invalid(Box::new(self.to_token()))
    }

    pub fn to_invalid_pair(self, actual: lex_4_25::Token) -> NonTerminal {
        NonTerminal::InvalidPair(Box::new(self.to_token()), Box::new(actual.to_token()))
    }
}

impl NonTerminal {
    pub fn next_to_identifier(input: &Vec<char>, current_ptr: &mut usize) -> NonTerminal {
        return match lex_4_25::next_token(input, current_ptr) {
            lex_4_25::Token::Identifier(name) => NonTerminal::Identifier(name),
            _ => NonTerminal::Identifier("".to_string()).to_invalid()
        }
    }

    pub fn next_token(input: &Vec<char>, current_ptr: &mut usize) -> NonTerminal {
        NonTerminal::Token(lex_4_25::next_token(input, current_ptr))
    }

    pub fn to_leaf(self) -> ParseTree {
        ParseTree { root: self, leaves: vec![] }
    }

    pub fn to_invalid(self) -> NonTerminal {
        NonTerminal::Invalid(Box::new(self))
    }

    pub fn to_invalid_pair(self, actual: NonTerminal) -> NonTerminal {
        NonTerminal::InvalidPair(Box::new(self), Box::new(actual))
    }

    pub fn to_invalid_token_pair(self, actual: lex_4_25::Token) -> NonTerminal {
        NonTerminal::InvalidPair(Box::new(self), Box::new(actual.to_token()))
    }
}

/*** Top-Level ***/

// Parses the input contract and returns its ParseTree
pub fn parse(input_string: String) -> ParseTree {
    let mut tree = NonTerminal::SourceUnit.to_leaf();
    let current_ptr = &mut 0;
    let input = &mut input_string.chars().collect::<Vec<char>>();
    while *current_ptr < input.len() {
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::Pragma => {
                tree.add_tree(parse_pragma_directive(input, current_ptr));
            }
            lex_4_25::Token::Import => {
                tree.add_tree(parse_import_directive(input, current_ptr));
            }
            lex_4_25::Token::Contract  |
            lex_4_25::Token::Library   |
            lex_4_25::Token::Interface => {
                tree.add_tree(parse_contract_definition(input, current_ptr));
            }
            lex_4_25::Token::EOF => {
                lex_4_25::next_token(input, current_ptr);
            }
            actual => {
                lex_4_25::next_token(input, current_ptr);
                tree.root = NonTerminal::SourceUnit.to_invalid_token_pair(actual);
            }
        }
    }
    tree
}

/*** Pragma ***/

/**
 * @dev Parses a PragmaDirective and writes any errors that are found into the ParseTree for
 *      debugging.
 * @param input The characters from the input that should be read.
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that either represents a PragmaDirective.
 */
pub fn parse_pragma_directive(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::PragmaDirective.to_leaf();
    // Expect a Pragma token. If not found, set the root to an invalid token
    // for later debugging.
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Pragma => (),
        _ => tree.root = NonTerminal::PragmaDirective.to_invalid()
    }
    // Expect an Identifier token. If not found, set add an invalid token as a leaf
    // for later debugging.
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Identifier(name) => tree.add_leaf(NonTerminal::Identifier(name)),
        // TODO(jalextowle): We currently expect `solidity`, but this may change.
        _ => tree.add_invalid(NonTerminal::Identifier(String::from("solidity")))
    }
    // If the next token is a BitwiseXor, it is likely part of the version, and it
    // should be added as a leaf to the tree.
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::BitwiseXor => tree.add_token(lex_4_25::next_token(&input, current_ptr)),
        _ => ()
    }
    // Expect a Version token.
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Version(version) => tree.add_token(lex_4_25::Token::Version(version)),
        _ => tree.add_invalid_token(lex_4_25::Token::Version(String::from("")))
    }
    // Expect a Semicolon token.
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Semicolon => (),
        _ => tree.add_invalid_token(lex_4_25::Token::Semicolon)
    }
    tree
}

/*** Import ***/

/**
 * @dev Parses an ImportDirective nonterminal.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that either represents a valid contract defition or detailed error
 *         information.
 */
fn parse_import_directive(_input: &Vec<char>, _current_ptr: &mut usize) -> ParseTree { ParseTree::empty() }

/*** Contract ***/

/**
 * @dev Parses a contract definition nonterminal.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that either represents a valid contract defition or detailed error
 *         information.
 */
fn parse_contract_definition(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = ParseTree::empty();
    // Look at the next token. The expected token is a Contract, Interface, or Library token.
    // Anything else is Invalid.
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Contract => tree.root = NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
        lex_4_25::Token::Interface => tree.root = NonTerminal::ContractDefinition(lex_4_25::Token::Interface),
        lex_4_25::Token::Library => tree.root = NonTerminal::ContractDefinition(lex_4_25::Token::Library),
        _ => tree.root = NonTerminal::ContractDefinition(lex_4_25::Token::Contract).to_invalid()
    }
    // The next token should be the Contract name. If an identifier is not found, add an Invalid
    // leaf to the ParseTree.
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Identifier(name) => tree.add_leaf(NonTerminal::Identifier(name)),
        _ => tree.add_invalid(NonTerminal::Identifier(String::from("")))
    }
    // Determine if the ContractDefinition specifies an inheritance hierarchy for the contract that
    // is being defined.
    let mut inheritance = false;
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::OpenBrace => (),
        lex_4_25::Token::Is => inheritance = true,
        // TODO(jalextowle): It would be nice to be able to add a list of expected tokens so that
        // this could be completely accurate.
        _ => tree.add_invalid_token(lex_4_25::Token::Is)
    }
    if inheritance {
        tree.add_tree(parse_inheritance_list(input, current_ptr));
    }
    // If an inheritance hierarchy was defined, parse the inheritance hierarchy.
    // Parse the contract part and add it to the tree.
    tree.add_tree(parse_contract_part(input, current_ptr));
    tree
}

/**
 * @dev Parses an InheritanceList nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an inheritance list.
 */
fn parse_inheritance_list(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::InheritanceList.to_leaf();
    // Expect an Is token
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Is => (),
        actual => tree.root = NonTerminal::InheritanceList.to_invalid_token_pair(actual)
    }
    let mut stop = false;
    while !stop {
        tree.add_tree(parse_inheritance_specifier(input, current_ptr));
        if let lex_4_25::Token::Comma = lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::next_token(input, current_ptr);
        } else {
            stop = true;
        }
    }
    tree
}

/**
 * @dev Parse an InheritanceSpecifier nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an inheritance list.
 */
fn parse_inheritance_specifier(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::InheritanceSpecifier.to_leaf();
    tree.add_tree(parse_user_defined_type_name(input, current_ptr));
    if let lex_4_25::Token::OpenParenthesis = lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::next_token(input, current_ptr);
        tree.add_tree(parse_expression_list(input, current_ptr));
        match lex_4_25::next_token(input, current_ptr) {
            lex_4_25::Token::CloseParenthesis => (),
            _ => tree.add_invalid_token(lex_4_25::Token::CloseParenthesis)
        }
    }
    tree
}

/**
 * @dev Parse a ContractPart nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a contract block.
 */
fn parse_contract_part(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::ContractPart.to_leaf();
    // Expect an open brace
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::OpenBrace => (),
        actual => tree.add_leaf(lex_4_25::Token::OpenBrace.to_invalid_pair(actual))
    }
    let mut stop = false;
    while !stop {
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::Enum => tree.add_tree(parse_enum_definition(input, current_ptr)),
            lex_4_25::Token::Event => tree.add_tree(parse_event_definition(input, current_ptr)),
            lex_4_25::Token::Function => tree.add_tree(parse_function_definition(input, current_ptr)),
            lex_4_25::Token::Modifier => tree.add_tree(parse_modifier_definition(input, current_ptr)),
            lex_4_25::Token::Using => tree.add_tree(parse_using_for_declaration(input, current_ptr)),
            lex_4_25::Token::Struct => tree.add_tree(parse_struct_definition(input, current_ptr)),
            lex_4_25::Token::CloseBrace => stop = true,
            _ => tree.add_tree(parse_state_variable_declaration(input, current_ptr))
        }
    }
    // Expect a close brace
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::CloseBrace => (),
        actual => tree.add_leaf(lex_4_25::Token::CloseBrace.to_invalid_pair(actual))
    }
    tree
}

/**
 * @dev Parse a StructDefinition nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a struct definition.
 */
fn parse_struct_definition(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree { ParseTree::empty() }

/**
 * @dev Parse a StateVariableDeclaration nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a state variable declaration.
 */
fn parse_state_variable_declaration(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::StateVariableDeclaration.to_leaf();
    tree.add_tree(parse_type_name(input, current_ptr));
    let mut stop = false;
    while !stop {
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::Constant |
            lex_4_25::Token::Internal |
            lex_4_25::Token::Private  |
            lex_4_25::Token::Public   => tree.add_token(lex_4_25::next_token(input, current_ptr)),
            _ => stop = true
        }
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Identifier(name) => tree.add_leaf(NonTerminal::Identifier(name)),
        _ => tree.add_invalid(NonTerminal::Identifier(String::from("")))
    }
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Assignment => {
            tree.add_token(lex_4_25::next_token(input, current_ptr));
            let last = tree.leaves.len() - 1;
            tree.leaves[last].add_tree(parse_expression(input, current_ptr));
        }
        _ => ()
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Semicolon => (),
        _ => tree.add_invalid_token(lex_4_25::Token::Semicolon)
    }
    tree
}

/**
 * @dev Parse a EnumDefinition nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an enum definition.
 */
fn parse_enum_definition(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::EnumDefinition.to_leaf();
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Enum => (),
        _ => tree.root = NonTerminal::EnumDefinition.to_invalid()
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Identifier(name) => tree.add_leaf(NonTerminal::Identifier(name)),
        _ => tree.add_invalid(NonTerminal::Identifier(String::from("")))
    }
    tree.add_tree(parse_enum_value_list(input, current_ptr));
    tree
}

/**
 * @dev Parse a EnumValueList nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an enum value list.
 */
fn parse_enum_value_list(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::EnumValueList.to_leaf();
    // Expect an open brace to start the enum value list
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::OpenBrace => (),
        _ => tree.add_invalid_token(lex_4_25::Token::OpenBrace)
    }
    // Loop until an enum value is no longer expected
    let mut stop = false;
    while !stop {
        // If an identifier is found, add it as an EnumValue to the EnumValueList. Otherwise, break
        // out of the loop.
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::Identifier(name) => {
                tree.add_leaf(NonTerminal::EnumValue(name));
                lex_4_25::next_token(input, current_ptr);
            }
            _ => stop = true
        }
        // If the loop should continue, check for a comma. If a comma is found, the loop can
        // continue, but if a comma is not found, break out of the loop.
        if !stop {
            if let lex_4_25::Token::Comma = lex_4_25::peek_token(input, current_ptr) {
                lex_4_25::next_token(input, current_ptr);
            } else {
                stop = true;
            }
        }
    }
    // Expect a close brace to end the enum value list
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::CloseBrace => (),
        _ => tree.add_invalid_token(lex_4_25::Token::CloseBrace)
    }
    tree
}

/**
 * @dev Parse a UsingForDeclaration nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a using for declaration.
 */
fn parse_using_for_declaration(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::UsingForDeclaration.to_leaf();
    // Expect a Using token
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Using => (),
        _ => tree.add_invalid_token(lex_4_25::Token::Using)
    }
    // Expect an identifier
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Identifier(name) => tree.add_leaf(NonTerminal::Identifier(name)),
        _ => tree.add_invalid_token(lex_4_25::Token::Identifier(String::from("")))
    }
    // Expect a For token
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::For => (),
        _ => tree.add_invalid_token(lex_4_25::Token::For)
    }
    // Expect a Multiply token or a TypeName
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Multiply => tree.add_token(lex_4_25::next_token(input, current_ptr)),
        _ => tree.add_tree(parse_type_name(input, current_ptr))
    }
    // Expect a Semicolon token
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Semicolon => (),
        _ => tree.add_invalid_token(lex_4_25::Token::Semicolon)
    }
    tree
}

/**
 * @dev Parse an EventDefinition nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a event definition.
 */
fn parse_event_definition(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::EventDefinition.to_leaf();
    // Expect an Event token
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Event => (),
        _ => tree.add_invalid(NonTerminal::EventDefinition)
    }
    // Expect an Identifier
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Identifier(name) => tree.add_leaf(NonTerminal::Identifier(name)),
        _ => tree.add_invalid_token(lex_4_25::Token::Identifier("".to_string()))
    }
    // Parse the parameter list of this event
    tree.add_tree(parse_event_parameter_list(input, current_ptr));
    // If the parser finds an Anonymous token, add it to the tree.
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Anonymous => tree.add_token(lex_4_25::next_token(input, current_ptr)),
        _ => (),
    }
    // Expect a Semicolon token.
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Semicolon => (),
        _ => tree.add_invalid_token(lex_4_25::Token::Semicolon)
    }
    tree
}

/**
 * @dev Parse an EventParameterList nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a event parameter list.
 */
fn parse_event_parameter_list(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::EventParameterList.to_leaf();
    // Expect an OpenParenthesis token.
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::OpenParenthesis => (),
        _ => tree.root = NonTerminal::EventParameterList.to_invalid()
    }
    let mut stop = false;
    while !stop {
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::CloseParenthesis => stop = true,
            _ => tree.add_tree(parse_event_parameter(input, current_ptr))
        }
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::Comma => {
                lex_4_25::next_token(input, current_ptr);
            }
            lex_4_25::Token::CloseParenthesis => stop = true,
            // TODO(jalextowle): This would benefit from multiple expected values
            _ => tree.add_invalid_token(lex_4_25::Token::CloseParenthesis)
        }
    }
    // Expect a CloseParenthesis token.
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::CloseParenthesis => (),
        _ => tree.add_invalid_token(lex_4_25::Token::CloseParenthesis)
    }
    tree
}

/**
 * @dev Parse an EventParameter nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an event parameter.
 */
fn parse_event_parameter(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::EventParameter.to_leaf();
    tree.add_tree(parse_type_name(input, current_ptr));
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Indexed => tree.add_token(lex_4_25::next_token(input, current_ptr)),
        _ => ()
    }
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Identifier(..) => tree.add_leaf(NonTerminal::next_to_identifier(input, current_ptr)),
        lex_4_25::Token::From => {
            lex_4_25::next_token(input, current_ptr);
            tree.add_leaf(NonTerminal::Identifier(String::from("from")));
        }
        _ => ()
    }
    tree
}

/**
 * @dev Parse a ModifierDefinition nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an modifier definition.
 */
fn parse_modifier_definition(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::ModifierDefinition.to_leaf();
    // Expect a Modifier token
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Modifier => (),
        _ => tree.add_invalid(NonTerminal::ModifierDefinition)
    }
    // Expect an Identifier
    tree.add_leaf(NonTerminal::next_to_identifier(input, current_ptr));
    // If the next token is an OpenParenthesis, parse a parameter list.
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::OpenParenthesis => tree.add_tree(parse_parameter_list(input, current_ptr)),
        _ => ()
    }
    tree.add_tree(parse_block(input, current_ptr));
    tree
}

/**
 * @dev Parse a FunctionDefinition nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a function definition.
 */
fn parse_function_definition(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::FunctionDefinition.to_leaf();
    // Expect a Function token
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Function => (),
        _ => tree.add_invalid_token(lex_4_25::Token::Function)
    }
    // Expect an Identifier
    tree.add_leaf(NonTerminal::next_to_identifier(input, current_ptr));
    // Parse the function parameter list
    tree.add_tree(parse_parameter_list(input, current_ptr));
    let mut stop = false;
    while !stop {
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::External |
            lex_4_25::Token::Internal |
            lex_4_25::Token::Public   |
            lex_4_25::Token::Private  => tree.add_token(lex_4_25::next_token(input, current_ptr)),
            lex_4_25::Token::Pure     |
            lex_4_25::Token::Constant |
            lex_4_25::Token::View     |
            lex_4_25::Token::Payable => tree.add_leaf(NonTerminal::StateMutability(lex_4_25::next_token(input, current_ptr))),
            lex_4_25::Token::Identifier(..) => tree.add_tree(parse_modifier_invocation(input, current_ptr)),
            _ => stop = true
        }
    }
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Returns => {
            tree.add_token(lex_4_25::next_token(input, current_ptr));
            tree.add_tree(parse_parameter_list(input, current_ptr));
        }
        _ => (),
    }
    tree.add_tree(parse_block(input, current_ptr));
    tree
}

/**
 * @dev Parse an ModifierInvocation nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an modifier invocation.
 */
fn parse_modifier_invocation(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree { ParseTree::empty() }

/**
 * @dev Parse a ParameterList nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a parameter list.
 */
fn parse_parameter_list(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::ParameterList.to_leaf();
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::OpenParenthesis => (),
        // TODO(jalextowle): It may make sense to expand ParameterList (and all nonterminals like
        // it) to take in two lex::Tokens that represent the start and end. In this way, error
        // information would be included more cleanly.
        _ => tree.root = tree.root.to_invalid()
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::CloseParenthesis => (),
        _ => tree.add_invalid_token(lex_4_25::Token::CloseParenthesis)
    }
    tree
}

/**
 * @dev Parse a Parameter nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a parameter.
 */
fn parse_parameter(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree { ParseTree::empty() }

/**
 * @dev Parse a Block nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a block.
 */
fn parse_block(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::Block.to_leaf();
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::OpenBrace => (),
        _ => tree.root = tree.root.to_invalid()
    }
    let mut stop = false;
    while !stop {
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::CloseBrace => stop = true,
            _ => tree.add_tree(parse_statement(input, current_ptr))
        }
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::CloseBrace => (),
        // TODO(jalextowle): Change this to be included directly in the root
        _ => tree.add_invalid_token(lex_4_25::Token::CloseBrace)
    }
    tree
}

/*** Statements ***/

/**
 * @dev Parse an Statement nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a statement.
 */
fn parse_statement(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    return match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::If => parse_if_statement(input, current_ptr),
        lex_4_25::Token::While => parse_while_statement(input, current_ptr),
        lex_4_25::Token::For => parse_for_statement(input, current_ptr),
        lex_4_25::Token::Assembly => parse_inline_assembly_statement(input, current_ptr),
        lex_4_25::Token::Do => parse_do_while_statement(input, current_ptr),
        lex_4_25::Token::Placeholder => {
            lex_4_25::next_token(input, current_ptr);
            match lex_4_25::next_token(input, current_ptr) {
                lex_4_25::Token::Semicolon => (),
                _ => return lex_4_25::Token::Placeholder.to_invalid().to_leaf()
            }
            NonTerminal::Token(lex_4_25::Token::Placeholder).to_leaf()
        }
        lex_4_25::Token::Emit => parse_emit_statement(input, current_ptr),
        // TODO: This actually should be parse_variable_declaration | parse_expression
        _ => {
            let mut tree = parse_expression(input, current_ptr);
            match lex_4_25::next_token(input, current_ptr) {
                lex_4_25::Token::Semicolon => (),
                // TODO(jalextowle): This should likely use a similar strategy of error reporting
                // to blocks.
                _ => tree.add_invalid_token(lex_4_25::Token::Semicolon)
            }
            tree
        }
    }
}

/**
 * @dev Parse an IfStatement nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an if statement.
 */
fn parse_if_statement(_input: &Vec<char>, _current_ptr: &mut usize) -> ParseTree { ParseTree::empty() }

/**
 * @dev Parse a WhileStatement nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a while statement.
 */
fn parse_while_statement(_input: &Vec<char>, _current_ptr: &mut usize) -> ParseTree { ParseTree::empty() }

/**
 * @dev Parse a ForStatement nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a for statement.
 */
fn parse_for_statement(_input: &Vec<char>, _current_ptr: &mut usize) -> ParseTree { ParseTree::empty() }

/**
 * @dev Parse an InlineAssemblyStatement nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an inline assembly.
 */
fn parse_inline_assembly_statement(_input: &Vec<char>, _current_ptr: &mut usize) -> ParseTree { ParseTree::empty() }

/**
 * @dev Parse a DoWhile nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a do-while loop.
 */
fn parse_do_while_statement(_input: &Vec<char>, _current_ptr: &mut usize) -> ParseTree { ParseTree::empty() }

/**
 * @dev Parse an EmitStatement nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an emit statement.
 */
fn parse_emit_statement(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::EmitStatement.to_leaf();
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Emit => (),
        _ => tree.root = tree.root.to_invalid()
    }
    tree.add_tree(parse_expression(input, current_ptr));
    match &tree.leaves[0].root {
        NonTerminal::FunctionCall => (),
        _ => tree.add_invalid(NonTerminal::FunctionCall)
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Semicolon => (),
        _ => tree.add_invalid_token(lex_4_25::Token::Semicolon)
    }
    tree
}

/*** Expression ***/

/**
 * @note TODO(jalextowle): This function and parse_expression need to be replaced.
 * @dev Parse an Operation nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an event parameter.
 */
fn parse_operation(input: &Vec<char>, current_ptr: &mut usize, left: ParseTree) -> ParseTree {
    let mut tree = ParseTree::empty();
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Decrement | lex_4_25::Token::Increment => {
            tree.root = NonTerminal::Token(lex_4_25::next_token(input, current_ptr));
            tree.add_tree(left);
            return parse_operation(input, current_ptr, tree);
        }
        lex_4_25::Token::OpenBracket => {
            tree.root = NonTerminal::IndexAccess;
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::Power               |
                        lex_4_25::Token::Divide              |
                        lex_4_25::Token::Minus               |
                        lex_4_25::Token::Modulus             |
                        lex_4_25::Token::Multiply            |
                        lex_4_25::Token::Plus                |
                        lex_4_25::Token::ShiftLeft           |
                        lex_4_25::Token::ShiftRight          |
                        lex_4_25::Token::BitwiseAnd          |
                        lex_4_25::Token::BitwiseXor          |
                        lex_4_25::Token::BitwiseOr           |
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                NonTerminal::IndexAccess => tree = tree.merge_expressions(right),
                NonTerminal::MemberAccess => tree = tree.merge_expressions(right),
                _ => tree.add_invalid(NonTerminal::Expression)
            }
            match lex_4_25::next_token(input, current_ptr) {
                lex_4_25::Token::CloseBracket => return parse_operation(input, current_ptr, tree),
                _ => tree.add_invalid_token(lex_4_25::Token::CloseBracket)
            }
        }
        lex_4_25::Token::Dot => {
            lex_4_25::next_token(input, current_ptr);
            tree.root = NonTerminal::MemberAccess;
            // TODO(jalextowle): Is there a way to flatten this if it is all MemberAccess?
            tree.add_tree(left);
            let right = lex_4_25::next_token(input, current_ptr);
            match right {
                lex_4_25::Token::Identifier(name) => tree.add_leaf(NonTerminal::Identifier(name)),
                _ => tree.add_invalid(NonTerminal::Identifier(String::from("")))
            }
            return parse_operation(input, current_ptr, tree);
        }
        lex_4_25::Token::OpenParenthesis => {
            tree.root = NonTerminal::FunctionCall;
            tree.add_tree(left);
            tree.add_tree(parse_function_call_arguments(&input, current_ptr));
            let right = parse_expression(&input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::Power               |
                        lex_4_25::Token::Divide              |
                        lex_4_25::Token::Minus               |
                        lex_4_25::Token::Modulus             |
                        lex_4_25::Token::Multiply            |
                        lex_4_25::Token::Plus                |
                        lex_4_25::Token::ShiftLeft           |
                        lex_4_25::Token::ShiftRight          |
                        lex_4_25::Token::BitwiseAnd          |
                        lex_4_25::Token::BitwiseXor          |
                        lex_4_25::Token::BitwiseOr           |
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => ()
            }
        }
        lex_4_25::Token::Power => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::Power               |
                        lex_4_25::Token::Divide              |
                        lex_4_25::Token::Minus               |
                        lex_4_25::Token::Modulus             |
                        lex_4_25::Token::Multiply            |
                        lex_4_25::Token::Plus                |
                        lex_4_25::Token::ShiftLeft           |
                        lex_4_25::Token::ShiftRight          |
                        lex_4_25::Token::BitwiseAnd          |
                        lex_4_25::Token::BitwiseXor          |
                        lex_4_25::Token::BitwiseOr           |
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::Divide   |
        lex_4_25::Token::Multiply |
        lex_4_25::Token::Modulus => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::Divide              |
                        lex_4_25::Token::Minus               |
                        lex_4_25::Token::Modulus             |
                        lex_4_25::Token::Multiply            |
                        lex_4_25::Token::Plus                |
                        lex_4_25::Token::ShiftLeft           |
                        lex_4_25::Token::ShiftRight          |
                        lex_4_25::Token::BitwiseAnd          |
                        lex_4_25::Token::BitwiseXor          |
                        lex_4_25::Token::BitwiseOr           |
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::Plus | lex_4_25::Token::Minus => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::Minus               |
                        lex_4_25::Token::Plus                |
                        lex_4_25::Token::ShiftLeft           |
                        lex_4_25::Token::ShiftRight          |
                        lex_4_25::Token::BitwiseAnd          |
                        lex_4_25::Token::BitwiseXor          |
                        lex_4_25::Token::BitwiseOr           |
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::ShiftLeft | lex_4_25::Token::ShiftRight => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::ShiftLeft           |
                        lex_4_25::Token::ShiftRight          |
                        lex_4_25::Token::BitwiseAnd          |
                        lex_4_25::Token::BitwiseXor          |
                        lex_4_25::Token::BitwiseOr           |
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::BitwiseAnd => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::BitwiseAnd          |
                        lex_4_25::Token::BitwiseXor          |
                        lex_4_25::Token::BitwiseOr           |
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::BitwiseXor => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::BitwiseXor          |
                        lex_4_25::Token::BitwiseOr           |
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::BitwiseOr => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::BitwiseOr           |
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::GreaterThan         |
        lex_4_25::Token::LessThan            |
        lex_4_25::Token::GreaterThanOrEquals |
        lex_4_25::Token::LessThanOrEquals => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::GreaterThan         |
                        lex_4_25::Token::GreaterThanOrEquals |
                        lex_4_25::Token::LessThan            |
                        lex_4_25::Token::LessThanOrEquals    |
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::Equals | lex_4_25::Token::NotEquals => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            println!("{:#?}", right);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::Equals              |
                        lex_4_25::Token::NotEquals           |
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                id @ NonTerminal::Identifier(..) => tree.add_leaf(id),
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::LogicalAnd | lex_4_25::Token::LogicalOr => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::LogicalAnd          |
                        lex_4_25::Token::LogicalOr           |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::Question | lex_4_25::Token::Colon => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::Question            |
                        lex_4_25::Token::Colon               |
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        lex_4_25::Token::Assignment       |
        lex_4_25::Token::OrEquals         |
        lex_4_25::Token::XorEquals        |
        lex_4_25::Token::AndEquals        |
        lex_4_25::Token::ShiftLeftEquals  |
        lex_4_25::Token::ShiftRightEquals |
        lex_4_25::Token::PlusEquals       |
        lex_4_25::Token::MinusEquals      |
        lex_4_25::Token::ModEquals        |
        lex_4_25::Token::MultiplyEquals   |
        lex_4_25::Token::DivideEquals => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(left);
            let right = parse_expression(input, current_ptr);
            match right.root.clone() {
                NonTerminal::Token(token) => {
                    match token {
                        lex_4_25::Token::Assignment          |
                        lex_4_25::Token::OrEquals            |
                        lex_4_25::Token::XorEquals           |
                        lex_4_25::Token::AndEquals           |
                        lex_4_25::Token::ShiftLeftEquals     |
                        lex_4_25::Token::ShiftRightEquals    |
                        lex_4_25::Token::PlusEquals          |
                        lex_4_25::Token::MinusEquals         |
                        lex_4_25::Token::MultiplyEquals      |
                        lex_4_25::Token::DivideEquals        |
                        lex_4_25::Token::ModEquals => tree = tree.merge_expressions(right),
                        lex_4_25::Token::NoMatch => (),
                        _ => tree.add_tree(right)
                    }
                }
                _ => tree.add_invalid(NonTerminal::Expression)
            }
        }
        _ => {
            tree = left;
        }
    }
    tree
}

/**
 * @dev Parse a FunctionCallArguments nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents function call arguments.
 */
fn parse_function_call_arguments(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::FunctionCallArguments.to_leaf();
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::OpenParenthesis => (),
        _ => tree.add_invalid_token(lex_4_25::Token::OpenParenthesis)
    }
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::OpenBrace => {
            lex_4_25::next_token(input, current_ptr);
            tree.add_tree(parse_name_value_list(input, current_ptr));
            match lex_4_25::next_token(input, current_ptr) {
                lex_4_25::Token::CloseBrace => (),
                _ => tree.add_invalid_token(lex_4_25::Token::CloseBrace)
            }
        }
        _ => {
            tree.add_tree(parse_expression_list(input, current_ptr));
        }
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::CloseParenthesis => (),
        _ => tree.add_invalid_token(lex_4_25::Token::CloseParenthesis)
    }
    tree
}

/**
 * @dev Parse an NameValueList nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a name value list.
 */
fn parse_name_value_list(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::NameValueList.to_leaf();
    let mut stop = false;
    while !stop {
        let mut subtree = NonTerminal::Token(lex_4_25::Token::Colon).to_leaf();
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::Identifier(..) => {
                subtree.add_leaf(NonTerminal::Identifier(lex_4_25::Token::next_identifier_name(input, current_ptr)));
            }
            _ => stop = true
        }
        if !stop {
            match lex_4_25::next_token(input, current_ptr) {
                lex_4_25::Token::Colon => (),
                _ => subtree.add_invalid_token(lex_4_25::Token::Colon)
            }
            subtree.add_tree(parse_expression(input, current_ptr));
            match lex_4_25::peek_token(input, current_ptr) {
                lex_4_25::Token::Comma => (),
                _ => stop = true
            }
            tree.add_tree(subtree);
        }
    }
    tree
}

/**
 * @dev Parse an Expression nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an expression.
 */
pub fn parse_expression(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::Expression.to_leaf();
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::New => {
            tree.add_leaf(NonTerminal::NewExpression);
            tree.leaves[0].add_tree(parse_type_name(input, current_ptr));
        }
        lex_4_25::Token::DecimalNumber(..) | lex_4_25::Token::HexNumber(..) => {
            let mut left = NonTerminal::next_token(input, current_ptr).to_leaf();
            let peek = lex_4_25::peek_token(input, current_ptr);
            if peek.is_number_unit() {
                 left.add_token(lex_4_25::next_token(input, current_ptr));
            }
            tree = parse_operation(input, current_ptr, left);
        }
        lex_4_25::Token::Identifier(..) => {
            let left = NonTerminal::next_to_identifier(input, current_ptr).to_leaf();
            println!("left: {:#?}", left);
            tree = parse_operation(input, current_ptr, left.clone());
            println!("tree: {:#?}", tree);
            match tree.root {
                NonTerminal::Invalid(..) => tree = left,
                _ => ()
            }
        }
        lex_4_25::Token::HexLiteral(..)    |
        lex_4_25::Token::StringLiteral(..) |
        lex_4_25::Token::True              |
        lex_4_25::Token::False => {
            let left = NonTerminal::next_token(input, current_ptr).to_leaf();
            tree = parse_operation(input, current_ptr, left);
        }
        lex_4_25::Token::OpenParenthesis => {
            lex_4_25::next_token(input, current_ptr);
            tree = parse_expression(input, current_ptr);
            let mut stop = false;
            while !stop {
                match lex_4_25::next_token(input, current_ptr) {
                    lex_4_25::Token::Comma => tree.add_tree(parse_expression(input, current_ptr)),
                    lex_4_25::Token::CloseParenthesis => stop = true,
                    // TODO(jalextowle): Use list of invalid tokens
                    _ => tree.add_invalid_token(lex_4_25::Token::CloseParenthesis)
                }
                tree = parse_operation(input, current_ptr, tree);
            }
        }
        lex_4_25::Token::Exclamation |
        lex_4_25::Token::Tilda       |
        lex_4_25::Token::Delete      |
        lex_4_25::Token::Increment   |
        lex_4_25::Token::Decrement   |
        lex_4_25::Token::Plus        |
        lex_4_25::Token::Minus => {
            tree.root = NonTerminal::next_token(input, current_ptr);
            tree.add_tree(parse_expression(input, current_ptr));
        }
        elementary => {
            if elementary.is_elementary_type() {
                let left = NonTerminal::next_token(input, current_ptr).to_leaf();
                tree = parse_operation(input, current_ptr, left);
            }
        }
    }
    tree
}

/**
 * @dev Parse an ExpressionList nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an expression list.
 */
fn parse_expression_list(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::ExpressionList.to_leaf();
    let mut stop = false;
    while !stop {
        let returned = parse_expression(&input, current_ptr);
        match tree.root {
            NonTerminal::Invalid(..) => stop = true,
            _ => tree.add_tree(returned)
        }
        if !stop {
            if let lex_4_25::Token::Comma = lex_4_25::peek_token(input, current_ptr) {
                lex_4_25::next_token(input, current_ptr);
            } else {
                stop = true;
            }
        }
    }
    tree
}

/*** Types ***/

/**
 * @dev Parse an TypeName nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a typename.
 */
pub fn parse_type_name(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    return match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Identifier(..) => parse_user_defined_type_name(input, current_ptr),
        lex_4_25::Token::Function => parse_function_type_name(input, current_ptr),
        lex_4_25::Token::Mapping => parse_mapping(input, current_ptr),
        elementary => {
            if elementary.is_elementary_type() {
                lex_4_25::next_token(input, current_ptr);
                // Try to parse an array type and return elementary if the type isn't followed by
                // array brackets.
                parse_array_type_name(input, current_ptr, elementary.to_token().to_leaf())
            } else {
                NonTerminal::TypeName.to_invalid().to_leaf()
            }
        }
    }
}

/**
 * @dev Parse a UserDefinedTypeName nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a user defined type name.
 */
fn parse_user_defined_type_name(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::UserDefinedTypeName.to_leaf();
    let mut stop = false;
    while !stop {
        // Expect an identifier
        tree.add_leaf(NonTerminal::next_to_identifier(input, current_ptr));
        if !stop {
            if let lex_4_25::Token::Dot = lex_4_25::peek_token(&input, current_ptr) {
                lex_4_25::next_token(&input, current_ptr);
            } else {
                stop = true;
            }
        }
    }
    tree
}

/**
 * @dev Parse a Mapping nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a mapping.
 */
fn parse_mapping(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::Mapping.to_leaf();
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Mapping => (),
        _ => tree.add_invalid(NonTerminal::Mapping)
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::OpenParenthesis => (),
        _ => tree.add_invalid_token(lex_4_25::Token::OpenParenthesis)
    }
    let elementary = lex_4_25::next_token(input, current_ptr);
    if elementary.is_elementary_type() {
        tree.add_leaf(elementary.to_token());
    } else {
        tree.add_invalid(NonTerminal::ElementaryTypeName);
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Arrow => (),
        _ => tree.add_invalid_token(lex_4_25::Token::Arrow)
    }
    tree.add_tree(parse_type_name(input, current_ptr));
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::CloseParenthesis => (),
        _ => tree.add_invalid_token(lex_4_25::Token::CloseParenthesis)
    }
    tree
}

/**
 * @dev Parse an ArrayTypeName nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an array type name.
 */
fn parse_array_type_name(input: &Vec<char>, current_ptr: &mut usize, left: ParseTree) -> ParseTree {
    let mut tree = NonTerminal::ArrayTypeName.to_leaf();
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::OpenBracket => {
            lex_4_25::next_token(input, current_ptr);
        }
        _ => return left
    }
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::CloseBracket => {
            lex_4_25::next_token(input, current_ptr);
            return left;
        }
        _ => {
            tree.add_tree(left);
            tree.add_tree(parse_expression(input, current_ptr));
        }
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::CloseBracket => (),
        _ => tree.add_invalid_token(lex_4_25::Token::CloseBracket)
    }
    tree
}

/**
 * @dev Parse an FunctionType nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a function type.
 */
fn parse_function_type_name(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::FunctionTypeName.to_leaf();
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::Function => (),
        _ => tree.root = tree.root.to_invalid()
    }
    tree.add_tree(parse_function_type_parameter_list(input, current_ptr));
    let mut stop = false;
    while !stop {
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::External |
            lex_4_25::Token::Internal |
            lex_4_25::Token::Public   |
            lex_4_25::Token::Private  => tree.add_token(lex_4_25::next_token(input, current_ptr)),
            lex_4_25::Token::Pure     |
            lex_4_25::Token::Constant |
            lex_4_25::Token::View     |
            lex_4_25::Token::Payable => tree.add_leaf(NonTerminal::StateMutability(lex_4_25::next_token(input, current_ptr))),
            _ => stop = true
        }
    }
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Returns => {
            tree.add_token(lex_4_25::next_token(input, current_ptr));
            let last = tree.leaves.len() - 1;
            tree.leaves[last].add_tree(parse_function_type_parameter_list(input, current_ptr));
        }
        _ => (),
    }
    tree
}

/**
 * @dev Parse an FunctionTypeParameterList nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents an function type parameter list.
 */
fn parse_function_type_parameter_list(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::FunctionTypeParameterList.to_leaf();
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::OpenParenthesis => (),
        _ => tree.root = tree.root.to_invalid()
    }
    let mut stop = false;
    while !stop {
        match lex_4_25::peek_token(input, current_ptr) {
            lex_4_25::Token::CloseParenthesis => stop = true,
            _ => tree.add_tree(parse_function_type_parameter(input, current_ptr))
        }
        if !stop {
            match lex_4_25::peek_token(input, current_ptr) {
                lex_4_25::Token::Comma => {
                    lex_4_25::next_token(input, current_ptr);
                }
                _ => stop = true
            }
        }
    }
    match lex_4_25::next_token(input, current_ptr) {
        lex_4_25::Token::CloseParenthesis => (),
        _ => tree.add_invalid_token(lex_4_25::Token::CloseParenthesis)
    }
    tree
}

/**
 * @dev Parse an FunctionTypeParameter nonterminal. If there are issues parsing the nonterminal, error
 *      information will be recorded in the ParseTree to aid with debugging.
 * @param input The characters from the input that should be read
 * @param current_ptr The position in the input characters where the lexer should start reading.
 * @return A ParseTree that represents a function type parameter.
 */
fn parse_function_type_parameter(input: &Vec<char>, current_ptr: &mut usize) -> ParseTree {
    let mut tree = NonTerminal::FunctionTypeParameter.to_leaf();
    tree.add_tree(parse_type_name(input, current_ptr));
    match lex_4_25::peek_token(input, current_ptr) {
        lex_4_25::Token::Memory |
        lex_4_25::Token::Storage => {
            tree.add_leaf(NonTerminal::StorageLocation(lex_4_25::next_token(input, current_ptr)));
        }
        _ => ()
    }
    tree
}
