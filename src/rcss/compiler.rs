use crate::{ Rule, process_function_call };
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<String>,
    pub declarations: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Keyframes {
    pub name: String,
    pub frames: HashMap<String, Vec<String>>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum MetaDataValue {
    Function(Function),
    Keyframes(Keyframes),
}

pub fn process_rule(
    rule_pair: pest::iterators::Pair<Rule>,
    meta_data: &HashMap<String, HashMap<std::string::String, MetaDataValue>>,
    human_readable: bool,
    verbose: bool
) -> String {
    let inner_pairs = rule_pair.into_inner();
    let mut previous_selector = String::new();
    let space = if human_readable { " " } else { "" };
    let mut element_to_decleration: HashMap<String, Vec<String>> = HashMap::new();

    let binding = HashMap::new();
    let functions = meta_data.get("functions").unwrap_or(&binding);

    for pair in inner_pairs {
        match pair.as_rule() {
            Rule::selector => {
                let trimmed_selector = pair.clone().as_str().trim();

                // push selector ex: (h1, h2, h3, div)
                previous_selector.push_str(trimmed_selector);
                previous_selector.push_str(space);
            }

            Rule::declaration => {
                let key = previous_selector.clone().trim().to_string();
                let value = pair.as_str().trim().to_string();

                element_to_decleration.entry(key).or_insert_with(Vec::new).push(value);
            }

            Rule::user_created_function_call => {
                if
                    let Some(function_content) = process_function_call(
                        pair.clone(),
                        functions,
                        human_readable,
                        verbose
                    )
                {
                    let key = previous_selector.clone().trim().to_string();
                    let value = function_content.trim().to_string();

                    element_to_decleration.entry(key).or_insert_with(Vec::new).push(value);
                }
            }

            Rule::right_curly_brace => {
                // clear last selector
                if let Some(last_space_index) = previous_selector.trim_end().rfind(' ') {
                    previous_selector.truncate(last_space_index);
                } else {
                    previous_selector.clear();
                }
            }

            _ => {}
        }
    }

    // sort by key
    let mut sorted_elements: Vec<_> = element_to_decleration.iter().collect();
    sorted_elements.sort_by_key(|&(key, _)| key);
    let sorted_map: HashMap<_, _> = sorted_elements
        .into_iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    generate_css(&sorted_map, human_readable)
}

pub fn generate_css(elements: &HashMap<String, Vec<String>>, human_readable: bool) -> String {
    let mut result = String::new();
    let newline = if human_readable { "\n" } else { "" };
    let space = if human_readable { " " } else { "" };
    let indent = if human_readable { "    " } else { "" };

    for (selector, declarations) in elements {
        result.push_str(&format!("{}{}{{{}", selector.trim(), space, newline));

        for declaration in declarations {
            let declaration_str = declaration.trim();

            let semicolon = if declaration_str.ends_with(';') { "" } else { ";" };

            result.push_str(&format!("{}{}{}{}", indent, declaration_str, semicolon, newline));
        }

        result.push_str("}");
    }

    result.trim_end().to_string()
}
