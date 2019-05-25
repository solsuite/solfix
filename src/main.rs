use std::env;
use std::fs;
mod lex_4_25;

/* Parser */

#[derive(Debug)]
struct ParseTree {
    children: Vec<ParseNode>
}

#[derive(Clone, Debug)]
struct ParseNode {
    node: lex_4_25::Token,
    children: Vec<Box<ParseNode>>
}

#[derive(PartialEq)]
enum Level {
    Top,
    Pragma(u8),
    Import(u8, u8),
    Contract(u8),
    ContractPart,
    InheritanceSpecifier(u8),
    UserDefinedTypeName,
    Expression,
}

impl Level {
    fn is_contract(&self) -> bool {
        match self {
            Level::Contract(..) => true,
            _ => false
        }
    }

    fn is_contract_part(&self) -> bool {
        match self {
            Level::ContractPart => true,
            _ => false
        }
    }

    fn is_import(&self) -> bool {
        match self {
            Level::Import(..) => true,
            _ => false
        }
    }

    fn is_inheritance_specifier(&self) -> bool {
        match self {
            Level::InheritanceSpecifier(..) => true,
            _ => false
        }
    }

    fn is_pragma(&self) -> bool {
        match self {
            Level::Pragma(..) => true,
            _ => false
        }
    }

    fn is_top(&self) -> bool {
        match self {
            Level::Top => true,
            _ => false
        }
    }
}

fn parse_pragma(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { 
    let mut next = lex_4_25::next_token(chars, cur);
    match next {
        lex_4_25::Token::Identifier(name) => {
            if name != "solidity" {
                panic!("Invalid source file: Not a solidity file")
            }
            node.children.push(Box::new(ParseNode{ node: lex_4_25::Token::Identifier(name), children: vec![] }));
        }
        _ => panic!("Invalid pragma declaration")
    }
    next = lex_4_25::next_token(&chars, cur);
    match next {
        lex_4_25::Token::Version(version) => {
            if version != "^0.4.25" && version != "0.4.25" {
                panic!("Invalid source file: version other than 0.4.25 specfied")
            }
            node.children.push(Box::new(ParseNode{ node: lex_4_25::Token::Version(version), children: vec![] }));
        }
        _ => panic!("Invalid pragma declaration")
    }
    next = lex_4_25::next_token(&chars, cur);
    match next {
        lex_4_25::Token::Semicolon => {
            tree.children.push((*node).clone());
        }
        _ => panic!("Invalid pragma declaration")
    }
} 

fn parse_import(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_contract(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_contract_part(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_inheritance_specifier(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_user_defined_type_name(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_expression(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse(input: String) -> ParseTree {
    let current_node = &mut ParseNode{ node: lex_4_25::Token::NoMatch, children: vec![] }; 
    let mut tree = ParseTree{ children: vec![] };
    let mut cur = 0;
    let input_chars = input.chars().collect::<Vec<char>>(); 
    while cur < input_chars.len() {
        let next = lex_4_25::next_token(&input_chars, &mut cur);
        match next {
            lex_4_25::Token::Pragma => {
                current_node.node = lex_4_25::Token::Pragma;
                parse_pragma(&input_chars, &mut cur, current_node, &mut tree);
            }
            lex_4_25::Token::Import => {
                // TODO
            }
            lex_4_25::Token::Contract => {
                current_node.node = lex_4_25::Token::Contract;
                // TODO
            }
            lex_4_25::Token::Library => {
                current_node.node = lex_4_25::Token::Library;
                // TODO
            }
            lex_4_25::Token::Interface => {
                current_node.node = lex_4_25::Token::Interface;
                // TODO
            }
            _ => {
                // TODO: Add the below when everything is implemented
                /* panic!("Invalid top level expression: {:?}", none) */
            }
        }
    }
    tree
}

/* Main */

fn main() {
    let name = env::args().nth(1).expect("Usage: solidity-fix FILE_NAME");
    let input = fs::read_to_string(name).expect("Unable to open input file");
    println!("{:?}", parse(input));
}
