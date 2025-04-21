use pest::iterators::Pair;
use crate::{
    compile::Rule,
    error::{ display_error, RCSSError, get_error_context },
    MetaData,
    Result,
};
use std::collections::HashMap;

pub fn process_import_statement(
    meta_data: &mut Vec<MetaData>,
    project_meta_data: &mut HashMap<String, Vec<MetaData>>,
    raw_rcss: &str,
    input_path: &str,
    relative_path: &str,
    pair: Pair<Rule>
) -> Result<Vec<MetaData>> {
    let inner_pairs = pair.clone().into_inner();
    let mut target_import_file: Vec<String> = Vec::new();

    for import_in_pair in inner_pairs {
        match import_in_pair.as_rule() {
            Rule::identifier => {
                target_import_file.push(import_in_pair.as_str().to_string());
            }

            _ => {}
        }
    }

    //TODO - redo this better.. it sucks rn
    let full_path = format!("{}/{}", relative_path, target_import_file.join("/")) + ".rcss";

    if let Some(imported_meta_data) = project_meta_data.get(&full_path) {
        meta_data.extend(imported_meta_data.clone());
    } else {
        let position = pair.line_col();
        let line = position.0;
        let column = position.1;
        let context = get_error_context(raw_rcss, line, 2);

        let err = RCSSError::ImportError {
            file_path: input_path.into(),
            line: line,
            column: column,
            message: "File not found".to_string(),
            context: context,
        };

        display_error(&err);
        return Err(err);
    }

    Ok(meta_data.clone())
}
