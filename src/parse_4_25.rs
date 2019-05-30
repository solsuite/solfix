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
    fn add_child(&mut self, token: lex_4_25::Token) { 
        self.children.push(Box::new(ParseNode{ node: token, children: vec![] })); 
    }

    fn remove_left_child(&mut self) -> Box<ParseNode> { 
        let mut children = self.children.clone();
        self.children = children.split_off(1); 
        children.pop().unwrap()
    }

    fn merge_expressions(mut self, mut other: ParseNode) -> ParseNode {
        self.children.push(other.remove_left_child());
        other.children.push(Box::new(self));
        other.children.swap(0, 1);
        other
    }
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

fn parse_operation(chars: &Vec<char>, cur: &mut usize, left: lex_4_25::Token) -> ParseNode {
    let mut result = ParseNode{ node: lex_4_25::Token::NoMatch, children: vec![] };
    let mut peek = lex_4_25::peek_token(&chars, cur);
    match peek {
        lex_4_25::Token::Power => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::Power;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::Divide => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::Divide; 
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::Multiply => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::Multiply; 
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::Modulus => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::Modulus; 
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::Plus => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::Plus; 
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::Minus => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::Minus; 
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::ShiftLeft => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::ShiftLeft;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::ShiftRight => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::ShiftRight;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::BitwiseAnd => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::BitwiseAnd;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::BitwiseXor => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::BitwiseXor;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::BitwiseOr => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::BitwiseOr;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
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
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::GreaterThan => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::GreaterThan;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::GreaterThan         |
                lex_4_25::Token::GreaterThanOrEquals |
                lex_4_25::Token::LessThan            |
                lex_4_25::Token::LessThanOrEquals    |
                lex_4_25::Token::Equals              |
                lex_4_25::Token::NotEquals           |
                lex_4_25::Token::LogicalAnd          |
                lex_4_25::Token::LogicalOr           |
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::LessThan => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::LessThan;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::GreaterThan         |
                lex_4_25::Token::GreaterThanOrEquals |
                lex_4_25::Token::LessThan            |
                lex_4_25::Token::LessThanOrEquals    |
                lex_4_25::Token::Equals              |
                lex_4_25::Token::NotEquals           |
                lex_4_25::Token::LogicalAnd          |
                lex_4_25::Token::LogicalOr           |
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::GreaterThanOrEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::GreaterThanOrEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::GreaterThan         |
                lex_4_25::Token::GreaterThanOrEquals |
                lex_4_25::Token::LessThan            |
                lex_4_25::Token::LessThanOrEquals    |
                lex_4_25::Token::Equals              |
                lex_4_25::Token::NotEquals           |
                lex_4_25::Token::LogicalAnd          |
                lex_4_25::Token::LogicalOr           |
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::LessThanOrEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::LessThanOrEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::GreaterThan         |
                lex_4_25::Token::GreaterThanOrEquals |
                lex_4_25::Token::LessThan            |
                lex_4_25::Token::LessThanOrEquals    |
                lex_4_25::Token::Equals              |
                lex_4_25::Token::NotEquals           |
                lex_4_25::Token::LogicalAnd          |
                lex_4_25::Token::LogicalOr           |
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::Equals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::Equals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Equals              |
                lex_4_25::Token::NotEquals           |
                lex_4_25::Token::LogicalAnd          |
                lex_4_25::Token::LogicalOr           |
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::NotEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::NotEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Equals              |
                lex_4_25::Token::NotEquals           |
                lex_4_25::Token::LogicalAnd          |
                lex_4_25::Token::LogicalOr           |
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::LogicalAnd => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::LogicalAnd;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::LogicalAnd          |
                lex_4_25::Token::LogicalOr           | 
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::LogicalOr => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::LogicalOr;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::LogicalOr           | 
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::Set => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::Set;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::OrEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::OrEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::XorEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::XorEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::AndEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::AndEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::ShiftLeftEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::ShiftLeftEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::ShiftRightEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::ShiftRightEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::PlusEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::PlusEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::MinusEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::MinusEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::ModEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::ModEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::MultiplyEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::MultiplyEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        lex_4_25::Token::DivideEquals => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::DivideEquals;
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            let mut right = parse_expression(&chars, cur);
            match right.node {
                lex_4_25::Token::Set                 |
                lex_4_25::Token::OrEquals            |
                lex_4_25::Token::XorEquals           |
                lex_4_25::Token::AndEquals           |
                lex_4_25::Token::ShiftLeftEquals     |
                lex_4_25::Token::ShiftRightEquals    |
                lex_4_25::Token::PlusEquals          |
                lex_4_25::Token::MinusEquals         |
                lex_4_25::Token::MultiplyEquals      |
                lex_4_25::Token::DivideEquals        |
                lex_4_25::Token::ModEquals => result = result.merge_expressions(right), 
                _ => result.children.push(Box::new(right))
            }
        }
        // TODO
        lex_4_25::Token::Dot => {
            lex_4_25::next_token(&chars, cur);
            result.node = lex_4_25::Token::Dot; 
            result.children.push(Box::new(ParseNode{ node: left, children: vec![] }));
            result.children.push(Box::new(parse_expression(&chars, cur)));
        }
        _ => {
            result.node = left;
        }
    }
    result
}

