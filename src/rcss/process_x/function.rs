use std::collections::HashMap;
use crate::{ rcss::compiler::Function, MetaDataValue, Rule };
use colored::*;

pub fn process_function_definition(
    function_pair: pest::iterators::Pair<Rule>
) -> Option<MetaDataValue> {
    let mut name = String::new();
    let mut declarations = Vec::new();

    for pair in function_pair.into_inner() {
        match pair.as_rule() {
            Rule::function_name => {
                name = pair.as_str().to_string();
            }
            Rule::parameter_list => {
                // We'll implement parameters later, just collect the structure for now
                // Current implementation may be kept empty as parameter functionality will be added later
            }
            Rule::function_block => {
                for decl in pair.into_inner() {
                    if decl.as_rule() == Rule::declaration {
                        declarations.push(decl.as_str().trim().to_string());
                    }
                }
            }
            _ => {
                // Handle all other cases
            }
        }

        if !name.is_empty() {
            return Some(
                MetaDataValue::Function(Function {
                    name,
                    parameters: vec![],
                    declarations,
                })
            );
        } else {
            return None;
        }
    }
    None
}

pub fn process_function_call(
    call_pair: pest::iterators::Pair<Rule>,
    meta_data: &HashMap<String, MetaDataValue>,
    human_readable: bool,
    verbose: bool
) -> Option<String> {
    let function_name = call_pair.as_str().trim_end_matches("();").trim();

    if let Some(MetaDataValue::Function(function)) = meta_data.get(function_name) {
        if verbose {
            println!("{} {}", "Called: ".blue().bold(), format!("{}()", function_name));
        }

        let mut result = String::new();
        let newline = if human_readable { "\n" } else { "" };
        let indent_size = if human_readable { 4 } else { 0 };
        let space = if human_readable { " " } else { "" };

        for decl in &function.declarations {
            result.push_str(
                &format!(
                    "{}{}{}{}",
                    space.repeat(indent_size),
                    decl,
                    if !decl.ends_with(";") {
                        ";"
                    } else {
                        ""
                    },
                    newline
                )
            );
        }

        Some(result)
    } else {
        None
    }
}
