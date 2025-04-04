use crate::{ HashMap, MetaDataValue, Rule, Path };

#[allow(dead_code)]
pub fn process_import<'a>(
    meta_data_to_file: &'a HashMap<String, HashMap<String, HashMap<String, MetaDataValue>>>,
    meta_data: &'a mut HashMap<String, HashMap<String, MetaDataValue>>,
    canonical_input_dir: &'a Path,
    import_pair: &pest::iterators::Pair<Rule>,
    verbose: bool
) -> Result<&'a mut HashMap<String, HashMap<String, MetaDataValue>>, ()> {
    let inner_pairs = import_pair.clone().into_inner();

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
                println!("{:?}", meta_data_to_file);

                if
                    let Some(file_meta_data) = meta_data_to_file.get(
                        path.to_string_lossy().as_ref()
                    )
                {
                    for (key, value) in file_meta_data {
                        meta_data
                            .entry(key.clone())
                            .or_insert_with(HashMap::new)
                            .extend(value.iter().map(|(k, v)| (k.clone(), v.clone())));
                    }
                } else {
                    return Err(());
                }
            }
            _ => {}
        }
    }

    Ok(meta_data)
}
