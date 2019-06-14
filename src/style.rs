use super::lex_4_25;
use super::parse_4_25;

enum Style {
    /*** Contract ***/
    BeforeContract,
    Contract,
    Interface,
    Library,
    AfterContract,
    BeforeContractIdentifier,
    AfterContractIdentifier,
    /*** Pragma Keyword ***/
    BeforePragma,
    Pragma,
    AfterPragma,
    /*** Pragma Identifier ***/
    BeforePragmaIdentifier,
    AfterPragmaIdentifier,
    /*** Pragma Version ***/
    BeforeVersion,
    AfterVersion
}

trait StyleResolver {
    fn resolve(&mut self, style: Style);
}

impl StyleResolver for String {
    fn resolve(&mut self, style: Style) {
        match style {
            /*** Contract ***/
            Style::BeforeContractIdentifier => (),
            Style::AfterContractIdentifier => self.push_str(" "),
            /*** Contract Part ***/
            Style::BeforeContractPartOpenBrace => (),
            Style::ContractPartOpenBrace => self.push_str("{"),
            Style::AfterContractPartOpenBrace => (),

            Style::BeforeIs => self.push_str(" "),
            Style::Is => self.push_str("is"),
            Style::AfterIs => self.push_str("\n"),

            Style::BeforeInheritanceName => self.push_str("    "),
            Style::AfterInheritanceName => self.push_str(",\n"),
            Style::AfterInheritanceNameLast => self.push_str(",\n"),
            Style::BeforeContract => self.push_str("\n\n\n"),
            Style::Contract => self.push_str("contract"),
            Style::Interface => self.push_str("interface"),
            Style::Library => self.push_str("library"),
            Style::AfterContract => self.push_str(" "),
            Style::BeforePragma => (),
            Style::Pragma => self.push_str("pragma"),
            Style::BeforePragma => (),
            Style::AfterPragma => self.push_str(" "),
            Style::BeforePragmaIdentifier => (),
            Style::AfterPragmaIdentifier => self.push_str(" "),
            Style::AfterPragma => self.push_str(" "),
            Style::BeforeVersion => (),
            Style::AfterVersion => self.push_str(";"),
            _ => (),
        }
    }
}

/*** Top Level ***/

pub fn stylize(tree: parse_4_25::ParseTree) -> String {
    let mut result = String::new();
    for child in tree.children {
        match child.node {
            lex_4_25::Token::Pragma => result.push_str(&stylize_pragma(child)),
            lex_4_25::Token::Import => result.push_str(&stylize_import(child)),
            lex_4_25::Token::Library   |
            lex_4_25::Token::Contract  |
            lex_4_25::Token::Interface => result.push_str(&stylize_contract(child)),
            _ => panic!("Invalid top level tree")
        }
    }
    result    
}

/*** Pragma ***/

fn stylize_pragma(pragma: parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    if let lex_4_25::Token::Pragma = pragma.node {
        result.resolve(Style::BeforePragma);
        result.resolve(Style::Pragma);
        result.resolve(Style::AfterPragma);
    }
    if pragma.children.len() >= 2 {
        if let lex_4_25::Token::Identifier(name) = &pragma.children[0].node {
            result.resolve(Style::BeforePragmaIdentifier);
            result.push_str(&name);
            result.resolve(Style::AfterPragmaIdentifier);
        }
        if pragma.children.len() == 2 {
            if let lex_4_25::Token::Version(version) = &pragma.children[1].node {
                result.resolve(Style::BeforeVersion);
                result.push_str(&version);
                result.resolve(Style::AfterVersion);
            }
        } else if pragma.children.len() == 3 {
            result.resolve(Style::BeforeVersion);
            if let lex_4_25::Token::BitwiseXor = &pragma.children[1].node {
                result.push('^');
            }
            if let lex_4_25::Token::Version(version) = &pragma.children[2].node {
                result.push_str(&version);
            }
            result.resolve(Style::AfterVersion);
        }
    }
    result
}

