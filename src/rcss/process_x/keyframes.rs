use std::collections::HashMap;
use crate::{ Rule, MetaDataValue };

#[allow(unused_variables)]
pub fn process_keyframes(
    rule_pair: pest::iterators::Pair<Rule>,
    meta_data: &HashMap<String, MetaDataValue>,
    human_readable: bool,
    verbose: bool
) {
    let mut selectors: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_selector = String::new();
    let mut name = String::new();

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

    println!("{:?}", selectors)
}
