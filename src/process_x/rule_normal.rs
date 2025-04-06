use pest::iterators::Pair;
use crate::{ compile::{ print_rule, Rule }, MetaData };

pub fn process_rule_normal(mut meta_data: Vec<MetaData>, pair: Pair<Rule>) -> Vec<MetaData> {
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

            Rule::declaration | Rule::user_created_function_call => {
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

            _ => {
                print_rule(in_pair);
            }
        }
    }

    meta_data
}