fn parse_expression(chars: &Vec<char>, cur: &mut usize) -> ParseNode {
    let mut result = ParseNode{ node: lex_4_25::Token::NoMatch, children: vec![] };
    let mut peek = lex_4_25::peek_token(&chars, cur);
    match peek {
        lex_4_25::Token::Identifier(..) => {
            let left = lex_4_25::next_token(&chars, cur);
            result = parse_operation(&chars, cur, left);
        }
        lex_4_25::Token::New => {
            result.node = lex_4_25::next_token(&chars, cur);
            result.children.push(Box::new(parse_type_name(&chars, cur)));
        }
        lex_4_25::Token::DecimalNumber(..) => {
            let left = lex_4_25::next_token(&chars, cur);
            result = parse_operation(&chars, cur, left);
        }
        lex_4_25::Token::HexNumber(..) => {
            let left = lex_4_25::next_token(&chars, cur);
            result = parse_operation(&chars, cur, left);
        }
        lex_4_25::Token::Exclamation | 
        lex_4_25::Token::Tilda       | 
        lex_4_25::Token::Delete      | 
        lex_4_25::Token::Increment   | 
        lex_4_25::Token::Decrement   |
        lex_4_25::Token::Plus        |
        lex_4_25::Token::Minus => {
            result.node = lex_4_25::next_token(&chars, cur);
            result.children.push(Box::new(parse_expression(&chars, cur)));
        }
        _ => ()
    }
    result
}

/*** Types ***/

fn parse_type_name(chars: &Vec<char>, cur: &mut usize) -> ParseNode {
    ParseNode{ node: lex_4_25::Token::NoMatch, children: vec![] }
}

/*** Tests ***/

#[cfg(test)]
mod tests {
    use super::*;

    /* Expressions */

