use crate::Rule;

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
