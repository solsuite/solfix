use super::lex_4_25;
use super::parse_4_25;

/*** Top Level ***/

pub fn generate(tree: parse_4_25::ParseTree) -> String {
    let mut result = String::new();
    for child in tree.children {
        match child.node {
            lex_4_25::Token::Pragma => result.push_str(&generate_pragma(child)),
            lex_4_25::Token::Import => result.push_str(&generate_import(child)),
            lex_4_25::Token::Library   |
            lex_4_25::Token::Contract  |
            lex_4_25::Token::Interface => result.push_str(&generate_contract(child)),
            _ => panic!("Invalid top level tree")
        }
    }
    result    
}

/*** Pragma ***/

fn generate_pragma(pragma: parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    if let lex_4_25::Token::Pragma = pragma.node {
        result.push_str("pragma ");
    }
    if pragma.children.len() >= 2 {
        if let lex_4_25::Token::Identifier(name) = &pragma.children[0].node {
            result.push_str(&name);
            result.push(' ');
        }
        if pragma.children.len() == 2 {
            if let lex_4_25::Token::Version(version) = &pragma.children[1].node {
                result.push_str(&version);
                result.push_str(";");
            }
        } else if pragma.children.len() == 3 {
            if let lex_4_25::Token::BitwiseXor = &pragma.children[1].node {
                result.push('^');
            }
            if let lex_4_25::Token::Version(version) = &pragma.children[2].node {
                result.push_str(&version);
                result.push_str(";");
            }
        }
    }
    result
}

/*** Import ***/

fn generate_import(import: parse_4_25::ParseNode) -> String { String::new() }

/*** Contract ***/

fn generate_contract(contract: parse_4_25::ParseNode) -> String { 
    let mut result = String::new();
    match contract.node {
        lex_4_25::Token::Contract => result.push_str("\n\ncontract "),
        lex_4_25::Token::Library => result.push_str("\n\nlibrary "),
        lex_4_25::Token::Interface => result.push_str("\n\ninterface "),
        _ => panic!("Invalid contract definition")
    }
    if contract.children.len() == 2 {
        match &contract.children[0].node {
            lex_4_25::Token::Identifier(name) => {
                result.push_str(&name);
                result.push(' ');
            }
            _ => panic!("Invalid contract definition")
        }
        match &contract.children[1].node {
            lex_4_25::Token::OpenBrace => {
                result.push_str(&generate_contract_part(&contract.children[1]));
            }
            _ => panic!("Invalid contract definition")
        }
    } else if contract.children.len() == 3 {
        match &contract.children[0].node {
            lex_4_25::Token::Identifier(name) => {
                result.push_str(&name);
                result.push(' ');
            }
            _ => panic!("Invalid contract definition")
        }
        match &contract.children[1].node {
            lex_4_25::Token::Is => {
                result.push_str(&generate_inheritance_specifier(&contract.children[1]));
            }
            _ => panic!("Invalid contract definition")
        }
        match &contract.children[2].node {
            lex_4_25::Token::OpenBrace => {
                result.push_str(&generate_contract_part(&contract.children[2]));
            }
            _ => panic!("Invalid contract definition")
        }
    }
    result
}

/*** Inheritance Specifier ***/

fn generate_inheritance_specifier(block: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match block.node {
        lex_4_25::Token::Is => result.push_str("is\n"),
        _ => panic!("Invalid inheritance specifier")
    }
    match &block.children[0].node {
        lex_4_25::Token::OpenParenthesis => (),
        _ => panic!("Invalid inheritance specifier")
    }
    for i in 0..=block.children[0].children.len() - 1 {
        // FIXME Replace with result.indent()
        result.push_str("    ");
        result.push_str(&generate_user_defined_type_name(&block.children[0].children[i]));
        if i != block.children[0].children.len() - 1 {
            result.push_str(",\n");
        } else {
            result.push('\n');
        }
    }
    result
}

