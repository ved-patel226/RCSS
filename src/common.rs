use pest::iterators::Pair;
use crate::{ compile::Rule, MetaData, error::{ RCSSError, get_error_context, display_error } };

/// Process a declaration containing potential variable references
/// Returns the processed value with variables replaced or an error if variable not found
pub fn process_variable_declaration(
    declaration_pair: &Pair<Rule>,
    meta_data: &[MetaData],
    raw_scss: &str,
    input_path: &str
) -> Result<String, RCSSError> {
    let declaration_inner = declaration_pair.clone().into_inner();
    let mut variable_reference = String::new();

    for dec_in_pair in declaration_inner {
        match dec_in_pair.as_rule() {
            Rule::variable_reference => {
                variable_reference = dec_in_pair.as_str().to_string();
            }
            _ => {}
        }
    }

    let default_value = declaration_pair.as_str().trim().to_string();

    if !variable_reference.is_empty() {
        let mut found_var = false;

        for md in meta_data {
            if let MetaData::Variables { name, value } = md {
                if name == variable_reference.trim_start_matches('&') {
                    found_var = true;
                    let replaced_value = default_value.replace(&variable_reference, value);
                    return Ok(replaced_value);
                }
            }
        }

        if !found_var {
            let position = declaration_pair.line_col();
            let line = position.0;
            let column = position.1;
            let context = get_error_context(raw_scss, line, 2);

            let err = RCSSError::VariableError {
                file_path: input_path.into(),
                line,
                column,
                variable_name: variable_reference.trim_start_matches("&").to_string(),
                message: format!(
                    "Could not find variable: {}",
                    variable_reference.trim_start_matches("&")
                ),
                context,
            };

            display_error(&err);
            return Err(err);
        }
    }

    Ok(default_value)
}
