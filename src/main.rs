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
        lex_4_25::Token::Semicolon => tree.children.push((*node).clone()), 
        _ => panic!("Invalid pragma declaration")
    }
} 

fn parse_import(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_contract(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { 
    let mut next = lex_4_25::next_token(&chars, cur);
    match next {
        id@lex_4_25::Token::Identifier(..) => node.children.push(Box::new(ParseNode{ node: id, children: vec![]})),
        _ => panic!("Invalid contract definition")
    }
    next = lex_4_25::next_token(&chars, cur);
    match next {
        lex_4_25::Token::Is => { 
            node.children.push(Box::new(ParseNode{ node: lex_4_25::Token::Is, children: vec![] }));
            parse_inheritance_specifier(chars, cur, node, tree);
        }
        lex_4_25::Token::OpenBrace => (),
        _ => panic!("Invalid contract definition")
    }
    parse_contract_part(chars, cur, node, tree);
    tree.children.push((*node).clone());
}

/*** Contract Part ***/

fn parse_contract_part(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) {
    let mut next = lex_4_25::next_token(&chars, cur);
    match next {
        lex_4_25::Token::Using => {
            node.children.push(Box::new(ParseNode{ node: lex_4_25::Token::Using, children: vec![] }));
            parse_using_for_declaration(chars, cur, node, tree);
        }
        lex_4_25::Token::Struct => {
            node.children.push(Box::new(ParseNode{ node: lex_4_25::Token::Struct, children: vec![] }));
            parse_struct_definition(chars, cur, node, tree);
        }
        lex_4_25::Token::Modifier => {
            node.children.push(Box::new(ParseNode{ node: lex_4_25::Token::Modifier, children: vec![] }));
            parse_modifier_definition(chars, cur, node, tree);
        }
        lex_4_25::Token::Function => {
            node.children.push(Box::new(ParseNode{ node: lex_4_25::Token::Function, children: vec![] }));
            parse_function_definition(chars, cur, node, tree);
        }
        lex_4_25::Token::Event => {
            node.children.push(Box::new(ParseNode{ node: lex_4_25::Token::Event, children: vec![] }));
            parse_event_definition(chars, cur, node, tree);
        }
        lex_4_25::Token::Enum => {
            node.children.push(Box::new(ParseNode{ node: lex_4_25::Token::Enum, children: vec![] }));
            parse_enum_definition(chars, cur, node, tree);
        }
        matched => parse_state_variable_declaration(matched, chars, cur, node, tree)
    }
}

fn parse_state_variable_declaration(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree, first: lex_4_25::Token) { 
    parse_type_name(chars, cur, node, tree, first);
    let mut next = lex_4_25::next_token(&chars, cur);
}

fn parse_type_name(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree, first: lex_4_25::Token) { }

fn parse_using_for_declaration(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_struct_definition(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_modifier_definition(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_function_definition(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_event_definition(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { }

fn parse_enum_definition(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { 
    
}

/*** Inheritance Specifier ***/ 

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
                parse_contract(&input_chars, &mut cur, current_node, &mut tree);
            }
            lex_4_25::Token::Library => {
                current_node.node = lex_4_25::Token::Library;
                parse_contract(&input_chars, &mut cur, current_node, &mut tree);
            }
            lex_4_25::Token::Interface => {
                current_node.node = lex_4_25::Token::Interface;
                parse_contract(&input_chars, &mut cur, current_node, &mut tree);
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