/*** Contract Part ***/

fn generate_contract_part(block: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match block.node {
        lex_4_25::Token::OpenBrace => result.push('{'),
        _ => panic!("Invalid contract part")
    }
    for child in &block.children {
        match child.node {
            lex_4_25::Token::Enum => result.push_str(&generate_enum(&child)),
            lex_4_25::Token::Event => result.push_str(&generate_event(&child)),
            lex_4_25::Token::Function => result.push_str(&generate_function(&child)),
            lex_4_25::Token::Modifier => result.push_str(&generate_modifier(&child)),
            lex_4_25::Token::Using => result.push_str(&generate_using_for(&child)),
            lex_4_25::Token::Struct => result.push_str(&generate_struct(&child)),
            _ => panic!("Invalid contract part")
        }
    }
    result.push('}');
    result
}

/*** Sub-contract Structures ***/

fn generate_enum(node: &parse_4_25::ParseNode) -> String { 
    let mut result = String::new();
    match node.node {
        lex_4_25::Token::Enum => result.push_str("enum"),
        _ => panic!("Invalid enum definition")
    }
    result.push(' ');
    match &node.children[0].node {
        lex_4_25::Token::Identifier(name) => result.push_str(&name),
        _ => panic!("Invalid enum definition")
    }
    result.push(' ');
    match &node.children[1].node {
        lex_4_25::Token::OpenBrace => result.push_str("{"),
        _ => panic!("Invalid enum definition")
    }
    if node.children[1].children.len() > 0 {
        for i in 0..=node.children[1].children.len() - 1 {
            result.push('\n');
            match &node.children[1].children[i].node {
                lex_4_25::Token::Identifier(name) => {
                    result.push_str("    ");
                    result.push_str(&name);
                }
                _ => panic!("Invalid enum definition")
            }
            if i != node.children[1].children.len() - 1 {
                result.push(',');
            } else {
                result.push('\n');
            }
        }
    }
    result.push_str("}");
    result
}

fn generate_event(node: &parse_4_25::ParseNode) -> String { String::new() }

fn generate_function(node: &parse_4_25::ParseNode) -> String { String::new() }

fn generate_modifier(node: &parse_4_25::ParseNode) -> String { String::new() }

fn generate_using_for(node: &parse_4_25::ParseNode) -> String { String::new() }

fn generate_struct(node: &parse_4_25::ParseNode) -> String { String::new() }

/*** Types ***/

fn generate_user_defined_type_name(type_name: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match type_name.node {
        lex_4_25::Token::UserDefinedTypeName => (),
        _ => panic!("Invalid user defined type name")
    }
    for i in 0..=type_name.children.len() - 1{
        match &type_name.children[i].node {
            lex_4_25::Token::Identifier(name) => result.push_str(&name),
            _ => panic!("Invalid user defined type name")
        }
        if i != type_name.children.len() - 1 {
            result.push('.');
        }
    }
    result
}

