use std::collections::HashMap;
use clap::error::{ Error, Result };

use crate::{ Rule, MetaDataValue };

#[allow(unused_variables)]
pub fn process_keyframes(
    rule_pair: pest::iterators::Pair<Rule>,
    meta_data: &HashMap<String, MetaDataValue>,
    human_readable: bool,
    verbose: bool
) -> Result<String, Error> {
    let mut selectors: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_selector = String::new();
    let mut name = String::new();
    let tab = "\t";

    for inner_pair in rule_pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::keyframes_name => {
                name = inner_pair.as_str().to_string();
            }

            Rule::keyframe_selector_block => {
                for selector_pair in inner_pair.into_inner() {
                    match selector_pair.as_rule() {
                        Rule::keyframe_selector => {
                            let text = selector_pair.as_str().trim().to_string();
                            current_selector = text.clone();

                            selectors.entry(text).or_insert_with(Vec::new);
                        }
                        Rule::declaration => {
                            selectors
                                .entry(current_selector.to_string())
                                .or_insert_with(Vec::new)
                                .push(selector_pair.as_str().trim().to_string());
                        }

                        _ => {}
                    }
                }
            }

            _ => {}
        }
    }

    if name.is_empty() {
        eprintln!("Error: Keyframes name is missing.");
        return Err(Error::raw(clap::error::ErrorKind::InvalidValue, "Keyframes name is missing"));
    }

    let mut sorted_keys: Vec<_> = selectors.keys().cloned().collect();
    if sorted_keys.iter().all(|key| key.ends_with('%')) {
        sorted_keys.sort_by_key(|key| key.trim_end_matches('%').parse::<u32>().unwrap_or(0));
    }

    println!("{:?}", selectors);

    let mut result = String::new();

    result.push_str("@keyframes ");
    result.push_str(&name);
    result.push_str(" {\n");

    for key in selectors.keys() {
        result.push_str(&format!("{}{} {{\n", tab, key));

        for value in selectors.get(key).unwrap_or(&Vec::new()) {
            result.push_str(&format!("{}{}{}\n", tab, tab, value));
        }
        result.push_str(&format!("{}}}\n", tab));
    }

    result.push_str("}");

    Ok(result)
}
