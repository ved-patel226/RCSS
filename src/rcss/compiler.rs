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

pub fn process_media_query(
    media_query_pair: pest::iterators::Pair<Rule>,
    human_readable: bool
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
                inner_rules.push(process_rule(pair, human_readable));
            }
            Rule::media_query => {
                // Handle nested media queries if needed
                inner_rules.push(process_media_query(pair, human_readable));
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

pub fn process_rule(rule_pair: pest::iterators::Pair<Rule>, human_readable: bool) -> String {
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
