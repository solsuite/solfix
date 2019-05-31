extern crate solfix;

#[cfg(test)]
mod types_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::parse_type_name;

    #[test]
    fn elementary_type_test1() { 
        let cur = &mut 0;
        let chars = &String::from("address").chars().collect::<Vec<char>>();
        let type_node = parse_type_name(chars, cur);
        match type_node.node {
            lex_4_25::Token::Address => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Address, actual)
        }
        assert_eq!(type_node.children.len(), 0);
    }

    #[test]
    fn user_defined_type_test1() { 
        let cur = &mut 0;
        let chars = &String::from("Address.Enum").chars().collect::<Vec<char>>();
        let type_node = parse_type_name(chars, cur);
        match type_node.node {
            lex_4_25::Token::Dot => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Dot, actual)
        }
        assert_eq!(type_node.children.len(), 2);
        match &type_node.children[0].node {
            lex_4_25::Token::Identifier(name) => assert_eq!(name, &"Address"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Address".to_string()), actual)
        }
        match &type_node.children[1].node {
            lex_4_25::Token::Identifier(name) => assert_eq!(name, &"Enum"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Enum".to_string()), actual)
        }
        assert_eq!(type_node.children[0].children.len(), 0);
        assert_eq!(type_node.children[1].children.len(), 0);
    }

    #[test]
    fn mapping_test1() {
        let cur = &mut 0;
        let chars = &String::from("mapping (uint256 => uint256)").chars().collect::<Vec<char>>();
        let type_node = parse_type_name(chars, cur);
        match type_node.node {
            lex_4_25::Token::Mapping => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Mapping, actual)
        }
        assert_eq!(type_node.children.len(), 2);
        match &type_node.children[0].node {
            lex_4_25::Token::Uint256 => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Uint256, actual)
        }
        match &type_node.children[1].node {
            lex_4_25::Token::Uint256 => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Uint256, actual)
        }
        assert_eq!(type_node.children[0].children.len(), 0);
        assert_eq!(type_node.children[1].children.len(), 0);
    }
}
