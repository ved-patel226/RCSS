use pest::iterators::Pair;
use crate::{
    compile::{ print_rule, Rule },
    MetaData,
    error::{ RCSSError, get_error_context, display_error },
};

pub fn process_function_definition(
    mut meta_data: Vec<MetaData>,
    pair: Pair<Rule>,
    raw_rcss: &str,
    input_path: &str,
    initial_compile: bool
) -> Result<Vec<MetaData>, RCSSError> {
    let inner_pairs = pair.into_inner();

    let mut name = String::new();
    let mut declerations: Vec<String> = vec![];

    for in_pair in inner_pairs {
        match in_pair.as_rule() {
            Rule::function_name => {
                name = in_pair.as_str().trim().to_string();
            }

            Rule::function_block => {
                let function_block_inner_pairs = in_pair.into_inner();

                for func_in_pair in function_block_inner_pairs {
                    match func_in_pair.as_rule() {
                        Rule::declaration => {
                            let declaration_inner = func_in_pair.clone().into_inner();
                            let mut variable_reference = String::new();

                            for dec_in_pair in declaration_inner {
                                match dec_in_pair.as_rule() {
                                    Rule::variable_reference => {
                                        variable_reference = dec_in_pair.as_str().to_string();
                                    }

                                    _ => {}
                                }
                            }

                            let default_value = func_in_pair.as_str().trim().to_string();

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
                                            declerations.push(replaced_value);
                                        }
                                    }
                                }

                                if !found_var {
                                    let position = func_in_pair.line_col();
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
                                declerations.push(default_value);
                            }
                        }

                        _ => {}
                    }
                }
            }

            Rule::parameter_list => {}

            _ => {
                print_rule(in_pair);
            }
        }
    }

    meta_data.push(MetaData::Function { name, body: declerations });

    Ok(meta_data)
}
