use super::lex_4_25;

#[derive(Debug)]
pub struct ParseTree {
    children: Vec<ParseNode>
}

#[derive(Clone, Debug)]
pub struct ParseNode {
    node: lex_4_25::Token,
    children: Vec<Box<ParseNode>>
}

impl ParseNode {
    fn add_child(&mut self, token: lex_4_25::Token) { self.children.push(Box::new(ParseNode{ node: token, children: vec![] })); }
}

pub fn parse(input: String) -> ParseTree {
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

fn parse_pragma(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) { 
    let mut next = lex_4_25::next_token(chars, cur);
    match next {
        lex_4_25::Token::Identifier(name) => {
            if name != "solidity" {
                panic!("Invalid source file: Not a solidity file")
            }
            node.add_child(lex_4_25::Token::Identifier(name));
        }
        _ => panic!("Invalid pragma declaration")
    }
    next = lex_4_25::next_token(&chars, cur);
    match next {
        lex_4_25::Token::Version(version) => {
            if version != "^0.4.25" && version != "0.4.25" {
                panic!("Invalid source file: version other than 0.4.25 specfied")
            }
            node.add_child(lex_4_25::Token::Version(version));
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
        id@lex_4_25::Token::Identifier(..) => node.add_child(id),
        _ => panic!("Invalid contract definition")
    }
    next = lex_4_25::next_token(&chars, cur);
    match next {
        lex_4_25::Token::Is => { 
            node.add_child(lex_4_25::Token::Is);
            // TODO: parse_inheritance_specifier(chars, cur, node, tree);
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
            node.add_child(lex_4_25::Token::Using);
            // TODO: parse_using_for_declaration(chars, cur, node, tree);
        }
        lex_4_25::Token::Struct => {
            node.add_child(lex_4_25::Token::Struct);
            // TODO: parse_struct_definition(chars, cur, node, tree);
        }
        lex_4_25::Token::Modifier => {
            node.add_child(lex_4_25::Token::Modifier);
            // TODO: parse_modifier_definition(chars, cur, node, tree);
        }
        lex_4_25::Token::Function => {
            node.add_child(lex_4_25::Token::Function);
            // TODO: parse_function_definition(chars, cur, node, tree);
        }
        lex_4_25::Token::Event => {
            node.add_child(lex_4_25::Token::Event);
            // TODO: parse_event_definition(chars, cur, node, tree);
        }
        lex_4_25::Token::Enum => {
            node.add_child(lex_4_25::Token::Enum);
            // TODO: parse_enum_definition(chars, cur, node, tree);
        }
        matched => ()// TODO: parse_state_variable_declaration(chars, cur, node, tree, matched)
    }
}

/*** Expression ***/

// TODO: This needs to be tested
fn parse_expression(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) {

}

fn parse_simple_expression(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) {
    let next = lex_4_25::next_token(&chars, cur);
    match next {
        // Preceded Expression TODO: Add expression parsing after
        matched @ lex_4_25::Token::Delete |
        matched @ lex_4_25::Token::Exclamation |
        matched @ lex_4_25::Token::Minus |
        matched @ lex_4_25::Token::Plus |
        matched @ lex_4_25::Token::Tilda => {
            node.add_child(matched);
            parse_expression(chars, cur, node, tree);
        }
        // TODO: Add increment and decrement
        // Primary Expression
        boolean @ lex_4_25::Token::True | boolean @ lex_4_25::Token::False => node.add_child(boolean),
        lex_4_25::Token::False => node.add_child(lex_4_25::Token::False),
        num @ lex_4_25::Token::DecimalNumber(..) |
        num @ lex_4_25::Token::HexNumber(..) => {
            node.add_child(num);
            let peek = lex_4_25::peek_token(&chars, cur);
            if peek.is_number_unit() {
               node.add_child(lex_4_25::next_token(&chars, cur)); 
            }
        }
        num @ lex_4_25::Token::HexLiteral(..) => node.add_child(num),
        string @ lex_4_25::Token::StringLiteral(..) => node.add_child(string),
        open @ lex_4_25::Token::OpenParenthesis |
        open @ lex_4_25::Token::OpenBracket => {
            node.add_child(open);
            parse_tuple_expression(chars, cur, node, tree);
        }
        id@lex_4_25::Token::Identifier(..) => node.add_child(id),
        result => {
            if result.is_elementary_type() {
                node.add_child(result);
            } else {
                panic!()
            }
        }
    }
}

fn parse_tuple_expression(chars: &Vec<char>, cur: &mut usize, node: &mut ParseNode, tree: &mut ParseTree) {
}

/*** Types ***/
