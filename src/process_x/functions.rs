use pest::iterators::Pair;
use crate::{ compile::{ print_rule, Rule }, MetaData };

pub fn process_function_definition(
    mut meta_data: Vec<MetaData>,
    pair: Pair<Rule>
) -> Vec<MetaData> {
    let inner_pairs = pair.into_inner();

    let mut name = String::new();
    let mut declerations: Vec<String> = vec![];

    for in_pair in inner_pairs {
        match in_pair.as_rule() {
            Rule::function_name => {
                name = in_pair.as_str().trim().to_string();
            }

            Rule::function_block => {
                let function_block_inner_pairs = in_pair.into_inner();

                for func_in_pair in function_block_inner_pairs {
                    match func_in_pair.as_rule() {
                        Rule::declaration => {
                            declerations.push(func_in_pair.as_str().trim().to_string());
                        }

                        _ => {}
                    }
                }
            }

            Rule::parameter_list => {}

            _ => {
                print_rule(in_pair);
            }
        }
    }

    meta_data.push(MetaData::Function { name: name, body: declerations });

    meta_data
}
