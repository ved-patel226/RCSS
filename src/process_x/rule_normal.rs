use pest::iterators::Pair;
use crate::{
    compile::{ print_rule, Rule },
    error::{ display_error, RCSSError, get_error_context },
    MetaData,
    Result,
};
use std::collections::HashMap;

pub fn process_rule_normal(
    mut meta_data: Vec<MetaData>,
    mut declarations: HashMap<String, Vec<String>>,
    pair: Pair<Rule>,
    raw_rcss: &str,
    input_path: &str
) -> Result<(Vec<MetaData>, HashMap<String, Vec<String>>)> {
    let inner_pairs = pair.into_inner();
    let mut current_selector: Vec<String> = Vec::new();

    for in_pair in inner_pairs {
        match in_pair.as_rule() {
            Rule::selector => {
                current_selector.push(in_pair.as_str().trim().to_string());
            }

            Rule::right_curly_brace => {
                if !current_selector.is_empty() {
                    current_selector.pop();
                }
            }

            Rule::declaration => {
                let declaration_inner = in_pair.clone().into_inner();
                let mut variable_reference = String::new();

                for dec_in_pair in declaration_inner {
                    match dec_in_pair.as_rule() {
                        Rule::variable_reference => {
                            variable_reference = dec_in_pair.as_str().to_string();
                        }

                        _ => {}
                    }
                }

                let joined_selector = current_selector.join(" ");

                let key = joined_selector.trim();
                let default_value = in_pair.as_str().trim().to_string();

                if !variable_reference.is_empty() {
                    let mut found_var = false;

                    for md in &meta_data {
                        if let MetaData::Variables { name, value } = md {
                            if name == variable_reference.trim_start_matches('&') {
                                found_var = true;

                                let replaced_value = default_value.replace(
                                    &variable_reference,
                                    value
                                );
                                if let Some(values) = declarations.get_mut(key) {
                                    values.push(replaced_value);
                                } else {
                                    declarations.insert(key.to_string(), vec![replaced_value]);
                                }
                            }
                        }
                    }

                    if found_var == false {
                        let position = in_pair.line_col();
                        let line = position.0;
                        let column = position.1;
                        let context = get_error_context(raw_rcss, line, 2);

                        let err = RCSSError::VariableError {
                            file_path: input_path.into(),
                            line: line,
                            column: column,
                            variable_name: variable_reference.trim_start_matches("&").to_string(),
                            message: format!(
                                "Could not find variable: {}",
                                variable_reference.trim_start_matches("&")
                            ),
                            context: context,
                        };

                        display_error(&err);

                        return Err(err);
                    }
                } else {
                    if let Some(values) = declarations.get_mut(key) {
                        values.push(default_value.clone());
                    } else {
                        declarations.insert(key.to_string(), vec![default_value.clone()]);
                    }
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

                for data in &mut meta_data {
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

            _ => {
                print_rule(in_pair);
            }
        }
    }

    Ok((meta_data, declarations))
}