/*** Import ***/

fn stylize_import(import: parse_4_25::ParseNode) -> String { String::new() }

/*** Contract ***/

fn stylize_contract(contract: parse_4_25::ParseNode) -> String { 
    let mut result = String::new();
    result.resolve(Style::BeforeContract);
    match contract.node {
        lex_4_25::Token::Contract => result.resolve(Style::Contract),
        lex_4_25::Token::Interface => result.resolve(Style::Interface),
        lex_4_25::Token::Library => result.resolve(Style::Library),
        _ => panic!("Invalid contract definition")
    }
    result.resolve(Style::AfterContract);
    if contract.children.len() == 2 {
        match &contract.children[0].node {
            lex_4_25::Token::Identifier(name) => {
                result.resolve(Style::BeforeContractIdentifier);
                result.push_str(&name);
                result.resolve(Style::AfterContractIdentifier);
            }
            _ => panic!("Invalid contract definition")
        }
        match &contract.children[1].node {
            lex_4_25::Token::OpenBrace => result.push_str(&stylize_contract_part(&contract.children[1])),
            _ => panic!("Invalid contract definition")
        }
    } else if contract.children.len() == 3 {
        match &contract.children[0].node {
            lex_4_25::Token::Identifier(name) => {
                result.resolve(Style::BeforeContractIdentifier);
                result.push_str(&name);
                result.resolve(Style::AfterContractIdentifier);
            }
            _ => panic!("Invalid contract definition")
        }
        match &contract.children[1].node {
            lex_4_25::Token::Is => result.push_str(&stylize_inheritance_specifier(&contract.children[1])),
            _ => panic!("Invalid contract definition")
        }
        match &contract.children[2].node {
            lex_4_25::Token::OpenBrace => result.push_str(&stylize_contract_part(&contract.children[2])),
            _ => panic!("Invalid contract definition")
        }
    }
    result
}

/*** Inheritance Specifier ***/

fn stylize_inheritance_specifier(block: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match block.node {
        lex_4_25::Token::Is => {
            result.resolve(Style::BeforeIs);
            result.resolve(Style::Is);
            result.resolve(Style::AfterIs);
        }
        _ => panic!("Invalid inheritance specifier")
    }
    match &block.children[0].node {
        lex_4_25::Token::OpenParenthesis => (),
        _ => panic!("Invalid inheritance specifier")
    }
    for i in 0..=block.children.len() - 1 {
        result.resolve(Style::BeforeInheritanceName);
        result.push_str(&stylize_user_defined_type_name(&block.children[i].children[0]));
        if i != block.children.len() - 1 {
            result.resolve(Style::AfterInheritanceName);
        } else {
            result.resolve(Style::AfterInheritanceNameLast);
        }
    }
    result
}

/*** Contract Part ***/

fn stylize_contract_part(block: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    result.resolve(Style::BeforeContractPartOpenBrace);
    match block.node {
        lex_4_25::Token::OpenBrace => {
            result.resolve(Style::ContractPartOpenBrace);
        }
        _ => panic!("Invalid contract part")
    }
    if block.children.len() > 0 {
        result.resolve(Style::AfterContractPartOpenBrace);
    } else {
        result.resolve(Style::ContractPartCloseBrace);
        return result
    }
    for child in &block.children {
        match child.node {
            lex_4_25::Token::Enum => result.push_str(&stylize_enum_definition(&child)),
            lex_4_25::Token::Event => result.push_str(&stylize_event_declaration(&child)),
            lex_4_25::Token::Function => result.push_str(&stylize_function_definition(&child)),
            lex_4_25::Token::Modifier => result.push_str(&stylize_modifier_definition(&child)),
            lex_4_25::Token::Using => result.push_str(&stylize_using_for_declaration(&child)),
            lex_4_25::Token::Struct => result.push_str(&stylize_struct_definition(&child)),
            lex_4_25::Token::StateVariable => result.push_str(&stylize_state_variable_declaration(&child)),
            _ => panic!("Invalid contract part")
        }
    }
    result.resolve(Style::ContractPartCloseBrace);
    result
}

