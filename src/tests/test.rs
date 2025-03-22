#[allow(unused_imports)]
use crate::{ rcss, RCSSParser, Rule };
#[allow(unused_imports)]
use notify::{ recommended_watcher, Event, RecursiveMode, Watcher };

#[allow(unused_imports)]
use pest::Parser;
#[allow(unused_imports)]
use pest_derive::Parser;
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::io::Write;
#[allow(unused_imports)]
use std::collections::HashMap;
#[allow(unused_imports)]
use std::path::Path;
#[allow(unused_imports)]
use clap::{ Arg, Command };
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::path::{ Component, PathBuf };
#[allow(unused_imports)]
use colored::*;
#[allow(unused_imports)]
use chrono::Local;
#[allow(unused_imports)]
use std::time::Instant;

#[allow(unused_imports)]
use rcss::{
    errors::{ RCSSError, display_error },
    process_x::{
        function::{ process_function_definition, process_function_call },
        media_query::process_media_query,
        variable::process_variable,
        keyframes::process_keyframes,
    },
    compiler::{ process_rule, MetaDataValue },
};

#[test]
fn website_tests() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let input_path = "src/tests/test.rcss";
    let output_path = "src/tests/output.css";
    let verbose = true;
    let human_readable = true;

    let start_time = Instant::now();

    let unparsed_css = fs::read_to_string(input_path).map_err(|e| {
        display_error(
            &(RCSSError::FileError {
                path: input_path.to_string(),
                message: format!("Failed to read file: {}", e),
            })
        );
        e
    })?;

    let pairs = match RCSSParser::parse(Rule::css, &unparsed_css) {
        Ok(p) => p,
        Err(e) => {
            // Extract location information from pest error
            let (line, column) = match e.line_col {
                pest::error::LineColLocation::Pos((line, col)) => (line, col),
                pest::error::LineColLocation::Span((line, col), _) => (line, col),
            };

            // Get a few lines around the error for context
            let lines: Vec<&str> = unparsed_css.lines().collect();
            let start = line.saturating_sub(2);
            let end = (line + 1).min(lines.len());
            let context = lines[start..end].join("\n");

            display_error(
                &(RCSSError::ParseError {
                    line,
                    column,
                    message: format!("{}", e),
                    snippet: context,
                })
            );

            return Err(Box::new(e));
        }
    };

    let mut css_output = String::new();
    let mut variables = HashMap::new();

    let mut meta_data: HashMap<String, HashMap<String, MetaDataValue>> = HashMap::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::variable_declaration => {
                let var = process_variable(pair);

                if let Some((name, value)) = var {
                    if verbose {
                        println!("{} {} = \"{}\"", "Variable:".blue().bold(), name, value);
                    }
                    variables.insert(name, value);
                }
            }

            Rule::function_definition => {
                if let Some(function) = process_function_definition(pair) {
                    if verbose {
                        if let MetaDataValue::Function(inner_function) = &function {
                            println!("{} {}()", "Definition:".blue().bold(), inner_function.name);
                        }
                    }
                    meta_data.entry("functions".to_string()).or_insert_with(HashMap::new);
                }
            }

            Rule::rule_normal => {
                let rule_css = process_rule(pair, &meta_data, human_readable, verbose);
                css_output.push_str(&rule_css);
            }

            Rule::media_query => {
                let media_css = process_media_query(pair, &meta_data, human_readable, verbose);
                css_output.push_str(&media_css);
            }

            Rule::keyframes_rule => {
                // let keyframes_css = process_media_query(pair, &meta_data, human_readable, verbose);
                let keyframes_css = process_keyframes(
                    pair,
                    &meta_data.entry("keyframes".to_string()).or_insert_with(HashMap::new),
                    human_readable,
                    verbose
                )?;

                css_output.push_str(&keyframes_css);
            }

            Rule::EOI => {}
            Rule::rule_comment => {}

            _ => {
                if verbose {
                    println!("{} {:?}", "Unhandled rule:".yellow().bold(), pair.as_rule());
                }
            }
        }
    }

    for (name, value) in variables {
        css_output = css_output.replace(&("&".to_string() + &name), &value);
    }

    let regex = Regex::new(r"\&([a-zA-Z][a-zA-Z0-9_\-]*)").unwrap();
    let mut undeclared_vars = Vec::new();

    for capture in regex.captures_iter(&css_output) {
        if let Some(var_name) = capture.get(1) {
            undeclared_vars.push(var_name.as_str().to_string());
        }
    }

    // Report any undeclared variables as warnings
    if !undeclared_vars.is_empty() {
        for var in undeclared_vars {
            display_error(
                &(RCSSError::VariableError {
                    name: var.clone(),
                    message: format!("Undeclared variable:"),
                })
            );
        }
    }

    fs::File
        ::create(&output_path)
        .and_then(|mut file| file.write_all(css_output.as_bytes()))
        .map_err(|e| {
            display_error(
                &(RCSSError::FileError {
                    path: output_path.to_string(),
                    message: format!("Failed to write file: {}", e),
                })
            );
            Box::new(e) as Box<dyn std::error::Error>
        })?;

    {
        let simplified_path = {
            let mut pb = PathBuf::new();
            for component in Path::new(output_path).components() {
                match component {
                    Component::ParentDir => {
                        pb.pop();
                    }
                    Component::CurDir => {
                        continue;
                    }
                    _ => pb.push(component.as_os_str()),
                }
            }
            pb
        };

        let now = Local::now();
        let formatted_time = now.format("%I:%M:%S %p");

        let elapsed_time = start_time.elapsed();

        println!(
            "{} {} {}",
            format!("CSS written to {}", simplified_path.display()).green(),
            format!("in {:.2?}", elapsed_time).truecolor(128, 128, 128),
            format!("@ {}", formatted_time).truecolor(128, 128, 128)
        );
    }

    Ok(())
}
