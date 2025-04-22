use pest::iterators::Pair;
use crate::{ compile::Rule, MetaData, error::{ RCSSError, get_error_context, display_error } };
use std::collections::HashMap;

pub fn process_keyframes_definition(
    mut keyframes: HashMap<String, HashMap<String, Vec<String>>>,
    pair: Pair<Rule>,
    meta_data: &[MetaData],
    raw_rcss: &str,
    input_path: &str
) -> Result<HashMap<String, HashMap<String, Vec<String>>>, RCSSError> {
    let inner_pairs = pair.into_inner();
    let mut name = String::new();
    let mut selector_to_declarations: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_selector = String::new();

    for in_pair in inner_pairs {
        match in_pair.as_rule() {
            Rule::keyframes_name => {
                name = in_pair.as_str().to_string();
            }

            Rule::keyframe_selector_block => {
                let key_selector_block_inner_pairs = in_pair.into_inner();

                for ksb_in_pair in key_selector_block_inner_pairs {
                    match ksb_in_pair.as_rule() {
                        // from/to/100%
                        Rule::keyframe_selector => {
                            current_selector = ksb_in_pair.as_str().to_string();
                        }

                        // color: red;
                        Rule::declaration => {
                            if !current_selector.is_empty() {
                                let declaration_inner = ksb_in_pair.clone().into_inner();
                                let mut variable_reference = String::new();

                                for dec_in_pair in declaration_inner {
                                    match dec_in_pair.as_rule() {
                                        Rule::variable_reference => {
                                            variable_reference = dec_in_pair.as_str().to_string();
                                        }

                                        _ => {}
                                    }
                                }

                                let default_value = ksb_in_pair.as_str().trim().to_string();

                                if !variable_reference.is_empty() {
                                    let mut found_var = false;

                                    for md in meta_data {
                                        if let MetaData::Variables { name, value } = md {
                                            if name == variable_reference.trim_start_matches('&') {
                                                found_var = true;

                                                let replaced_value = default_value.replace(
                                                    &variable_reference,
                                                    value
                                                );
                                                selector_to_declarations
                                                    .entry(current_selector.clone())
                                                    .or_insert_with(Vec::new)
                                                    .push(replaced_value);
                                            }
                                        }
                                    }

                                    if !found_var {
                                        let position = ksb_in_pair.line_col();
                                        let line = position.0;
                                        let column = position.1;
                                        let context = get_error_context(raw_rcss, line, 2);

                                        let err = RCSSError::VariableError {
                                            file_path: input_path.into(),
                                            line,
                                            column,
                                            variable_name: variable_reference
                                                .trim_start_matches("&")
                                                .to_string(),
                                            message: format!(
                                                "Could not find variable: {}",
                                                variable_reference.trim_start_matches("&")
                                            ),
                                            context,
                                        };

                                        display_error(&err);
                                        return Err(err);
                                    }
                                } else {
                                    selector_to_declarations
                                        .entry(current_selector.clone())
                                        .or_insert_with(Vec::new)
                                        .push(default_value);
                                }
                            }
                        }

                        Rule::right_curly_brace => {
                            current_selector = String::new();
                        }

                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }

    keyframes.insert(format!("@keyframes {}", name), selector_to_declarations);
    Ok(keyframes)
}
