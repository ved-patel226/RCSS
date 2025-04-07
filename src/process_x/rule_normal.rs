use pest::iterators::Pair;
use crate::{ compile::{ print_rule, Rule }, error::{ display_error, RCSSError }, MetaData, Result };

pub fn process_rule_normal(
    mut meta_data: Vec<MetaData>,
    pair: Pair<Rule>,
    raw_scss: &str,
    input_path: &str
) -> Result<Vec<MetaData>> {
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
                let mut found_key = false;

                let joined_selector = current_selector.join(" ");

                let key = joined_selector.trim();
                let value = in_pair.as_str().trim().to_string();

                for data in &mut meta_data {
                    if let MetaData::StyleMap { selector, declarations } = data {
                        if selector == key {
                            found_key = true;
                            declarations.push(value.clone());
                        }
                    }
                }

                if !found_key {
                    meta_data.push(MetaData::StyleMap {
                        selector: key.to_string(),
                        declarations: vec![value],
                    });
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
                    let context = get_error_context(raw_scss, line, 2);

                    let err = RCSSError::FunctionError {
                        file_path: input_path.to_string().into(), //FIXME - acc return the path
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
                let mut found_key = false;

                for data in &mut meta_data {
                    if let MetaData::StyleMap { selector, declarations } = data {
                        if selector == key {
                            found_key = true;
                            declarations.extend(func_declarations.clone());
                        }
                    }
                }

                if !found_key {
                    meta_data.push(MetaData::StyleMap {
                        selector: key.to_string(),
                        declarations: func_declarations.clone(),
                    });
                }
            }

            _ => {
                print_rule(in_pair);
            }
        }
    }

    Ok(meta_data)
}

fn get_error_context(file_content: &str, error_line: usize, context_lines: usize) -> String {
    let lines: Vec<&str> = file_content.lines().collect();

    // Calculate start and end lines for context, ensuring bounds
    let start_line = error_line.saturating_sub(context_lines);
    let end_line = std::cmp::min(error_line + context_lines, lines.len());

    // Build context string with line numbers
    let mut context = String::new();
    for i in start_line..end_line {
        if i < lines.len() {
            context.push_str(&format!("{}\n", lines[i]));
        }
    }

    context
}
