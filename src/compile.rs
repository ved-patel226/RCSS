use std::fs;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::time::Instant;
use colored::*;
use chrono::Local;

use crate::{ error::{ RCSSError, display_error }, Result };

use crate::{ rule_normal, variables, functions, keyframes, imports, media_queries, MetaData };

#[derive(Parser)]
#[grammar = "rcss.pest"]
pub struct RCSSParser;

#[allow(dead_code)]
pub fn print_rule(pair: pest::iterators::Pair<Rule>) {
    println!("{:?} -> {}", pair.as_rule(), pair.as_str());
}

#[allow(unused)]
pub fn compile(
    input_path: &str,
    output_path: &str,
    relative_path: &str,
    project_meta_data: &mut HashMap<String, Vec<MetaData>>,
    verbose: bool,
    initial_compile: bool
) -> Result<HashMap<String, Vec<MetaData>>> {
    let start_time = Instant::now();

    let raw_rcss = fs::read_to_string(input_path)?;

    let pairs = match RCSSParser::parse(Rule::rcss, &raw_rcss) {
        Ok(p) => p,
        Err(e) => {
            // Extract location information from pest error
            let (line, column) = match e.line_col {
                pest::error::LineColLocation::Pos((line, col)) => (line, col),
                pest::error::LineColLocation::Span((line, col), _) => (line, col),
            };

            // Get a few lines around the error for context
            let lines: Vec<&str> = raw_rcss.lines().collect();
            let start = line.saturating_sub(2);
            let end = (line + 1).min(lines.len());
            let context = lines[start..end].join("\n");

            let err = RCSSError::ParseError {
                line,
                column,
                message: format!("{}", e),
                file_path: input_path.into(),
                context: context,
            };

            display_error(&err);

            return Err(err);
        }
    };

    let mut css_output = String::new();
    let mut meta_data: Vec<MetaData> = Vec::new();
    let mut declarations: HashMap<String, Vec<String>> = HashMap::new();
    let mut keyframes: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
    let mut one_liners: Vec<String> = Vec::new();
    let mut media_queries: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::import_statement => {
                // we don't want to import anything on initial check
                // we just want to fill project_meta_data
                if initial_compile {
                    continue;
                }

                meta_data = imports::process_import_statement(
                    &mut meta_data,
                    project_meta_data,
                    &raw_rcss,
                    input_path,
                    relative_path,
                    pair
                )?;
            }

            Rule::variable_declaration => {
                meta_data = variables::process_variable_declaration(meta_data, pair);
            }

            Rule::function_definition => {
                meta_data = functions::process_function_definition(
                    meta_data,
                    pair,
                    &raw_rcss,
                    &input_path,
                    initial_compile
                )?;
            }

            Rule::rule_normal => {
                // we don't want to import anything on initial check
                // we just want to fill project_meta_data
                if initial_compile {
                    continue;
                }

                declarations = rule_normal::process_rule_normal(
                    meta_data.clone(),
                    declarations,
                    pair,
                    &raw_rcss,
                    &input_path
                )?;
            }

            Rule::keyframes_rule => {
                if initial_compile {
                    continue;
                }

                keyframes = keyframes::process_keyframes_definition(
                    keyframes,
                    pair,
                    &meta_data,
                    &raw_rcss,
                    input_path
                )?;
            }

            Rule::rule_comment => {}

            Rule::EOI => {}

            Rule::at_methods_oneliner => {
                one_liners.push(pair.as_str().trim().to_string());
            }

            Rule::media_query => {
                if initial_compile {
                    continue;
                }

                media_queries = media_queries::process_media_query(
                    media_queries,
                    pair,
                    &meta_data,
                    &raw_rcss,
                    input_path
                )?;
            }

            _ => {
                // println!("{:?} -> {}", pair.as_rule(), pair.as_str());
            }
        }
    }

    project_meta_data.insert(input_path.to_string(), meta_data.clone());

    let now = Local::now();
    let formatted_time = now.format("%I:%M:%S %p");

    let elapsed_time = start_time.elapsed();

    if initial_compile {
        return Ok(project_meta_data.clone());
    }

    let css_output = css_map_to_string(&declarations, &keyframes, &one_liners, &media_queries);

    // Create folders
    if let Some(parent) = std::path::Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output_path, css_output)?;

    println!(
        "{} {} {}",
        format!("CSS written to {}", output_path).green(),
        format!("in {:.2?}", elapsed_time).truecolor(128, 128, 128),
        format!("@ {}", formatted_time).truecolor(128, 128, 128)
    );

    Ok(project_meta_data.clone())
}

fn css_map_to_string(
    css_map: &HashMap<String, Vec<String>>,
    keyframes: &HashMap<String, HashMap<String, Vec<String>>>,
    one_liners: &Vec<String>,
    media_queries: &HashMap<String, HashMap<String, Vec<String>>>
) -> String {
    let mut css_string = String::new();

    // Process regular CSS rules
    let mut sorted_css_map: Vec<_> = css_map.iter().collect();
    sorted_css_map.sort_by_key(|(selector, _)| *selector);

    for one_liner in one_liners {
        css_string.push_str(one_liner);
        css_string.push_str("\n");
    }

    if !one_liners.is_empty() {
        css_string.push_str("\n");
    }

    for (selector, properties) in sorted_css_map {
        css_string.push_str(selector);
        css_string.push_str(" {\n");

        let mut sorted_properties = properties.clone();
        sorted_properties.sort();

        for property in sorted_properties {
            css_string.push_str("    ");
            css_string.push_str(&property);
            css_string.push('\n');
        }

        css_string.push_str("}\n\n");
    }

    // Process at-methods like @keyframes
    let mut keyframes: Vec<_> = keyframes.iter().collect();
    keyframes.sort_by_key(|(at_rule, _)| *at_rule);

    for (at_rule, keyframes) in keyframes {
        css_string.push_str(at_rule);
        css_string.push_str(" {\n");

        let mut sorted_keyframes: Vec<_> = keyframes.iter().collect();
        sorted_keyframes.sort_by_key(|(keyframe_selector, _)| *keyframe_selector);

        for (keyframe_selector, properties) in sorted_keyframes {
            css_string.push_str("    ");
            css_string.push_str(keyframe_selector);
            css_string.push_str(" {\n");

            let mut sorted_properties = properties.clone();
            sorted_properties.sort();

            for property in sorted_properties {
                css_string.push_str("        ");
                css_string.push_str(&property);
                css_string.push('\n');
            }

            css_string.push_str("    }\n\n");
        }

        css_string.push_str("}\n\n");
    }

    for (condition, property_value) in media_queries {
        css_string.push_str(&format!("{} {{\n", condition.trim()));

        for (property, values) in property_value {
            css_string.push_str(&format!("    {} {{\n", property.trim()));

            for value in values {
                css_string.push_str("        ");
                css_string.push_str(&value);
                css_string.push_str("\n");
            }

            css_string.push_str("    }\n");
        }

        css_string.push_str("}\n");
    }

    css_string
}