/*** Sub-contract Structures ***/

fn stylize_state_variable_declaration(node: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    if node.children.len() < 2 {
        panic!("Invalid state variable");
    }
    result.resolve(Style::BeforeStateVariableElementaryType);
    result.push_str(&stylize_elementary_type(&node.children[0]));
    result.resolve(Style::AfterStateVariableElementaryType);
    let mut i = 1;
    let mut stop = false;
    while !stop && i < node.children.len() {
        match &node.children[i].node {
            lex_4_25::Token::Constant => result.push_str("constant"),
            lex_4_25::Token::Internal => result.push_str("internal"),
            lex_4_25::Token::Private => result.push_str("private"),
            lex_4_25::Token::Public => result.push_str("public"),
            _ => stop = true
        }
        if !stop {
            result.push(' ');
            i += 1;
        }
    }
    match &node.children[i].node {
        lex_4_25::Token::Identifier(name) => result.push_str(&name),
        _ => panic!("1Invalid state variable")
    }
    i += 1;
    if i == node.children.len() {
        result.push_str(";\n");
        return result;
    } else {
        result.push(' ');
        match node.children[i].node {
            lex_4_25::Token::Assignment => {
                result.push_str("=");
                result.push_str(" ");
                result.push_str(&stylize_expression(&node.children[i].children[0]));
                result.push_str(";\n");
            }
            _ => panic!("Invalid state variable")
        }
    }
    result
}

