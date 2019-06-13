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
        lex_4_25::Token::OpenBrace => result.push('{'),
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
    result.push('}');
    result
}

fn generate_event(node: &parse_4_25::ParseNode) -> String { 
    let mut result = String::new();
    match node.node {
        lex_4_25::Token::Event => result.push_str("event"),
        _ => panic!("Invalid event definition")
    }
    result.push(' ');
    match &node.children[0].node {
        lex_4_25::Token::Identifier(name) => result.push_str(&name),
        _ => panic!("Invalid event definition")
    }
    result.push(' ');
    match &node.children[1].node {
        lex_4_25::Token::OpenParenthesis => result.push('('),
        _ => panic!("Invalid event definition")
    }
    if node.children[1].children.len() > 0 {
        for i in 0..=node.children[1].children.len() - 1 {
            result.push('\n');
            result.push_str("    ");
            result.push_str(&generate_event_parameter(&node.children[1].children[i]));
            if i != node.children[1].children.len() - 1 {
                result.push(',');
            } else {
                result.push('\n');
            }
        }
    }
    result.push(')');
    result
}

fn generate_event_parameter(node: &parse_4_25::ParseNode) -> String { 
    let mut result = String::new();
    match node.node {
        lex_4_25::Token::EventParameter => (),
        _ => panic!("Invalid event parameter")
    }
    // Format the type name of the event parameter and append it to the result
    result.push_str(&generate_type_name(&node.children[0]));
    if node.children.len() == 2 {
        match &node.children[0].node {
            lex_4_25::Token::Indexed => result.push_str(" indexed"),
            _ => panic!("Invalid event parameter")
        }
    } else if node.children.len() == 3 {
        match &node.children[0].node {
            lex_4_25::Token::Indexed => result.push_str(" indexed "),
            _ => panic!("Invalid event parameter")
        }
        match &node.children[1].node {
            lex_4_25::Token::Identifier(name) => result.push_str(&name),
            _ => panic!("Invalid event parameter")
        }
    } else if node.children.len() != 1 {
        panic!("Invalid event parameter");
    }
    result
}

fn generate_function(node: &parse_4_25::ParseNode) -> String { String::new() }

fn generate_modifier(node: &parse_4_25::ParseNode) -> String { String::new() }

fn generate_using_for(node: &parse_4_25::ParseNode) -> String { String::new() }

fn generate_struct(node: &parse_4_25::ParseNode) -> String { String::new() }

/*** Types ***/

fn generate_type_name(type_name: &parse_4_25::ParseNode) -> String { 
    return match type_name.node {
        lex_4_25::Token::Identifier(..) => generate_user_defined_type_name(type_name),
        lex_4_25::Token::Function => generate_function_type(type_name),
        lex_4_25::Token::Mapping => generate_mapping(type_name),
        lex_4_25::Token::OpenBracket => generate_array_type_name(type_name),
        ref elementary => {
            if elementary.is_elementary_type() {
                return generate_elementary_type(type_name);
            } else {
                panic!("Invalid type name");
            }
        }

    }
}

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

fn generate_array_type_name(function_type: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    result
}

fn generate_function_type(function_type: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    result
}

fn generate_mapping(function_type: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    result
}