/*** Testing ***/

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn generate_pragma_test1() {
        let tree = parse_4_25::ParseTree {
            children: vec![
                parse_4_25::ParseNode {
                    node: lex_4_25::Token::Pragma,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("solidity".to_string()).to_leaf()),
                        Box::new(lex_4_25::Token::Version("0.4.25".to_string()).to_leaf())
                    ]
                }
            ]
        };
        let actual_generated = generate(tree);
        let expected_generated = "pragma solidity 0.4.25;";
        assert_eq!(expected_generated, actual_generated);
    }

    #[test]
    fn generate_inheritance_specifier_test1() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::Is,
            children: vec![
                Box::new(parse_4_25::ParseNode {
                    node: lex_4_25::Token::OpenParenthesis,
                    children: vec![
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::UserDefinedTypeName,
                            children: vec![
                                Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf())
                            ]
                        })
                    ]
                })
            ]
        };
        let actual_generated = generate_inheritance_specifier(&node);
        let expected_generated = "is\n    A\n";
        assert_eq!(actual_generated, expected_generated);
    }

    #[test]
    fn generate_inheritance_specifier_test2() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::Is,
            children: vec![
                Box::new(parse_4_25::ParseNode {
                    node: lex_4_25::Token::OpenParenthesis,
                    children: vec![
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::UserDefinedTypeName,
                            children: vec![
                                Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf())
                            ]
                        }),
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::UserDefinedTypeName,
                            children: vec![
                                Box::new(lex_4_25::Token::Identifier("Bat".to_string()).to_leaf()),
                                Box::new(lex_4_25::Token::Identifier("Car".to_string()).to_leaf())
                            ]
                        })
                    ]
                })
            ]
        };
        let actual_generated = generate_inheritance_specifier(&node);
        let expected_generated = "is\n    A,\n    Bat.Car\n";
        assert_eq!(actual_generated, expected_generated);
    }

    #[test]
    fn generate_inheritance_specifier_test3() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::Is,
            children: vec![
                Box::new(parse_4_25::ParseNode {
                    node: lex_4_25::Token::OpenParenthesis,
                    children: vec![
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::UserDefinedTypeName,
                            children: vec![
                                Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf())
                            ]
                        }),
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::UserDefinedTypeName,
                            children: vec![
                                Box::new(lex_4_25::Token::Identifier("Bat".to_string()).to_leaf()),
                                Box::new(lex_4_25::Token::Identifier("Car".to_string()).to_leaf())
                            ]
                        }),
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::UserDefinedTypeName,
                            children: vec![
                                Box::new(lex_4_25::Token::Identifier("foo".to_string()).to_leaf()),
                                Box::new(lex_4_25::Token::Identifier("bar".to_string()).to_leaf()),
                                Box::new(lex_4_25::Token::Identifier("baz".to_string()).to_leaf())
                            ]
                        })
                    ]
                })
            ]
        };
        let actual_generated = generate_inheritance_specifier(&node);
        let expected_generated = "is\n    A,\n    Bat.Car,\n    foo.bar.baz\n";
        assert_eq!(actual_generated, expected_generated);
    }

    #[test]
    fn generate_enum_test1() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::Enum,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("empty".to_string()).to_leaf()),
                Box::new(lex_4_25::Token::OpenBrace.to_leaf())
            ]
        };
        let actual_generated = generate_enum(&node);
        let expected_generated = String::from("enum empty {}");
        assert_eq!(expected_generated, actual_generated);
    }

    #[test]
    fn generate_enum_test2() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::Enum,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("Letters".to_string()).to_leaf()),
                Box::new(parse_4_25::ParseNode {
                    node: lex_4_25::Token::OpenBrace,
                    children: vec![
                        Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf()),
                        Box::new(lex_4_25::Token::Identifier("B".to_string()).to_leaf()),
                        Box::new(lex_4_25::Token::Identifier("C".to_string()).to_leaf())
                    ]
                })
            ]
        };
        let actual_generated = generate_enum(&node);
        let expected_generated = String::from("enum Letters {\n    A,\n    B,\n    C\n}");
        assert_eq!(expected_generated, actual_generated);
    }

    #[test]
    fn generate_user_defined_type_name_test1() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::UserDefinedTypeName,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf())
            ]
        };
        let actual_generated = generate_user_defined_type_name(&node);
        let expected_generated = String::from("A");
        assert_eq!(expected_generated, actual_generated);
    }

    #[test]
    fn generate_user_defined_type_name_test2() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::UserDefinedTypeName,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf()),
                Box::new(lex_4_25::Token::Identifier("b".to_string()).to_leaf())
            ]
        };
        let actual_generated = generate_user_defined_type_name(&node);
        let expected_generated = String::from("A.b");
        assert_eq!(expected_generated, actual_generated);
    }
}