fn stylize_enum_definition(node: &parse_4_25::ParseNode) -> String { 
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

fn stylize_event_declaration(node: &parse_4_25::ParseNode) -> String { 
    let mut result = String::new();
    result.push_str("    ");
    match node.node {
        lex_4_25::Token::Event => result.push_str("event"),
        _ => panic!("Invalid event definition")
    }
    result.push(' ');
    match &node.children[0].node {
        lex_4_25::Token::Identifier(name) => result.push_str(&name),
        _ => panic!("Invalid event definition")
    }
    match &node.children[1].node {
        lex_4_25::Token::OpenParenthesis => result.push('('),
        _ => panic!("Invalid event definition")
    }
    if node.children[1].children.len() > 0 {
        for i in 0..=node.children[1].children.len() - 1 {
            result.push('\n');
            result.push_str("        ");
            result.push_str(&stylize_event_parameter(&node.children[1].children[i]));
            if i != node.children[1].children.len() - 1 {
                result.push(',');
            } else {
                result.push('\n');
            }
        }
    }
    result.push_str("    ");
    result.push(')');
    result.push_str(";\n");
    result
}

fn stylize_event_parameter(node: &parse_4_25::ParseNode) -> String { 
    let mut result = String::new();
    match node.node {
        lex_4_25::Token::EventParameter => (),
        _ => panic!("Invalid event parameter")
    }
    // Format the type name of the event parameter and append it to the result
    result.push_str(&stylize_type_name(&node.children[0]));
    if node.children.len() == 2 {
        match &node.children[1].node {
            lex_4_25::Token::Indexed => result.push_str(" indexed"),
            _ => panic!("Invalid event parameter")
        }
    } else if node.children.len() == 3 {
        match &node.children[1].node {
            lex_4_25::Token::Indexed => result.push_str(" indexed "),
            _ => panic!("Invalid event parameter")
        }
        match &node.children[2].node {
            lex_4_25::Token::Identifier(name) => result.push_str(&name),
            _ => panic!("Invalid event parameter")
        }
    } else if node.children.len() != 1 {
        panic!("Invalid event parameter");
    }
    result
}

fn stylize_function_definition(node: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match node.node {
        lex_4_25::Token::Function => result.push_str("function"),
        _ => panic!("Invalid function definition")
    }
    match &node.children[0].node {
        lex_4_25::Token::Identifier(name) => result.push_str(&name),
        _ => panic!("Invalid function definition")
    }
    match &node.children[1].node {
        lex_4_25::Token::OpenParenthesis => result.push('('),
        _ => panic!("Invalid function definition")
    }
    if node.children[1].children.len() > 0 {
        for i in 0..=node.children[1].children.len() - 1 {
            result.push_str("            \n");
            result.push_str(&stylize_parameter(&node.children[1].children[i]));
            if i == node.children[1].children.len() - 1 {
                result.push('\n');
            } else {
                result.push_str(",\n");
            }
        }
    }
    result.push_str("        )\n");
    if node.children.len() >= 3 {
        for i in 2..=node.children.len() - 1 { 
            result.push_str("            \n");
            match &node.children[i].node {
                lex_4_25::Token::External => {
                    result.push_str("            \n");
                    result.push_str("external");
                }
                lex_4_25::Token::Internal => {
                    result.push_str("            \n");
                    result.push_str("internal"); 
                }
                lex_4_25::Token::Pure => {
                    result.push_str("            \n");
                    result.push_str("pure"); 
                }
                lex_4_25::Token::Constant => {
                    result.push_str("            \n");
                    result.push_str("constant");
                }
                lex_4_25::Token::View => {
                    result.push_str("            \n");
                    result.push_str("view");
                }
                lex_4_25::Token::Payable => {
                    result.push_str("            \n");
                    result.push_str("payable");
                }
                lex_4_25::Token::Identifier(..) => {
                    result.push_str("            \n");
                    result.push_str(&stylize_method_invocation(&node.children[i]));
                }
                lex_4_25::Token::Returns => break,
                lex_4_25::Token::OpenBrace => break,
                _ => panic!("Invalid function definition")
            }
        }
    } else {
        panic!("Invalid function definition");
    }
    let last = node.children.len() - 1;
    match &node.children[last - 1].node {
        lex_4_25::Token::Returns => {
            result.push_str("returns");
            match &node.children[last - 1].children[0].node {
                lex_4_25::Token::OpenParenthesis => {
                    result.push('(');
                    for i in 0..=node.children[last - 1].children[0].children.len() - 1 {
                        if i != 0 {
                            result.push(' ');
                        }
                        result.push_str(&stylize_parameter(&node.children[last - 1].children[0].children[i]));
                        if i != node.children[last - 1].children[0].children.len() - 1 {
                            result.push_str(",");
                        }
                    }
                    result.push(')');
                }
                _ => panic!("Invalid function definition")
            }
        }
        _ => ()
    }
    match &node.children[last].node {
        lex_4_25::Token::OpenBrace => result.push_str(&stylize_block(&node.children[last])),
        lex_4_25::Token::Semicolon => result.push(';'),
        _ => panic!("Invalid function definition")
    }
    result.push('\n');
    result
}

fn stylize_block(block: &parse_4_25::ParseNode) -> String { 
    let mut result = String::new();
    match block.node {
        lex_4_25::Token::OpenBrace => result.push_str("{"),
        _ => panic!("Invalid block")
    }
    if block.children.len() == 0 {
        result.push_str("}");
    } else {
        for child in &block.children {
            result.push_str(&stylize_statement(&child));
        }
    }
    result
}

fn stylize_statement(statement: &parse_4_25::ParseNode) -> String { 
    return match statement.node {
        lex_4_25::Token::If => stylize_if_statement(statement),
        lex_4_25::Token::While => stylize_while_statement(statement),
        lex_4_25::Token::For => stylize_for_statement(statement),
        lex_4_25::Token::Assembly => stylize_inline_assembly_statement(statement),
        lex_4_25::Token::Do => stylize_do_while_statement(statement),
        lex_4_25::Token::Placeholder => String::from("\n\n_;"),
        lex_4_25::Token::Emit => stylize_emit_statement(statement),
        // TODO: This actually should be parse_variable_declaration | parse_expression
        _ => {
            let mut result = stylize_expression(statement);
            match statement.node {
                lex_4_25::Token::NoMatch => panic!("Invalid statement"),
                _ => ()
            }
            result.push_str(";");
            result
        }
    }
}

fn stylize_if_statement(if_statement: &parse_4_25::ParseNode) -> String { String::new() }

fn stylize_while_statement(if_statement: &parse_4_25::ParseNode) -> String { String::new() }

fn stylize_for_statement(if_statement: &parse_4_25::ParseNode) -> String { String::new() }

fn stylize_inline_assembly_statement(if_statement: &parse_4_25::ParseNode) -> String { String::new() }

fn stylize_do_while_statement(if_statement: &parse_4_25::ParseNode) -> String { String::new() }

fn stylize_emit_statement(if_statement: &parse_4_25::ParseNode) -> String { String::new() }

fn stylize_method_invocation(node: &parse_4_25::ParseNode) -> String { String::new() }

fn stylize_parameter(node: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match node.node {
        lex_4_25::Token::Parameter => (),
        _ => panic!("Invalid parameter")
    }
    result.push_str(&stylize_type_name(&node.children[0]));
    result.push(' ');
    if node.children.len() == 2 {
        match &node.children[1].node {
            lex_4_25::Token::Identifier(name) => result.push_str(&name),
            lex_4_25::Token::Memory => result.push_str("memory"),
            lex_4_25::Token::Storage => result.push_str("storage"),
            _ => panic!("Invalid parameter")
        }
    } else if node.children.len() == 3 {
        match &node.children[1].node {
            lex_4_25::Token::Memory => result.push_str("memory"),
            lex_4_25::Token::Storage => result.push_str("storage"),
            _ => panic!("Invalid parameter")
        }
        result.push(' ');
        match &node.children[2].node {
            lex_4_25::Token::Identifier(name) => result.push_str(&name),
            _ => panic!("Invalid parameter")
        }
    }
    result
}

fn stylize_modifier_definition(node: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match node.node {
        lex_4_25::Token::Modifier => {
            result.resolve(Style::BeforeModifier);
            result.resolve(Style::Modifier);
            result.resolve(Style::AfterModifier);
        }
        _ => panic!("Invalid modifier definition")
    }
    if node.children.len() == 2 {
        match &node.children[0].node {
            lex_4_25::Token::Identifier(name) => {
                result.resolve(Style::BeforeModifierIdentifier);
                result.push_str(&name);
                result.resolve(Style::AfterModifierIdentifier);
            }
            _ => panic!("Invalid modifier definition")
        }
        result.push_str(&stylize_block(&node.children[1]));
    } else if node.children.len() == 3 {
        match &node.children[0].node {
            lex_4_25::Token::Identifier(name) => {
                result.resolve(Style::BeforeModifierIdentifier);
                result.push_str(&name);
                result.resolve(Style::AfterModifierIdentifier);
            }
            _ => panic!("Invalid modifier definition")
        }
        result.push_str(" ");
        if node.children[1].children.len() > 0 {
            for i in 0..=node.children[1].children.len() - 1 {
                result.push_str("            \n");
                result.push_str(&stylize_parameter(&node.children[1].children[i]));
                if i == node.children[1].children.len() - 1 {
                    result.push('\n');
                } else {
                    result.push_str(",\n");
                }
            }
        }
        result.push_str(&stylize_block(&node.children[1]));
    } else {
        panic!("Invalid modifier definition");
    }
    result.push_str("\n");
    result
}

fn stylize_using_for_declaration(node: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    result
}

fn stylize_struct_definition(node: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match node.node {
        lex_4_25::Token::Struct => {
            result.resolve(Style::BeforeStruct);
            result.resolve(Style::Struct);
            result.resolve(Style::AfterStruct);
        }
        _ => panic!("Invalid struct definition")
    }
    match &node.children[0].node {
        lex_4_25::Token::Identifier(name) => {
            result.resolve(BeforeStructIdentifier);
            result.push_str(&name);
            result.resolve(AfterStructIdentifier);
        }
        _ => panic!("Invalid struct definition")
    }
    match &node.children[1].node {
        lex_4_25::Token::OpenBrace => {
            result.resolve(BeforeStructOpenBrace);
            result.resolve(StructOpenBrace);
            result.resolve(AfterStructOpenBrace);
        }
        _ => panic!("Invalid struct definition")
    }
    for i in 0..=node.children[1].children.len() - 1 {
        result.resolve(Style::BeforeStructVariableDeclaration);
        result.push_str(&stylize_variable_declaration(&node.children[1].children[i]));
        result.resolve(Style::Semicolon);
        result.resolve(Style::AfterStructVariableDeclaration);
    }
    result.resolve(Style::StructCloseBrace);
    result
}

fn stylize_variable_declaration(node: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match node.node {
        lex_4_25::Token::VariableDeclaration => (),
        _ => panic!("Invalid variable declaration")
    }
    result.resolve(Style::BeforeVariableDeclarationTypeName);
    result.push_str(&stylize_type_name(&node.children[0])); 
    result.resolve(Style::AfterVariableDeclarationTypeName);
    if node.children.len() == 2 {
        match &node.children[1].node {
            lex_4_25::Token::Identifier(name) => {
                result.resolve(Style::BeforeVariableDeclarationIdentifier);
                result.push_str(&name);
                result.resolve(Style::AfterVariableDeclarationIdentifier);
            }
            _ => panic!("Invalid variable declaration")
        }
    } else if node.children.len() == 3 {
        match &node.children[1].node {
            lex_4_25::Token::Memory => {
                result.resolve(Style::BeforeMemory);
                result.resolve(Style::Memory);
                result.resolve(Style::AfterMemory);
            }
            lex_4_25::Token::Storage => {
                result.resolve(Style::BeforeStorage);
                result.resolve(Style::Storage);
                result.resolve(Style::AfterStorage);
            }
            _ => panic!("Invalid variable declaration")
        }
        match &node.children[1].node {
            lex_4_25::Token::Identifier(name) => {
                result.resolve(Style::BeforeVariableDeclarationIdentifier);
                result.push_str(&name);
                result.resolve(Style::AfterVariableDeclarationIdentifier);
            }
            _ => panic!("Invalid variable declaration")
        }
    } else {
        panic!("Invalid variable declaration");
    }
    result
}

/*** Types ***/

fn stylize_type_name(type_name: &parse_4_25::ParseNode) -> String { 
    return match type_name.node {
        lex_4_25::Token::Identifier(..) => stylize_user_defined_type_name(type_name),
        lex_4_25::Token::Function => stylize_function_type(type_name),
        lex_4_25::Token::Mapping => stylize_mapping(type_name),
        lex_4_25::Token::OpenBracket => stylize_array_type_name(type_name),
        ref elementary => {
            if elementary.is_elementary_type() {
                return stylize_elementary_type(type_name);
            } else {
                panic!("Invalid type name");
            }
        }

    }
}

fn stylize_user_defined_type_name(type_name: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match type_name.node {
        lex_4_25::Token::UserDefinedTypeName => (),
        _ => panic!("Invalid user defined type name")
    }
    result.resolve(Style::BeforeUserDefinedTypeName);
    for i in 0..=type_name.children.len() - 1{
        match &type_name.children[i].node {
            lex_4_25::Token::Identifier(name) => result.push_str(&name),
            _ => panic!("Invalid user defined type name")
        }
        if i != type_name.children.len() - 1 {
            result.resolve(Style::Dot);
        }
    }
    result.resolve(Style::AfterUserDefinedTypeName);
    result
}

fn stylize_array_type_name(function_type: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    result
}

fn stylize_function_type(function_type: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    result
}

fn stylize_mapping(mapping: &parse_4_25::ParseNode) -> String {
    let mut result = String::new();
    match mapping.node {
        lex_4_25::Token::Mapping => {
            result.resolve(Style::BeforeMapping);
            result.resolve(Style::Mapping);
            result.resolve(Style::AfterMapping);
        }
        _ => panic!("Invalid mapping")
    }
    result
}

fn stylize_elementary_type(elementary_type: &parse_4_25::ParseNode) -> String {
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

/*** Sub-sub-statements ***/

fn stylize_expression(expression: &parse_4_25::ParseNode) -> String { 
    let mut result = String::new();
    // TODO: This needs to be expanded on significantly
    match expression.node {
        // This should do a lookahead parse to check more of an expression
        lex_4_25::Token::DecimalNumber(ref number) => result.push_str(number),
        lex_4_25::Token::HexNumber(ref number) => result.push_str(number),
        _ => panic!("Invalid expression")
    }
    result
}

/*** Testing ***/

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn stylize_pragma_test1() {
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
        let actual_stylized = stylize(tree);
        let expected_stylized = "pragma solidity 0.4.25;";
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_inheritance_specifier_test1() {
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
        let actual_stylized = stylize_inheritance_specifier(&node);
        let expected_stylized = "is\n    A\n";
        assert_eq!(actual_stylized, expected_stylized);
    }

    #[test]
    fn stylize_inheritance_specifier_test2() {
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
        let actual_stylized = stylize_inheritance_specifier(&node);
        let expected_stylized = "is\n    A,\n    Bat.Car\n";
        assert_eq!(actual_stylized, expected_stylized);
    }

    #[test]
    fn stylize_inheritance_specifier_test3() {
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
        let actual_stylized = stylize_inheritance_specifier(&node);
        let expected_stylized = "is\n    A,\n    Bat.Car,\n    foo.bar.baz\n";
        assert_eq!(actual_stylized, expected_stylized);
    }

    #[test]
    fn stylize_state_variable_declaration_test1() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::StateVariable,
            children: vec![
                Box::new(lex_4_25::Token::Uint256.to_leaf()),
                Box::new(lex_4_25::Token::Identifier("nothing".to_string()).to_leaf()),
            ]
        };
        let actual_stylized = stylize_state_variable_declaration(&node);
        let expected_stylized = String::from("\n    uint256 nothing;");
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_state_variable_declaration_test2() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::StateVariable,
            children: vec![
                Box::new(lex_4_25::Token::Uint256.to_leaf()),
                Box::new(lex_4_25::Token::Identifier("total_supply".to_string()).to_leaf()),
                Box::new(parse_4_25::ParseNode{ 
                    node: lex_4_25::Token::Assignment,
                    children: vec![
                        Box::new(lex_4_25::Token::DecimalNumber("10".to_string()).to_leaf())
                    ]
                })
            ]
        };
        let actual_stylized = stylize_state_variable_declaration(&node);
        let expected_stylized = String::from("\n    uint256 total_supply = 10;");
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_enum_test1() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::Enum,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("empty".to_string()).to_leaf()),
                Box::new(lex_4_25::Token::OpenBrace.to_leaf())
            ]
        };
        let actual_stylized = stylize_enum_definition(&node);
        let expected_stylized = String::from("enum empty {}");
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_enum_test2() {
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
        let actual_stylized = stylize_enum_definition(&node);
        let expected_stylized = String::from("enum Letters {\n    A,\n    B,\n    C\n}");
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_event_declaration_test1() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::Event,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("empty".to_string()).to_leaf()),
                Box::new(lex_4_25::Token::OpenParenthesis.to_leaf())
            ]
        };
        let actual_stylized = stylize_event_declaration(&node);
        let expected_stylized = "event empty();";
    }

    #[test]
    fn stylize_event_declaration_test2() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::Event,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("Transfer".to_string()).to_leaf()),
                Box::new(parse_4_25::ParseNode {
                    node: lex_4_25::Token::OpenParenthesis,
                    children: vec![
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::EventParameter,
                            children: vec![
                                Box::new(lex_4_25::Token::Address.to_leaf()),
                                Box::new(lex_4_25::Token::Indexed.to_leaf()),
                                Box::new(lex_4_25::Token::Identifier("owner".to_string()).to_leaf())
                            ]
                        }),
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::EventParameter,
                            children: vec![
                                Box::new(lex_4_25::Token::Address.to_leaf()),
                                Box::new(lex_4_25::Token::Indexed.to_leaf()),
                                Box::new(lex_4_25::Token::Identifier("recipient".to_string()).to_leaf())
                            ]
                        }),
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::EventParameter,
                            children: vec![
                                Box::new(lex_4_25::Token::Uint256.to_leaf()),
                                Box::new(lex_4_25::Token::Indexed.to_leaf()),
                                Box::new(lex_4_25::Token::Identifier("value".to_string()).to_leaf())
                            ]
                        })
                    ]
                })
            ]
        };
        let actual_stylized = stylize_event_declaration(&node);
        let expected_stylized = "event Transfer(\n    address indexed owner,\n    address indexed recipient,\n    uint256 indexed value\n);";
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_struct_definition_test1() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::Struct,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("Value".to_string()).to_leaf()),
                Box::new(parse_4_25::ParseNode {
                    node: lex_4_25::Token::OpenBrace,
                    children: vec![
                        Box::new(parse_4_25::ParseNode {
                            node: lex_4_25::Token::VariableDeclaration,
                            children: vec![
                                Box::new(lex_4_25::Token::Uint256.to_leaf()),
                                Box::new(lex_4_25::Token::Identifier("value".to_string()).to_leaf())
                            ]
                        })
                    ]
                })
            ]
        };
        let actual_stylized = stylize_struct_definition(&node);
        let expected_stylized = "struct Value {\n    uint256 value;\n}";
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_block_test1() {
        let node = parse_4_25::ParseNode { 
            node: lex_4_25::Token::OpenBrace,
            children: vec![]
        };
        let actual_stylized = stylize_block(&node);
        let expected_stylized = "{}";
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_user_defined_type_name_test1() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::UserDefinedTypeName,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf())
            ]
        };
        let actual_stylized = stylize_user_defined_type_name(&node);
        let expected_stylized = String::from("A");
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_user_defined_type_name_test2() {
        let node = parse_4_25::ParseNode {
            node: lex_4_25::Token::UserDefinedTypeName,
            children: vec![
                Box::new(lex_4_25::Token::Identifier("A".to_string()).to_leaf()),
                Box::new(lex_4_25::Token::Identifier("b".to_string()).to_leaf())
            ]
        };
        let actual_stylized = stylize_user_defined_type_name(&node);
        let expected_stylized = String::from("A.b");
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_elementary_type_test1() {
        let node = lex_4_25::Token::Address.to_leaf();
        let actual_stylized = stylize_elementary_type(&node);
        let expected_stylized = "address";
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_elementary_type_test2() {
        let node = lex_4_25::Token::Bool.to_leaf();
        let actual_stylized = stylize_elementary_type(&node);
        let expected_stylized = "bool";
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_elementary_type_test3() {
        let node = lex_4_25::Token::Byte.to_leaf();
        let actual_stylized = stylize_elementary_type(&node);
        let expected_stylized = "byte";
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_elementary_type_test4() {
        let node = lex_4_25::Token::Bytes.to_leaf();
        let actual_stylized = stylize_elementary_type(&node);
        let expected_stylized = "bytes";
        assert_eq!(expected_stylized, actual_stylized);
    }

    #[test]
    fn stylize_elementary_type_test5() {
        let node = lex_4_25::Token::Bytes1.to_leaf();
        let actual_stylized = stylize_elementary_type(&node);
        let expected_stylized = "bytes1";
        assert_eq!(expected_stylized, actual_stylized);
    }
}