    #[test]
    fn addition_parsing_test() {
        let addition = String::from("1500 + 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&addition, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        let left = String::from("1500");
        let right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::DecimalNumber(left) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(left), actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::HexNumber(right) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(right), actual)
        }
    }

    #[test]
    fn multiplication_parsing_test() {
        let multiplication = String::from("1500 * 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&multiplication, cur);
        match node.node {
            lex_4_25::Token::Multiply => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 0);
        assert_eq!(node.children[1].children.len(), 0);
        let left = String::from("1500");
        let right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::DecimalNumber(left) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(left), actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::HexNumber(right) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(right), actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test1() {
        let multiplication = String::from("(800 + 1500) * 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&multiplication, cur);
        println!("{:?}", node);
        match node.node {
            lex_4_25::Token::Multiply => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 2);
        assert_eq!(node.children[1].children.len(), 0);
        let inner_left = String::from("800");
        let inner_right = String::from("1500");
        let right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::Plus => {
                match &node.children[0].children[0].node {
                    lex_4_25::Token::DecimalNumber(inner_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(inner_left), actual)
                }
                match &node.children[0].children[1].node {
                    lex_4_25::Token::DecimalNumber(inner_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(inner_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::HexNumber(right) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(right), actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test2() {
        let multiplication = String::from("800 + 1500 * 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&multiplication, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 0);
        assert_eq!(node.children[1].children.len(), 2);
        let left = String::from("800");
        let inner_left = String::from("1500");
        let inner_right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::DecimalNumber(inner_left) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(left), actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::Multiply => {
                match &node.children[1].children[0].node {
                    lex_4_25::Token::DecimalNumber(inner_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(inner_left), actual)
                }
                match &node.children[1].children[1].node {
                    lex_4_25::Token::HexNumber(inner_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(inner_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test3() {
        let arithmetic = String::from("800 * 1500 + 0x000").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&arithmetic, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 2);
        assert_eq!(node.children[1].children.len(), 0);
        let inner_left = String::from("800");
        let inner_right = String::from("1500");
        let right = String::from("0x000");
        match &node.children[0].node {
            lex_4_25::Token::Multiply => {
                match &node.children[0].children[0].node {
                    lex_4_25::Token::DecimalNumber(inner_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(inner_left), actual)
                }
                match &node.children[0].children[1].node {
                    lex_4_25::Token::DecimalNumber(inner_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(inner_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::HexNumber(right) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(right), actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test4() {
        let arithmetic = String::from("800 * 1500 + 0x000 * 0x800").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&arithmetic, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 2);
        assert_eq!(node.children[1].children.len(), 2);
        let left_left = String::from("800");
        let left_right = String::from("1500");
        let right_left = String::from("0x000");
        let right_right = String::from("0x800");
        match &node.children[0].node {
            lex_4_25::Token::Multiply => {
                match &node.children[0].children[0].node {
                    lex_4_25::Token::DecimalNumber(left_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(left_left), actual)
                }
                match &node.children[0].children[1].node {
                    lex_4_25::Token::DecimalNumber(left_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(left_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::Multiply => {
                match &node.children[1].children[0].node {
                    lex_4_25::Token::HexNumber(right_left) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(right_left), actual)
                }
                match &node.children[1].children[1].node {
                    lex_4_25::Token::HexNumber(right_right) => (),
                    actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(right_right), actual)
                }
            }
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
    }

    #[test]
    fn arithmetic_parsing_test5() {
        let arithmetic = String::from("1 - 800 * 1500 + 0x000 * 0x800 / 5 ** 1800").chars().collect::<Vec<char>>();
        let cur = &mut 0;
        let node = parse_expression(&arithmetic, cur);
        match node.node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(node.children.len(), 2);
        assert_eq!(node.children[0].children.len(), 2);
        assert_eq!(node.children[1].children.len(), 2);
        match &node.children[0].node {
            lex_4_25::Token::Minus => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].node {
            lex_4_25::Token::Divide => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Divide, actual)
        }
        let one = String::from("1");
        match &node.children[0].children[0].node {
            lex_4_25::Token::DecimalNumber(one) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(one), actual)
        }
        match &node.children[0].children[1].node {
            lex_4_25::Token::Multiply => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].children[0].node {
            lex_4_25::Token::Multiply => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Multiply, actual)
        }
        match &node.children[1].children[1].node {
            lex_4_25::Token::Power => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Power, actual)
        }
        assert_eq!(node.children[0].children[0].children.len(), 0);
        assert_eq!(node.children[0].children[1].children.len(), 2);
        assert_eq!(node.children[1].children[0].children.len(), 2);
        assert_eq!(node.children[1].children[1].children.len(), 2);
        let eight_hundred = String::from("800");
        match &node.children[0].children[1].children[0].node {
            lex_4_25::Token::DecimalNumber(eight_hundred) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(eight_hundred), actual)
        }
        let one_thousand_five_hundred = String::from("1500");
        match &node.children[0].children[1].children[1].node {
            lex_4_25::Token::DecimalNumber(one_thousand_five_hundred) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::DecimalNumber(one_thousand_five_hundred), actual)
        }
        let zero = String::from("0x000");
        match &node.children[1].children[0].children[0].node {
            lex_4_25::Token::HexNumber(zero) => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::HexNumber(zero), actual)
        }
        let two_thousand_forty_eight = String::from("0x800");
        match &node.children[1].children[0].children[1].node {
            lex_4_25::Token::HexNumber(two_thousand_forty_eight) => (),
            actual => panic!("expected: {:?} | actual: {:?}", lex_4_25::Token::HexNumber(two_thousand_forty_eight), actual)
        }
        let five = String::from("5");
        match &node.children[1].children[1].children[0].node {
            lex_4_25::Token::DecimalNumber(five) => (),
            actual => panic!("expected: {:?} | actual: {:?}", lex_4_25::Token::DecimalNumber(five), actual)
        }
        let one_thousand_eight_hundred = String::from("1800");
        match &node.children[1].children[1].children[1].node {
            lex_4_25::Token::DecimalNumber(one_thousand_eight_hundred) => (),
            actual => panic!("expected: {:?} | actual: {:?}", lex_4_25::Token::DecimalNumber(one_thousand_eight_hundred), actual)
        }
    }
}
