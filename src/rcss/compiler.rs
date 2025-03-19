use crate::Rule;
use std::collections::HashMap;
use colored::*;

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub declarations: Vec<String>,
}

pub fn process_variable(var_pair: pest::iterators::Pair<Rule>) -> Option<(String, String)> {
    let mut name = String::new();
    let mut value = String::new();

    for pair in var_pair.into_inner() {
        match pair.as_rule() {
            Rule::variable_name => {
                name = pair.as_str().to_string();
            }

            Rule::string_literal => {
                let raw_str = pair.as_str();
                if raw_str.len() >= 2 {
                    value = raw_str[1..raw_str.len() - 1].to_string();
                } else {
                    value = raw_str.to_string();
                }
            }
            _ => {}
        }
    }

    if !name.is_empty() && !value.is_empty() {
        Some((name, value))
    } else {
        None
    }
}

pub fn process_function_definition(function_pair: pest::iterators::Pair<Rule>) -> Option<Function> {
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
            _ => {}
        }
    }

    if !name.is_empty() {
        Some(Function {
            name,
            parameters: vec![], // Empty parameters for now
            declarations,
        })
    } else {
        None
    }
}

pub fn process_function_call(
    call_pair: pest::iterators::Pair<Rule>,
    functions: &HashMap<String, Function>,
    human_readable: bool
) -> Option<String> {
    let function_name = call_pair.as_str().trim_end_matches("();").trim();

    if let Some(function) = functions.get(function_name) {
        println!("{} {}", "Called: ".blue().bold(), format!("{}()", function_name));

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

pub fn process_media_query(
    media_query_pair: pest::iterators::Pair<Rule>,
    human_readable: bool,
    functions: &HashMap<String, Function>
) -> String {
    let mut result = String::new();
    let newline = if human_readable { "\n" } else { "" };
    let space = if human_readable { " " } else { "" };

    let mut condition = String::new();
    let mut inner_rules = Vec::new();

    for pair in media_query_pair.into_inner() {
        match pair.as_rule() {
            Rule::media_condition => {
                condition = pair.as_str().trim().to_string();
            }
            Rule::rule_normal => {
                inner_rules.push(process_rule(pair, human_readable, functions));
            }
            Rule::media_query => {
                // Handle nested media queries if needed
                inner_rules.push(process_media_query(pair, human_readable, functions));
            }
            Rule::rule_comment => {
                // Handle comments if needed
            }
            _ => {}
        }
    }

    // Format the media query
    result.push_str(&format!("@media{}{}{{{}", space, condition, newline));

    // Add inner rules with proper indentation
    if human_readable {
        for rule in inner_rules {
            // Indent each line of the inner rule
            for line in rule.lines() {
                result.push_str(&format!("    {}{}", line, newline));
            }
        }
    } else {
        for rule in inner_rules {
            result.push_str(&rule);
        }
    }

    result.push_str(&format!("}}{}", newline));

    result
}

pub fn process_rule(
    rule_pair: pest::iterators::Pair<Rule>,
    human_readable: bool,
    functions: &HashMap<String, Function>
) -> String {
    let inner_pairs = rule_pair.into_inner();
    let mut previous_selector = String::new();
    let mut result = String::new();
    let newline = if human_readable { "\n" } else { "" };
    let space = if human_readable { " " } else { "" };
    let indent_size = if human_readable { 4 } else { 0 };

    for pair in inner_pairs {
        match pair.as_rule() {
            Rule::selector => {
                let trimmed_selector = pair.as_str().trim();

                if previous_selector.is_empty() {
                    result.push_str(&format!("{}{}{{{}", trimmed_selector, space, newline));
                } else {
                    result.push_str(
                        &format!(
                            "}}{}{} {}{}{{{}",
                            space,
                            previous_selector.trim(),
                            trimmed_selector,
                            space,
                            newline
                        )
                    );
                }
                previous_selector.push_str(trimmed_selector);
                previous_selector.push_str(space);
            }
            Rule::declaration => {
                result.push_str(
                    &format!("{}{}{}", space.repeat(indent_size), pair.as_str().trim(), newline)
                );
            }
            Rule::user_created_function_call => {
                if
                    let Some(function_content) = process_function_call(
                        pair,
                        functions,
                        human_readable
                    )
                {
                    result.push_str(&function_content);
                }
            }

            _ => {
                println!("Rule: {:?}, Content: {}", pair.as_rule(), pair.as_str().trim());
            }
        }
    }

    result.push_str("}");
    if human_readable {
        result.push_str("\n");
    }
    result
}
