use std::fs;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::time::Instant;
use colored::*;
use chrono::Local;

use crate::{ error::{ RCSSError, display_error }, Result };

use crate::{ rule_normal, variables, functions, keyframes, MetaData };

#[derive(Parser)]
#[grammar = "rcss.pest"]
pub struct RCSSParser;

pub fn print_rule(pair: pest::iterators::Pair<Rule>) {
    println!("{:?} -> {}", pair.as_rule(), pair.as_str());
}

#[allow(unused)]
pub fn compile(
    input_path: &str,
    output_path: &str,
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
    let mut at_methods: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::import_statement => {
                // we don't want to import anything on initial compile
                // we just want to fill project_meta_data
                if initial_compile {
                    continue;
                }

                print_rule(pair);
            }

            Rule::variable_declaration => {
                meta_data = variables::process_variable_declaration(meta_data, pair);
            }

            Rule::function_definition => {
                meta_data = functions::process_function_definition(
                    meta_data,
                    pair,
                    &raw_rcss,
                    &input_path
                )?;
            }

            Rule::rule_normal => {
                let (new_meta_data, new_declarations) = rule_normal::process_rule_normal(
                    meta_data.clone(),
                    declarations,
                    pair,
                    &raw_rcss,
                    &input_path
                )?;
                meta_data = new_meta_data;
                declarations = new_declarations;
            }

            Rule::keyframes_rule => {
                at_methods = keyframes::process_keyframes_definition(
                    at_methods,
                    pair,
                    &meta_data,
                    &raw_rcss,
                    input_path
                )?;
            }

            Rule::rule_comment => {}

            Rule::EOI => {}

            _ => {
                println!("{:?} -> {}", pair.as_rule(), pair.as_str());
            }
        }
    }

    project_meta_data.insert(input_path.to_string(), meta_data.clone());

    let css_output = css_map_to_string(&declarations, &at_methods);
    fs::write(output_path, css_output)?;

    let now = Local::now();
    let formatted_time = now.format("%I:%M:%S %p");

    let elapsed_time = start_time.elapsed();

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
    at_methods: &HashMap<String, HashMap<String, Vec<String>>>
) -> String {
    let mut css_string = String::new();

    // Process regular CSS rules
    for (selector, properties) in css_map {
        // Start building the CSS rule
        css_string.push_str(selector);
        css_string.push_str(" {\n");

        // Add each unique property
        for property in properties {
            css_string.push_str("    ");
            css_string.push_str(property);
            css_string.push('\n');
        }

        css_string.push_str("}\n\n");
    }

    // Process at-methods like @keyframes
    for (at_rule, keyframes) in at_methods {
        css_string.push_str(at_rule);
        css_string.push_str(" {\n");

        // Process each keyframe
        for (keyframe_selector, properties) in keyframes {
            css_string.push_str("    ");
            css_string.push_str(keyframe_selector);
            css_string.push_str(" {\n");

            // Add each property for this keyframe
            for property in properties {
                css_string.push_str("        ");
                css_string.push_str(property);
                css_string.push('\n');
            }

            css_string.push_str("    }\n\n");
        }

        css_string.push_str("}\n\n");
    }

    // Remove the last newline if the string is not empty
    if !css_string.is_empty() {
        css_string.pop();
        if !css_string.is_empty() {
            css_string.pop();
        }
    }

    css_string
}
