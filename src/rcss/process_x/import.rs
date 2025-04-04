use crate::{ HashMap, MetaDataValue, Rule, Path };

#[allow(dead_code)]
pub fn process_import(
    meta_data_to_file: &HashMap<String, HashMap<String, HashMap<String, MetaDataValue>>>,
    meta_data: &HashMap<String, HashMap<String, MetaDataValue>>,
    canonical_input_dir: &Path,
    import_pair: pest::iterators::Pair<Rule>,
    verbose: bool
) {
    let inner_pairs = import_pair.into_inner();

    for pair in inner_pairs.clone() {
        match pair.as_rule() {
            Rule::import_path => {
                let relative_path: Vec<&str> = pair
                    .as_str()
                    .trim()
                    .split("::")
                    .filter(|&s| s != "*")
                    .collect();

                let mut path = canonical_input_dir.join(relative_path.join("/"));
                if !path.to_string_lossy().ends_with(".rcss") {
                    path.set_extension("rcss");
                }

                println!("{:?}", path);
            }
            _ => {}
        }
    }
}
