use pest::iterators::Pair;
use crate::{ MetaData, compile::Rule };

pub fn process_variable_declaration(
    mut meta_data: Vec<MetaData>,
    pair: Pair<Rule>
) -> Vec<MetaData> {
    let inner_pairs = pair.into_inner();

    let mut name = String::new();
    let mut value = String::new();

    for in_pair in inner_pairs {
        match in_pair.as_rule() {
            Rule::variable_name => {
                name = in_pair.as_str().to_string();
            }

            Rule::string_literal => {
                value = in_pair.as_str().to_string();
            }

            _ => {}
        }
    }

    if name.is_empty() || value.is_empty() {
        return meta_data;
    }

    value = value.trim_matches('"').to_string();

    meta_data.push(MetaData::Variables { name: name, value: value });

    meta_data
}
