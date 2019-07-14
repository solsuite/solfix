extern crate solfix;

#[cfg(test)]
mod parse_tests {
    use solfix::lex_4_25;
    use solfix::parse_4_25::{ NonTerminal, parse, ParseTree };

    /*** Helpers ***/

    fn boxed_parse_tree(root: NonTerminal, leaves: Vec<Box<ParseTree>>) -> Box<ParseTree> {
        Box::new(ParseTree {
            root: root,
            leaves: leaves,
        })
    }

    trait TestUtils {
        fn to_boxed_leaf(self) -> Box<ParseTree>;
    }

    impl TestUtils for lex_4_25::Token {
        fn to_boxed_leaf(self) -> Box<ParseTree> {
            Box::new(NonTerminal::Token(self).to_leaf())
        }
    }

    impl TestUtils for NonTerminal {
        fn to_boxed_leaf(self) -> Box<ParseTree> {
            Box::new(self.to_leaf())
        }
    }

    trait PrettyAssertions<T> {
        fn assert_equal_pretty(self, other: T);
    }

    macro_rules! assert_eq_pretty {
        ($i1:ident, $i2:ident) => {
            assert_eq!($i1, $i2, "Expected\n========\n{:#?}\nActual\n======\n{:#?}", $i1, $i2);
        }
    }

    /*** Pragma ***/

