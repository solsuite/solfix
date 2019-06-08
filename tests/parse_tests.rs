extern crate solfix;

#[cfg(test)]
mod parse_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::parse;

    /*** Pragma ***/

    #[test]
    fn solidity_4_25_pragma_test1() {
        let tree = parse(String::from("pragma solidity 0.4.25;"));
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].node {
            lex_4_25::Token::Pragma => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Pragma, actual)
        }
        assert_eq!(tree.children[0].children.len(), 2);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(id) => assert_eq!(id, &"solidity"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("solidity".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::Version(version) => assert_eq!(version, &"0.4.25"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Version("0.4.25".to_string()), actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 0);
    }

    #[test]
    fn solidity_4_25_pragma_test2() {
        let tree = parse(String::from("pragma solidity ^0.4.25;"));
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].node {
            lex_4_25::Token::Pragma => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Pragma, actual)
        }
        assert_eq!(tree.children[0].children.len(), 2);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(id) => assert_eq!(id, &"solidity"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("solidity".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::Version(version) => assert_eq!(version, &"^0.4.25"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Version("^0.4.25".to_string()), actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 0);
    }

    /*** Contract ***/

    #[test]
    fn contract_test1() {
        let tree = parse(String::from("contract A {}"));
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].node {
            lex_4_25::Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Contract, actual)
        }
        assert_eq!(tree.children[0].children.len(), 2);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"A"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("A".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 0);
    }

    #[test]
    fn contract_test2() {
        let tree = parse(String::from("contract B is A {}"));
        println!("{:?}", tree);
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].node {
            lex_4_25::Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Contract, actual)
        }
        assert_eq!(tree.children[0].children.len(), 3);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"B"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("B".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::Is => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        match &tree.children[0].children[2].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 1);
        assert_eq!(tree.children[0].children[2].children.len(), 0);
        match &tree.children[0].children[1].children[0].node {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children.len(), 1);
        match &tree.children[0].children[1].children[0].children[0].node {
            lex_4_25::Token::Dot => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Dot, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children[0].children.len(), 1);
        match &tree.children[0].children[1].children[0].children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"A"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("A".to_string()), actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children[0].children[0].children.len(), 0);
    }

    #[test]
    fn contract_enum_test1() { 
        let tree = parse(String::from("contract Enum { enum Foo { } }"));
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].node {
            lex_4_25::Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Contract, actual)
        }
        assert_eq!(tree.children[0].children.len(), 2);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"Enum"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Enum".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 1);
        match &tree.children[0].children[1].children[0].node {
            lex_4_25::Token::Enum => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Enum, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children.len(), 2);
        match &tree.children[0].children[1].children[0].children[0].node {
            lex_4_25::Token::Identifier(foo) => assert_eq!(foo, &"Foo"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Foo".to_string()), actual)
        }
        match &tree.children[0].children[1].children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children[0].children[1].children.len(), 0);
    }

    #[test]
    fn contract_enum_test2() { 
        let tree = parse(String::from("contract Enum { enum Foo { Bar, Baz } }"));
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].node {
            lex_4_25::Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Contract, actual)
        }
        assert_eq!(tree.children[0].children.len(), 2);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"Enum"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Enum".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 1);
        match &tree.children[0].children[1].children[0].node {
            lex_4_25::Token::Enum => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Enum, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children.len(), 2);
        match &tree.children[0].children[1].children[0].children[0].node {
            lex_4_25::Token::Identifier(foo) => assert_eq!(foo, &"Foo"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Foo".to_string()), actual)
        }
        match &tree.children[0].children[1].children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children[0].children[1].children.len(), 2);
        match &tree.children[0].children[1].children[0].children[1].children[0].node {
            lex_4_25::Token::Identifier(bar) => assert_eq!(bar, &"Bar"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Bar".to_string()), actual)
        }
        match &tree.children[0].children[1].children[0].children[1].children[1].node {
            lex_4_25::Token::Identifier(baz) => assert_eq!(baz, &"Baz"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Baz".to_string()), actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children[1].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children[0].children[1].children[1].children.len(), 0);
    }

    #[test]
    fn contract_event_test1() { 
        let tree = parse(String::from("contract Event { event emptyEvent(); }"));
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"Event"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Event".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 1);
        match &tree.children[0].children[1].children[0].node {
            lex_4_25::Token::Event => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Event, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children.len(), 2);
        match &tree.children[0].children[1].children[0].children[0].node {
            lex_4_25::Token::Identifier(empty) => assert_eq!(empty, &"emptyEvent"),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Identifier("emptyEvent".to_string()), actual)
        }
        match &tree.children[0].children[1].children[0].children[1].node {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
    }

    #[test]
    fn contract_event_test2() { 
        let tree = parse(String::from("contract Event { event Transfer(address indexed from, address indexed to, uint256 indexed value); }"));
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"Event"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Event".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 1);
        match &tree.children[0].children[1].children[0].node {
            lex_4_25::Token::Event => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Event, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children.len(), 2);
        match &tree.children[0].children[1].children[0].children[0].node {
            lex_4_25::Token::Identifier(empty) => assert_eq!(empty, &"Transfer"),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Identifier("Transfer".to_string()), actual)
        }
        match &tree.children[0].children[1].children[0].children[1].node {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children[0].children[1].children.len(), 3);
        match &tree.children[0].children[1].children[0].children[1].children[0].node {
            lex_4_25::Token::EventParameter => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::EventParameter, actual)
        }
        match &tree.children[0].children[1].children[0].children[1].children[1].node {
            lex_4_25::Token::EventParameter => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::EventParameter, actual)
        }
        match &tree.children[0].children[1].children[0].children[1].children[2].node {
            lex_4_25::Token::EventParameter => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::EventParameter, actual)
        }
    }

    #[test]
    fn contract_function_test1() { 
        let tree = parse(String::from("contract Function { function doNothing() internal pure { } }"));
        println!("{:?}", tree);
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].node {
            lex_4_25::Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Contract, actual)
        }
        assert_eq!(tree.children[0].children.len(), 2);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"Function"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Function".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 1);
        match &tree.children[0].children[1].children[0].node {
            lex_4_25::Token::Function => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Function, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children.len(), 5);
        match &tree.children[0].children[1].children[0].children[0].node {
            lex_4_25::Token::Identifier(function) => assert_eq!(function, &"doNothing"),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Identifier("doNothing".to_string()), actual)
        }
        match &tree.children[0].children[1].children[0].children[1].node {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
        match &tree.children[0].children[1].children[0].children[2].node {
            lex_4_25::Token::Internal => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Internal, actual)
        }
        match &tree.children[0].children[1].children[0].children[3].node {
            lex_4_25::Token::Pure => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Pure, actual)
        }
        match &tree.children[0].children[1].children[0].children[4].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children[0].children[1].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children[0].children[2].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children[0].children[3].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children[0].children[4].children.len(), 0);
    }

    #[test]
    fn contract_function_test2() { 
        let tree = parse(String::from("contract Function { function emitEvent() internal { emit someEvent(1 + 1); } }"));
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].node {
            lex_4_25::Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Contract, actual)
        }
        assert_eq!(tree.children[0].children.len(), 2);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"Function"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Function".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 1);
        let mut tree = *tree.children[0].children[1].clone();
        match &tree.children[0].node {
            lex_4_25::Token::Function => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Function, actual)
        }
        assert_eq!(tree.children[0].children.len(), 4);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(function) => assert_eq!(function, &"emitEvent"),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Identifier("emitEvent".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
        match &tree.children[0].children[2].node {
            lex_4_25::Token::Internal => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Internal, actual)
        }
        match &tree.children[0].children[3].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 0);
        assert_eq!(tree.children[0].children[2].children.len(), 0);
        assert_eq!(tree.children[0].children[3].children.len(), 1);
        tree = *tree.children[0].children[3].clone();
        match &tree.children[0].node {
            lex_4_25::Token::Emit => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Emit, actual)
        }
        assert_eq!(tree.children[0].children.len(), 1);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 2);
        tree = *tree.children[0].children[0].clone();
        match &tree.children[0].node {
            lex_4_25::Token::Identifier(some_event) => assert_eq!(some_event, &"someEvent"),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Identifier("someEvent".to_string()), actual)
        }
        match &tree.children[1].node {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
        assert_eq!(tree.children[0].children.len(), 0);
        assert_eq!(tree.children[1].children.len(), 1);
        tree = *tree.children[1].clone();
        match &tree.children[0].node {
            lex_4_25::Token::Plus => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Plus, actual)
        }
        assert_eq!(tree.children[0].children.len(), 2);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::DecimalNumber(num) => assert_eq!(num, &"1"),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::DecimalNumber("1".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::DecimalNumber(num) => assert_eq!(num, &"1"),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::DecimalNumber("1".to_string()), actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 0);
    }

    #[test]
    fn contract_modifier_test1() { 
        let tree = parse(String::from("contract Modifier { modifier doNothing() { _;} }"));
        assert_eq!(tree.children.len(), 1);
        match &tree.children[0].node {
            lex_4_25::Token::Contract => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Contract, actual)
        }
        assert_eq!(tree.children[0].children.len(), 2);
        match &tree.children[0].children[0].node {
            lex_4_25::Token::Identifier(contract) => assert_eq!(contract, &"Modifier"),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::Identifier("Modifier".to_string()), actual)
        }
        match &tree.children[0].children[1].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual: {:?}", lex_4_25::Token::OpenBrace, actual)
        }
        assert_eq!(tree.children[0].children[0].children.len(), 0);
        assert_eq!(tree.children[0].children[1].children.len(), 1);
        match &tree.children[0].children[1].children[0].node {
            lex_4_25::Token::Modifier => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Modifier, actual)
        }
        assert_eq!(tree.children[0].children[1].children[0].children.len(), 3);
        match &tree.children[0].children[1].children[0].children[0].node {
            lex_4_25::Token::Identifier(modifier) => assert_eq!(modifier, &"doNothing"),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::Identifier("doNothing".to_string()), actual)
        }
        match &tree.children[0].children[1].children[0].children[1].node {
            lex_4_25::Token::OpenParenthesis => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenParenthesis, actual)
        }
        match &tree.children[0].children[1].children[0].children[2].node {
            lex_4_25::Token::OpenBrace => (),
            actual => panic!("Expected: {:?} | Actual {:?}", lex_4_25::Token::OpenBrace, actual)
        }
    }

    #[test]
    fn contract_using_for_test1() { }
}
