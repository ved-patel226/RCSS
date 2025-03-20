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
            parameters: vec![],
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
    let space = if human_readable { " " } else { "" };
    let mut element_to_decleration: HashMap<String, Vec<String>> = HashMap::new();

    for pair in inner_pairs {
        match pair.as_rule() {
            Rule::selector => {
                let trimmed_selector = pair.clone().as_str().trim();

                // if previous_selector.is_empty() {
                //     result.push_str(&format!("{}{}{{{}", trimmed_selector, space, newline));
                // } else {
                //     result.push_str(
                //         &format!(
                //             "}}{}{} {}{}{{{}",
                //             space,
                //             previous_selector.trim(),
                //             trimmed_selector,
                //             space,
                //             newline
                //         )
                //     );
                // }

                previous_selector.push_str(trimmed_selector);
                previous_selector.push_str(space);
            }

            Rule::declaration => {
                let key = previous_selector.clone().trim().to_string();
                let value = pair.as_str().trim().to_string();

                element_to_decleration.entry(key).or_insert_with(Vec::new).push(value);
            }

            Rule::user_created_function_call => {
                if
                    let Some(function_content) = process_function_call(
                        pair.clone(),
                        functions,
                        human_readable
                    )
                {
                    let key = previous_selector.clone().trim().to_string();
                    let value = function_content.trim().to_string();

                    element_to_decleration.entry(key).or_insert_with(Vec::new).push(value);
                }
            }

            Rule::right_curly_brace => {
                if let Some(last_space_index) = previous_selector.trim_end().rfind(' ') {
                    previous_selector.truncate(last_space_index);
                } else {
                    previous_selector.clear();
                }

                println!("{}", previous_selector);
            }

            _ => {}
        }

        println!("Rule: {:?}, Content: {}", pair.as_rule(), pair.as_str().trim());
    }

    println!("{:?}", element_to_decleration);

    generate_css(&element_to_decleration, human_readable)
}

pub fn generate_css(elements: &HashMap<String, Vec<String>>, human_readable: bool) -> String {
    let mut result = String::new();
    let newline = if human_readable { "\n" } else { "" };
    let space = if human_readable { " " } else { "" };

    for (selector, declarations) in elements {
        // Start CSS rule with selector
        result.push_str(&format!("{}{}{{{}", selector, space, newline));

        // Add declarations with proper indentation
        for declaration in declarations {
            let indent = if human_readable { "    " } else { "" };
            let declaration_str = declaration.trim();

            // Check if declaration ends with semicolon and add if needed
            let semicolon = if declaration_str.ends_with(';') { "" } else { ";" };

            result.push_str(&format!("{}{}{}{}", indent, declaration_str, semicolon, newline));
        }

        // Close CSS rule
        result.push_str(&format!("}}{}", newline));

        // Add empty line between rules for better readability if human_readable
        if human_readable {
            result.push_str(newline);
        }
    }

    result.trim_end().to_string()
}
