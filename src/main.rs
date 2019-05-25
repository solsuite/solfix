use std::env;
use std::fs::File;
use std::io::{BufReader, BufRead};
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
    Import(u8),
    Contract,
    Library,
    Interface
}

impl Level {
    fn is_contract(&self) -> bool {
        match self {
            Level::Contract => true,
            _ => false
        }
    }

    fn is_import(&self) -> bool {
        match self {
            Level::Import(..) => true,
            _ => false
        }
    }

    fn is_interface(&self) -> bool {
        match self {
            Level::Interface => true,
            _ => false
        }
    }

    fn is_library(&self) -> bool {
        match self {
            Level::Library => true,
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

fn parse(input: BufReader<File>) -> ParseTree {
    let mut node;
    let mut current_node = &mut ParseNode{ node: lex_4_25::Token::NoMatch, children: vec![] }; 
    let mut tree = ParseTree{ children: vec![] };
    let mut level = Level::Top;
    for line in input.lines() {
        let mut cur = 0;
        let chars = line.expect("Unable to read line").chars().collect::<Vec<char>>();
        while cur < chars.len() {
            let next = lex_4_25::next_token(&chars, &mut cur);
            cur += 1;
            if level.is_top() {
                match next {
                    lex_4_25::Token::Pragma => {
                        level = Level::Pragma(0);
                        current_node.node = lex_4_25::Token::Pragma;
                    }
                    lex_4_25::Token::Import => {
                        level = Level::Import(0);
                    }
                    lex_4_25::Token::Contract => {
                        level = Level::Contract;
                    }
                    lex_4_25::Token::Library => {
                        level = Level::Contract;
                    }
                    lex_4_25::Token::Interface => {
                        level = Level::Contract;
                    }
                    _ => panic!("Invalid top level expression")
                }
            } else if level.is_pragma() {
                if let Level::Pragma(term) = level {
                    if term == 0 {
                        match next {
                            lex_4_25::Token::Identifier(name) => { 
                                if name != "solidity" {
                                    panic!("Invalid pragma name");
                                }
                                level = Level::Pragma(1);
                                current_node.children.push(
                                    Box::new(
                                        ParseNode{ 
                                            node: lex_4_25::Token::Identifier("solidity".to_string()), 
                                            children: vec![] 
                                        }
                                    )
                                );
                            }
                            _ => panic!("Invalid pragma declaration")
                        }
                    } else if term == 1 {
                        match next {
                            lex_4_25::Token::Version(version) => {
                                if version != "^0.4.25" && version != "0.4.25" {
                                    panic!("{} is an invalid version number for this transpiler", version)
                                }
                                level = Level::Pragma(2);
                                current_node.children.push(
                                    Box::new(
                                        ParseNode{
                                            node: lex_4_25::Token::Version(version),
                                            children: vec![]
                                        }
                                    )
                                );
                            }
                            _ => panic!("Invalid pragma declaration")
                        }
                    } else if term == 2 {
                        match next {
                            lex_4_25::Token::Semicolon => {
                                level = Level::Top;
                                tree.children.push((*current_node).clone());
                                node = ParseNode{ node: lex_4_25::Token::NoMatch, children: vec![] };
                                current_node = &mut node;
                            }
                            _ => panic!("Invalid pragma declaration")
                        }
                    }
                }
            } else if level.is_import() {

            } else if level.is_contract() {

            }
        }
    }
    tree
}

/* Main */

fn main() {
    let name = env::args().nth(1).expect("Usage: solidity-fix FILE_NAME");
    let input = File::open(name).expect("Unable to open input file");
    println!("{:?}", parse(BufReader::new(input)));
}
