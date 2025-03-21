use std::collections::HashMap;
use crate::{ Rule, process_rule, MetaDataValue };

pub fn process_media_query(
    media_query_pair: pest::iterators::Pair<Rule>,
    meta_data: &HashMap<String, HashMap<std::string::String, MetaDataValue>>,
    human_readable: bool,
    verbose: bool
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
                inner_rules.push(process_rule(pair, meta_data, human_readable, verbose));
            }
            Rule::media_query => {
                // Handle nested media queries if needed
                inner_rules.push(process_media_query(pair, meta_data, verbose, human_readable));
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
