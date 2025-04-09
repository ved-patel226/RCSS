use pest::iterators::Pair;
use crate::{ compile::{ print_rule, Rule }, MetaData };
use std::collections::HashMap;

//TODO - Allow variables here
pub fn process_keyframes_definition(
    mut at_methods: HashMap<String, HashMap<String, Vec<String>>>,
    pair: Pair<Rule>
) -> HashMap<String, HashMap<String, Vec<String>>> {
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
                                selector_to_declarations
                                    .entry(current_selector.clone())
                                    .or_insert_with(Vec::new)
                                    .push(ksb_in_pair.as_str().trim().to_string());
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

    at_methods.insert(format!("@keyframes {}", name), selector_to_declarations);

    at_methods
}
