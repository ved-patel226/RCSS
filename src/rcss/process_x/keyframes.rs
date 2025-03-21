use std::collections::HashMap;
use crate::{ Rule, MetaDataValue };

#[allow(unused_variables)]
pub fn process_keyframes(
    rule_pair: pest::iterators::Pair<Rule>,
    meta_data: &HashMap<String, MetaDataValue>,
    human_readable: bool,
    verbose: bool
) {
    for inner_pair in rule_pair.into_inner() {
        println!("{:?} - {}", inner_pair.as_rule(), inner_pair.as_str());
    }
}
