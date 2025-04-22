use pest::iterators::Pair;
use crate::{
    compile::{ print_rule, Rule },
    error::{ display_error, get_error_context, RCSSError, Result },
    process_x::rule_normal,
    MetaData,
};
use std::collections::HashMap;

pub fn process_media_query(
    mut media_queries: HashMap<String, HashMap<String, Vec<String>>>,
    pair: Pair<Rule>,

    // Arguments passed to rule_nomral
    meta_data: &Vec<MetaData>,
    raw_rcss: &str,
    input_path: &str
) -> Result<HashMap<String, HashMap<String, Vec<String>>>> {
    let inner_pairs = pair.into_inner();
    let mut condition = String::new();
    let mut declarations: HashMap<String, Vec<String>> = HashMap::new();

    for inner_pair in inner_pairs {
        match inner_pair.as_rule() {
            Rule::media_condition => {
                condition = format!("@media {}", inner_pair.as_str().trim());
            }

            Rule::rule_normal => {
                declarations = rule_normal::process_rule_normal(
                    meta_data.clone(),
                    declarations,
                    inner_pair,
                    raw_rcss,
                    input_path
                )?;
            }

            _ => {}
        }
    }

    media_queries.insert(condition, declarations);

    Ok(media_queries)
}
