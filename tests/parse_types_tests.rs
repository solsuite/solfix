//extern crate solfix;
//
//#[cfg(test)]
//mod types_tests {
//    use solfix::lex_4_25;
//    use solfix::parse_4_25::parse_type_name;
//
//    #[test]
//    fn elementary_type_test1() {
//        let cur = &mut 0;
//        let chars = &String::from("address").chars().collect::<Vec<char>>();
//        let type_node = parse_type_name(chars, cur);
//        match type_node.node {
//            lex_4_25::Token::Address => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Address, actual)
//        }
//        assert_eq!(type_node.children.len(), 0);
//    }
//
//    #[test]
//    fn user_defined_type_test1() {
//        let cur = &mut 0;
//        let chars = &String::from("Address.Enum").chars().collect::<Vec<char>>();
//        let type_node = parse_type_name(chars, cur);
//        match type_node.node {
//            lex_4_25::Token::UserDefinedTypeName => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::UserDefinedTypeName, actual)
//        }
//        assert_eq!(type_node.children.len(), 2);
//        match &type_node.children[0].node {
//            lex_4_25::Token::Identifier(name) => assert_eq!(name, &"Address"),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Address".to_string()), actual)
//        }
//        match &type_node.children[1].node {
//            lex_4_25::Token::Identifier(name) => assert_eq!(name, &"Enum"),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Enum".to_string()), actual)
//        }
//        assert_eq!(type_node.children[0].children.len(), 0);
//        assert_eq!(type_node.children[1].children.len(), 0);
//    }
//
//    #[test]
//    fn mapping_test1() {
//        let cur = &mut 0;
//        let chars = &String::from("mapping (uint256 => uint256)").chars().collect::<Vec<char>>();
//        let type_node = parse_type_name(chars, cur);
//        match type_node.node {
//            lex_4_25::Token::Mapping => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Mapping, actual)
//        }
//        assert_eq!(type_node.children.len(), 2);
//        match &type_node.children[0].node {
//            lex_4_25::Token::Uint256 => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Uint256, actual)
//        }
//        match &type_node.children[1].node {
//            lex_4_25::Token::Uint256 => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Uint256, actual)
//        }
//        assert_eq!(type_node.children[0].children.len(), 0);
//        assert_eq!(type_node.children[1].children.len(), 0);
//    }
//
//    #[test]
//    fn array_type_name_test1() {
//        let cur = &mut 0;
//        let chars = &String::from("uint256[1 + 1]").chars().collect::<Vec<char>>();
//        let type_node = parse_type_name(chars, cur);
//        match type_node.node {
//            lex_4_25::Token::OpenBracket => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBracket, actual)
//        }
//        assert_eq!(type_node.children.len(), 2);
//        match &type_node.children[0].node {
//            lex_4_25::Token::Uint256 => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Uint256, actual)
//        }
//        match &type_node.children[1].node {
//            lex_4_25::Token::Plus => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
//        }
//        assert_eq!(type_node.children[0].children.len(), 0);
//        assert_eq!(type_node.children[1].children.len(), 2);
//        match &type_node.children[1].children[0].node {
//            lex_4_25::Token::DecimalNumber(num) => assert_eq!(num, &"1"),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
//        }
//        match &type_node.children[1].children[1].node {
//            lex_4_25::Token::DecimalNumber(num) => assert_eq!(num, &"1"),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Plus, actual)
//        }
//        assert_eq!(type_node.children[1].children[0].children.len(), 0);
//        assert_eq!(type_node.children[1].children[1].children.len(), 0);
//    }
//
//    #[test]
//    fn function_type_name_test1() {
//        let cur = &mut 0;
//        let chars = &String::from("function () internal").chars().collect::<Vec<char>>();
//        let type_node = parse_type_name(chars, cur);
//        match type_node.node {
//            lex_4_25::Token::Function => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Function, actual)
//        }
//        assert_eq!(type_node.children.len(), 2);
//        match &type_node.children[0].node {
//            lex_4_25::Token::OpenParenthesis => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenParenthesis, actual)
//        }
//        match &type_node.children[1].node {
//            lex_4_25::Token::Internal => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Internal, actual)
//        }
//        assert_eq!(type_node.children[0].children.len(), 0);
//        assert_eq!(type_node.children[1].children.len(), 0);
//    }
//
//    #[test]
//    fn function_type_name_test2() {
//        let cur = &mut 0;
//        let chars = &String::from("function () internal pure").chars().collect::<Vec<char>>();
//        let type_node = parse_type_name(chars, cur);
//        match type_node.node {
//            lex_4_25::Token::Function => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Function, actual)
//        }
//        assert_eq!(type_node.children.len(), 3);
//        match &type_node.children[0].node {
//            lex_4_25::Token::OpenParenthesis => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenParenthesis, actual)
//        }
//        match &type_node.children[1].node {
//            lex_4_25::Token::Internal => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Internal, actual)
//        }
//        match &type_node.children[2].node {
//            lex_4_25::Token::Pure => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Pure, actual)
//        }
//        assert_eq!(type_node.children[0].children.len(), 0);
//        assert_eq!(type_node.children[1].children.len(), 0);
//        assert_eq!(type_node.children[2].children.len(), 0);
//    }
//
//    #[test]
//    fn function_type_name_test3() {
//        let cur = &mut 0;
//        let chars = &String::from("function (uint256, bytes32) internal pure returns (bool) ").chars().collect::<Vec<char>>();
//        let type_node = parse_type_name(chars, cur);
//        match type_node.node {
//            lex_4_25::Token::Function => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Function, actual)
//        }
//        assert_eq!(type_node.children.len(), 4);
//        match &type_node.children[0].node {
//            lex_4_25::Token::OpenParenthesis => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenParenthesis, actual)
//        }
//        match &type_node.children[1].node {
//            lex_4_25::Token::Internal => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Internal, actual)
//        }
//        match &type_node.children[2].node {
//            lex_4_25::Token::Pure => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Pure, actual)
//        }
//        match &type_node.children[3].node {
//            lex_4_25::Token::Returns => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Returns, actual)
//        }
//        assert_eq!(type_node.children[0].children.len(), 2);
//        assert_eq!(type_node.children[1].children.len(), 0);
//        assert_eq!(type_node.children[2].children.len(), 0);
//        assert_eq!(type_node.children[3].children.len(), 1);
//        match &type_node.children[0].children[0].node {
//            lex_4_25::Token::Uint256 => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Uint256, actual)
//        }
//        match &type_node.children[0].children[1].node {
//            lex_4_25::Token::Bytes32 => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Bytes32, actual)
//        }
//        match &type_node.children[3].children[0].node {
//            lex_4_25::Token::OpenParenthesis => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenParenthesis, actual)
//        }
//        assert_eq!(type_node.children[0].children[0].children.len(), 0);
//        assert_eq!(type_node.children[0].children[1].children.len(), 0);
//        assert_eq!(type_node.children[3].children[0].children.len(), 1);
//        match &type_node.children[3].children[0].children[0].node {
//            lex_4_25::Token::Bool => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Bool, actual)
//        }
//        assert_eq!(type_node.children[3].children[0].children[0].children.len(), 0);
//    }
//
//    #[test]
//    fn function_type_name_test4() {
//        let cur = &mut 0;
//        let chars = &String::from("function (function (uint256) internal) external payable returns (function (bool) external returns (uint256)) ").chars().collect::<Vec<char>>();
//        let type_node = parse_type_name(chars, cur);
//        match type_node.node {
//            lex_4_25::Token::Function => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Function, actual)
//        }
//        assert_eq!(type_node.children.len(), 4);
//        match &type_node.children[0].node {
//            lex_4_25::Token::OpenParenthesis => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenParenthesis, actual)
//        }
//        match &type_node.children[1].node {
//            lex_4_25::Token::External => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::External, actual)
//        }
//        match &type_node.children[2].node {
//            lex_4_25::Token::Payable => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Payable, actual)
//        }
//        match &type_node.children[3].node {
//            lex_4_25::Token::Returns => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Returns, actual)
//        }
//        assert_eq!(type_node.children[0].children.len(), 1);
//        assert_eq!(type_node.children[1].children.len(), 0);
//        assert_eq!(type_node.children[2].children.len(), 0);
//        assert_eq!(type_node.children[3].children.len(), 1);
//        match &type_node.children[0].children[0].node {
//            lex_4_25::Token::Function => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Function, actual)
//        }
//        match &type_node.children[3].children[0].node {
//            lex_4_25::Token::OpenParenthesis => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenParenthesis, actual)
//        }
//        assert_eq!(type_node.children[0].children[0].children.len(), 2);
//        assert_eq!(type_node.children[3].children[0].children.len(), 1);
//        match &type_node.children[0].children[0].children[0].node {
//            lex_4_25::Token::OpenParenthesis => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenParenthesis, actual)
//        }
//        match &type_node.children[0].children[0].children[1].node {
//            lex_4_25::Token::Internal => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Internal, actual)
//        }
//        match &type_node.children[3].children[0].children[0].node {
//            lex_4_25::Token::Function => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Function, actual)
//        }
//        assert_eq!(type_node.children[0].children[0].children[0].children.len(), 1);
//        assert_eq!(type_node.children[0].children[0].children[1].children.len(), 0);
//        assert_eq!(type_node.children[3].children[0].children[0].children.len(), 3);
//        match &type_node.children[0].children[0].children[0].children[0].node {
//            lex_4_25::Token::Uint256 => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Uint256, actual)
//        }
//        match &type_node.children[3].children[0].children[0].children[0].node {
//            lex_4_25::Token::OpenParenthesis => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Function, actual)
//        }
//        match &type_node.children[3].children[0].children[0].children[1].node {
//            lex_4_25::Token::External => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::External, actual)
//        }
//        match &type_node.children[3].children[0].children[0].children[2].node {
//            lex_4_25::Token::Returns => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Returns, actual)
//        }
//        assert_eq!(type_node.children[3].children[0].children[0].children[0].children.len(), 1);
//        assert_eq!(type_node.children[3].children[0].children[0].children[1].children.len(), 0);
//        assert_eq!(type_node.children[3].children[0].children[0].children[2].children.len(), 1);
//        match &type_node.children[3].children[0].children[0].children[0].children[0].node {
//            lex_4_25::Token::Bool => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Bool, actual)
//        }
//        match &type_node.children[3].children[0].children[0].children[2].children[0].node {
//            lex_4_25::Token::OpenParenthesis => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenParenthesis, actual)
//        }
//        assert_eq!(type_node.children[3].children[0].children[0].children[2].children[0].children.len(), 1);
//        match &type_node.children[3].children[0].children[0].children[2].children[0].children[0].node {
//            lex_4_25::Token::Uint256 => (),
//            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Uint256, actual)
//        }
//        assert_eq!(type_node.children[3].children[0].children[0].children[2].children[0].children[0].children.len(), 0);
//    }
//}
