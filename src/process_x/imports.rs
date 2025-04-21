use pest::iterators::Pair;
use crate::{
    compile::{ print_rule, Rule },
    error::{ display_error, RCSSError, get_error_context },
    MetaData,
    Result,
};
use std::collections::HashMap;

pub fn process_import_statement(
    mut meta_data: Vec<MetaData>,
    mut project_meta_data: HashMap<String, Vec<MetaData>>,
    pair: Pair<Rule>
) -> Vec<MetaData> {
    print_rule(pair);

    meta_data
}