fn generate_elementary_type(elementary_type: &parse_4_25::ParseNode) -> String {
    return match elementary_type.node {
        lex_4_25::Token::Address => "address",
        lex_4_25::Token::Bool => "bool",
        lex_4_25::Token::Byte => "byte",
        lex_4_25::Token::Bytes => "bytes",
        lex_4_25::Token::Bytes1 => "bytes1",
        lex_4_25::Token::Bytes2 => "bytes2",
        lex_4_25::Token::Bytes3 => "bytes3",
        lex_4_25::Token::Bytes4 => "bytes4",
        lex_4_25::Token::Bytes5 => "bytes5",
        lex_4_25::Token::Bytes6 => "bytes6",
        lex_4_25::Token::Bytes7 => "bytes7",
        lex_4_25::Token::Bytes8 => "bytes8",
        lex_4_25::Token::Bytes9 => "bytes9",
        lex_4_25::Token::Bytes10 => "bytes10",
        lex_4_25::Token::Bytes11 => "bytes11",
        lex_4_25::Token::Bytes12 => "bytes12",
        lex_4_25::Token::Bytes13 => "bytes13",
        lex_4_25::Token::Bytes14 => "bytes14",
        lex_4_25::Token::Bytes15 => "bytes15",
        lex_4_25::Token::Bytes16 => "bytes16",
        lex_4_25::Token::Bytes17 => "bytes17",
        lex_4_25::Token::Bytes18 => "bytes18",
        lex_4_25::Token::Bytes19 => "bytes19",
        lex_4_25::Token::Bytes20 => "bytes20",
        lex_4_25::Token::Bytes21 => "bytes21",
        lex_4_25::Token::Bytes22 => "bytes22",
        lex_4_25::Token::Bytes23 => "bytes23",
        lex_4_25::Token::Bytes24 => "bytes24",
        lex_4_25::Token::Bytes25 => "bytes25",
        lex_4_25::Token::Bytes26 => "bytes26",
        lex_4_25::Token::Bytes27 => "bytes27",
        lex_4_25::Token::Bytes28 => "bytes28",
        lex_4_25::Token::Bytes29 => "bytes29",
        lex_4_25::Token::Bytes30 => "bytes30",
        lex_4_25::Token::Bytes31 => "bytes31",
        lex_4_25::Token::Bytes32 => "bytes32",
        lex_4_25::Token::Int => "int",
        lex_4_25::Token::Int8 => "int8",
        lex_4_25::Token::Int16 => "int16",
        lex_4_25::Token::Int24 => "int24",
        lex_4_25::Token::Int32 => "int32",
        lex_4_25::Token::Int40 => "int40",
        lex_4_25::Token::Int48 => "int48",
        lex_4_25::Token::Int56 => "int56",
        lex_4_25::Token::Int64 => "int64",
        lex_4_25::Token::Int72 => "int72",
        lex_4_25::Token::Int80 => "int80",
        lex_4_25::Token::Int88 => "int88",
        lex_4_25::Token::Int96 => "int96",
        lex_4_25::Token::Int104 => "int104",
        lex_4_25::Token::Int112 => "int112",
        lex_4_25::Token::Int120 => "int120",
        lex_4_25::Token::Int128 => "int128",
        lex_4_25::Token::Int136 => "int136",
        lex_4_25::Token::Int144 => "int144",
        lex_4_25::Token::Int152 => "int152",
        lex_4_25::Token::Int160 => "int160",
        lex_4_25::Token::Int168 => "int168",
        lex_4_25::Token::Int176 => "int176",
        lex_4_25::Token::Int184 => "int184",
        lex_4_25::Token::Int192 => "int192",
        lex_4_25::Token::Int200 => "int200",
        lex_4_25::Token::Int208 => "int208",
        lex_4_25::Token::Int216 => "int216",
        lex_4_25::Token::Int224 => "int224",
        lex_4_25::Token::Int232 => "int232",
        lex_4_25::Token::Int240 => "int240",
        lex_4_25::Token::Int248 => "int248",
        lex_4_25::Token::Int256 => "int256",
        lex_4_25::Token::String => "string",
        lex_4_25::Token::Var => "var",
        lex_4_25::Token::Uint => "uint",
        lex_4_25::Token::Uint8 => "uint8",
        lex_4_25::Token::Uint16 => "uint16",
        lex_4_25::Token::Uint24 => "uint24",
        lex_4_25::Token::Uint32 => "uint32",
        lex_4_25::Token::Uint40 => "uint40",
        lex_4_25::Token::Uint48 => "uint48",
        lex_4_25::Token::Uint56 => "uint56",
        lex_4_25::Token::Uint64 => "uint64",
        lex_4_25::Token::Uint72 => "uint72",
        lex_4_25::Token::Uint80 => "uint80",
        lex_4_25::Token::Uint88 => "uint88",
        lex_4_25::Token::Uint96 => "uint96",
        lex_4_25::Token::Uint104 => "uint104",
        lex_4_25::Token::Uint112 => "uint112",
        lex_4_25::Token::Uint120 => "uint120",
        lex_4_25::Token::Uint128 => "uint128",
        lex_4_25::Token::Uint136 => "uint136",
        lex_4_25::Token::Uint144 => "uint144",
        lex_4_25::Token::Uint152 => "uint152",
        lex_4_25::Token::Uint160 => "uint160",
        lex_4_25::Token::Uint168 => "uint168",
        lex_4_25::Token::Uint176 => "uint176",
        lex_4_25::Token::Uint184 => "uint184",
        lex_4_25::Token::Uint192 => "uint192",
        lex_4_25::Token::Uint200 => "uint200",
        lex_4_25::Token::Uint208 => "uint208",
        lex_4_25::Token::Uint216 => "uint216",
        lex_4_25::Token::Uint224 => "uint224",
        lex_4_25::Token::Uint232 => "uint232",
        lex_4_25::Token::Uint240 => "uint240",
        lex_4_25::Token::Uint248 => "uint248",
        lex_4_25::Token::Uint256 => "uint256",
        _ => panic!("Invalid elementary type")
    }.to_string()
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

    #[test]
    fn generate_elementary_type_test1() {
        let node = lex_4_25::Token::Address.to_leaf();
        let actual_generated = generate_elementary_type(&node);
        let expected_generated = "address";
        assert_eq!(expected_generated, actual_generated);
    }

    #[test]
    fn generate_elementary_type_test2() {
        let node = lex_4_25::Token::Bool.to_leaf();
        let actual_generated = generate_elementary_type(&node);
        let expected_generated = "bool";
        assert_eq!(expected_generated, actual_generated);
    }

    #[test]
    fn generate_elementary_type_test3() {
        let node = lex_4_25::Token::Byte.to_leaf();
        let actual_generated = generate_elementary_type(&node);
        let expected_generated = "byte";
        assert_eq!(expected_generated, actual_generated);
    }

    #[test]
    fn generate_elementary_type_test4() {
        let node = lex_4_25::Token::Bytes.to_leaf();
        let actual_generated = generate_elementary_type(&node);
        let expected_generated = "bytes";
        assert_eq!(expected_generated, actual_generated);
    }

    #[test]
    fn generate_elementary_type_test5() {
        let node = lex_4_25::Token::Bytes1.to_leaf();
        let actual_generated = generate_elementary_type(&node);
        let expected_generated = "bytes1";
        assert_eq!(expected_generated, actual_generated);
    }
}