    #[test]
    fn pragma_test1() {
        let actual_tree = parse(String::from("pragma solidity 0.4.25;"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::PragmaDirective,
                    vec![
                        NonTerminal::Identifier(String::from("solidity")).to_boxed_leaf(),
                        NonTerminal::Token(lex_4_25::Token::Version(String::from("0.4.25"))).to_boxed_leaf()
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn pragma_test2() {
        let actual_tree = parse(String::from("pragma solidity ^0.4.25;"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::PragmaDirective,
                    vec![
                        NonTerminal::Identifier(String::from("solidity")).to_boxed_leaf(),
                        lex_4_25::Token::BitwiseXor.to_boxed_leaf(),
                        NonTerminal::Token(lex_4_25::Token::Version(String::from("0.4.25"))).to_boxed_leaf()
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    /*** Contract ***/

    #[test]
    fn contract_test1() {
        let actual_tree = parse(String::from("contract A {}"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("A")).to_boxed_leaf(),
                        NonTerminal::ContractPart.to_boxed_leaf()
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_test2() {
        let actual_tree = parse(String::from("contract B is A {}"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("B")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::InheritanceList,
                            vec! [
                                boxed_parse_tree(
                                    NonTerminal::InheritanceSpecifier,
                                    vec![
                                        boxed_parse_tree(
                                            NonTerminal::UserDefinedTypeName,
                                            vec![
                                                NonTerminal::Identifier(String::from("A")).to_boxed_leaf()
                                            ]
                                        )
                                    ]
                                )
                            ]
                        ),
                        NonTerminal::ContractPart.to_boxed_leaf()
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_enum_test1() {
        let actual_tree = parse(String::from("contract Enum { enum Foo { } }"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Enum")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::EnumDefinition,
                                    vec![
                                        NonTerminal::Identifier(String::from("Foo")).to_boxed_leaf(),
                                        NonTerminal::EnumValueList.to_boxed_leaf()
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_enum_test2() {
        let actual_tree = parse(String::from("contract Enum { enum Foo { Bar, Baz } }"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Enum")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::EnumDefinition,
                                    vec![
                                        NonTerminal::Identifier(String::from("Foo")).to_boxed_leaf(),
                                        boxed_parse_tree(
                                            NonTerminal::EnumValueList,
                                            vec![
                                                NonTerminal::EnumValue(String::from("Bar")).to_boxed_leaf(),
                                                NonTerminal::EnumValue(String::from("Baz")).to_boxed_leaf(),
                                            ]
                                        )
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_event_test1() {
        let actual_tree = parse(String::from("contract Event { event emptyEvent(); }"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Event")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::EventDefinition,
                                    vec![
                                        NonTerminal::Identifier(String::from("emptyEvent")).to_boxed_leaf(),
                                        NonTerminal::EventParameterList.to_boxed_leaf()
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_event_test2() {
        let actual_tree = parse(String::from("contract Event { event Transfer(address indexed from, address indexed to, uint256 indexed value); }"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Event")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::EventDefinition,
                                    vec![
                                        NonTerminal::Identifier(String::from("Transfer")).to_boxed_leaf(),
                                        boxed_parse_tree(
                                            NonTerminal::EventParameterList,
                                            vec![
                                                boxed_parse_tree(
                                                    NonTerminal::EventParameter,
                                                    vec![
                                                        lex_4_25::Token::Address.to_boxed_leaf(),
                                                        lex_4_25::Token::Indexed.to_boxed_leaf(),
                                                        NonTerminal::Identifier(String::from("from")).to_boxed_leaf()
                                                    ]
                                                ),
                                                boxed_parse_tree(
                                                    NonTerminal::EventParameter,
                                                    vec![
                                                        lex_4_25::Token::Address.to_boxed_leaf(),
                                                        lex_4_25::Token::Indexed.to_boxed_leaf(),
                                                        NonTerminal::Identifier(String::from("to")).to_boxed_leaf()
                                                    ]
                                                ),
                                                boxed_parse_tree(
                                                    NonTerminal::EventParameter,
                                                    vec![
                                                        lex_4_25::Token::Uint256.to_boxed_leaf(),
                                                        lex_4_25::Token::Indexed.to_boxed_leaf(),
                                                        NonTerminal::Identifier(String::from("value")).to_boxed_leaf()
                                                    ]
                                                )
                                            ]
                                        )
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_function_test1() {
        let actual_tree = parse(String::from("contract Function { function doNothing() internal pure { } }"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Function")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::FunctionDefinition,
                                    vec![
                                        NonTerminal::Identifier(String::from("doNothing")).to_boxed_leaf(),
                                        NonTerminal::ParameterList.to_boxed_leaf(),
                                        lex_4_25::Token::Internal.to_boxed_leaf(),
                                        NonTerminal::StateMutability(lex_4_25::Token::Pure).to_boxed_leaf(),
                                        NonTerminal::Block.to_boxed_leaf()
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_function_test2() {
        let actual_tree = parse(String::from("contract Function { function emitEvent() internal { emit someEvent(1 + 1); } }"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Function")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::FunctionDefinition,
                                    vec![
                                        NonTerminal::Identifier(String::from("emitEvent")).to_boxed_leaf(),
                                        NonTerminal::ParameterList.to_boxed_leaf(),
                                        lex_4_25::Token::Internal.to_boxed_leaf(),
                                        boxed_parse_tree(
                                            NonTerminal::Block,
                                            vec![
                                                boxed_parse_tree(
                                                    NonTerminal::EmitStatement,
                                                    vec![
                                                        boxed_parse_tree(
                                                            NonTerminal::FunctionCall,
                                                            vec![
                                                                NonTerminal::Identifier(String::from("someEvent")).to_boxed_leaf(),
                                                                boxed_parse_tree(
                                                                    NonTerminal::FunctionCallArguments,
                                                                    vec![
                                                                        boxed_parse_tree(
                                                                            NonTerminal::ExpressionList,
                                                                            vec![
                                                                                boxed_parse_tree(
                                                                                    NonTerminal::Token(lex_4_25::Token::Plus),
                                                                                    vec![
                                                                                        lex_4_25::Token::DecimalNumber(String::from("1")).to_boxed_leaf(),
                                                                                        lex_4_25::Token::DecimalNumber(String::from("1")).to_boxed_leaf()
                                                                                    ]
                                                                                )
                                                                            ]
                                                                        )
                                                                    ]
                                                                )
                                                            ]
                                                        )
                                                    ]
                                                )
                                            ]
                                        )
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_modifier_test1() {
        let actual_tree = parse(String::from("contract Modifier { modifier doNothing { _;} }"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Modifier")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::ModifierDefinition,
                                    vec![
                                        NonTerminal::Identifier(String::from("doNothing")).to_boxed_leaf(),
                                        boxed_parse_tree(
                                            NonTerminal::Block,
                                            vec![
                                                lex_4_25::Token::Placeholder.to_boxed_leaf()
                                            ]
                                        )
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_using_for_test1() {
        let actual_tree = parse(String::from("contract Using { using SafeMath for uint256; }"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Using")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::UsingForDeclaration,
                                    vec![
                                        NonTerminal::Identifier(String::from("SafeMath")).to_boxed_leaf(),
                                        NonTerminal::Token(lex_4_25::Token::Uint256).to_boxed_leaf()
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }

    #[test]
    fn contract_using_for_test2() {
        let actual_tree = parse(String::from("contract Using { using SafeMath for *; }"));
        let expected_tree = ParseTree {
            root: NonTerminal::SourceUnit,
            leaves: vec![
                boxed_parse_tree(
                    NonTerminal::ContractDefinition(lex_4_25::Token::Contract),
                    vec![
                        NonTerminal::Identifier(String::from("Using")).to_boxed_leaf(),
                        boxed_parse_tree(
                            NonTerminal::ContractPart,
                            vec![
                                boxed_parse_tree(
                                    NonTerminal::UsingForDeclaration,
                                    vec![
                                        NonTerminal::Identifier(String::from("SafeMath")).to_boxed_leaf(),
                                        NonTerminal::Token(lex_4_25::Token::Multiply).to_boxed_leaf()
                                    ]
                                )
                            ]
                        )
                    ]
                )
            ]
        };
        assert_eq_pretty!(expected_tree, actual_tree);
    }
}
