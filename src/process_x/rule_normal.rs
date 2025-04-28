use pest::iterators::Pair;
use crate::{
    compile::Rule,
    error::{ display_error, RCSSError, get_error_context },
    MetaData,
    Result,
};
use std::collections::HashMap;

pub fn process_rule_normal(
    meta_data: Vec<MetaData>,
    mut declarations: HashMap<String, Vec<String>>,
    pair: Pair<Rule>,
    raw_rcss: &str,
    input_path: &str
) -> Result<HashMap<String, Vec<String>>> {
    let inner_pairs = pair.into_inner();
    let mut current_selector: Vec<String> = Vec::new();

    for in_pair in inner_pairs {
        match in_pair.as_rule() {
            Rule::selector => {
                // if not the pseduo thing
                let selector_str = in_pair.as_str().trim();

                if selector_str.starts_with("&::") || selector_str.starts_with("&:") {
                    // Handle parent selector reference with pseudo-elements/classes
                    if let Some(last) = current_selector.last_mut() {
                        // Append the pseudo-element/class to the parent selector
                        *last = format!("{}{}", last, selector_str.trim_start_matches('&'));
                    } else {
                        // This shouldn't happen with valid RCSS, but handle it gracefully
                        current_selector.push(selector_str.trim_start_matches('&').to_string());
                    }
                } else {
                    // Regular selector (no parent reference)
                    current_selector.push(selector_str.to_string());
                }

                println!("{:?}", current_selector);
            }

            Rule::right_curly_brace => {
                if let Some(last) = current_selector.last_mut() {
                    // Split by either ':' or '::', whichever comes first
                    let mut split_idx = None;

                    if let Some(idx) = last.find("::") {
                        split_idx = Some(idx);
                    } else if let Some(idx) = last.find(':') {
                        // Ensure it's the first ':' from the left, not part of '::'
                        if last.get(idx..idx + 2) != Some("::") {
                            split_idx = Some(idx);
                        }
                    }

                    if let Some(idx) = split_idx {
                        *last = last[..idx].to_string();
                        continue;
                    }
                }

                if !current_selector.is_empty() {
                    current_selector.pop();
                }
            }

            Rule::declaration => {
                // Extract "color" and "height" from the declaration
                let mut decl_str = in_pair.as_str().trim().to_string();
                // Split by ':' to get property and value

                let mut referenced_vars = Vec::new();

                if let Some((_property, value)) = decl_str.split_once(':') {
                    for token in value.split_whitespace() {
                        if token.starts_with("&") {
                            let var = token
                                .trim_start_matches('&')
                                .trim_end_matches(|c| (c == ';' || c == ','));
                            referenced_vars.push(var);
                        }
                    }
                }

                // Collect all replacements to avoid borrowing issues
                let mut replacements = Vec::new();
                for data in &meta_data {
                    if let MetaData::Variables { name, value: var_value } = data {
                        if referenced_vars.contains(&name.as_str()) {
                            replacements.push((format!("&{}", name), var_value.clone()));
                            referenced_vars.retain(|v| v != name);
                        }
                    }
                }

                for (pattern, replacement) in replacements {
                    decl_str = decl_str.replace(&pattern, &replacement);
                }

                let joined_selector = current_selector.join(" ");
                let key = joined_selector.trim();

                if let Some(values) = declarations.get_mut(key) {
                    values.push(decl_str.clone());
                } else {
                    declarations.insert(key.to_string(), vec![decl_str.clone()]);
                }
            }

            Rule::user_created_function_call => {
                let user_created_func_inner_pairs = in_pair.clone().into_inner();
                let mut func_name = String::new();
                let mut func_declarations: Vec<String> = Vec::new();

                for ucfunc_in_pair in user_created_func_inner_pairs {
                    match ucfunc_in_pair.as_rule() {
                        Rule::function_name => {
                            func_name = ucfunc_in_pair.as_str().trim().to_string();
                        }

                        _ => {}
                    }
                }

                for data in &meta_data {
                    if let MetaData::Function { name, body } = data {
                        if func_name == *name {
                            func_declarations = body.clone();
                        }
                    }
                }

                if func_declarations.is_empty() {
                    let position = in_pair.line_col();
                    let line = position.0;
                    let column = position.1;
                    let context = get_error_context(raw_rcss, line, 2);

                    let err = RCSSError::FunctionError {
                        file_path: input_path.to_string().into(),
                        function_name: func_name,
                        message: "Function not declared in scope".to_string(),
                        line: line,
                        column: column,
                        context: context,
                    };

                    display_error(&err);

                    return Err(err);
                }

                let joined_selector = current_selector.join(" ");

                let key = joined_selector.trim();

                if let Some(values) = declarations.get_mut(key) {
                    values.extend(func_declarations.clone());
                } else {
                    declarations.insert(key.to_string(), func_declarations.clone());
                }
            }

            _ => {}
        }
    }

    Ok(declarations)
}
