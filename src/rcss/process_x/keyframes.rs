use std::collections::HashMap;
use crate::{ Rule, MetaDataValue };

#[allow(unused_variables)]
pub fn process_keyframes(
    rule_pair: pest::iterators::Pair<Rule>,
    meta_data: &HashMap<String, MetaDataValue>,
    human_readable: bool,
    verbose: bool
) {
    let keyframes: HashMap<String, String> = HashMap::new();

    for inner_pair in rule_pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::keyframes_name => {
                keyframes.insert("name".to_string(), inner_pair.as_str().to_string());
            }

            _ => {}
        }
    }
}
