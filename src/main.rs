mod rcss {
    pub mod compiler;
    pub mod errors;
}

use notify::event::{ AccessKind, AccessMode };
use notify::{ recommended_watcher, Event, RecursiveMode, Result, Watcher, EventKind };
use std::sync::mpsc;

use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use std::path::Path;
use clap::{ Arg, Command };
use regex::Regex;
use std::path::{ Component, PathBuf };
use colored::*;
use chrono::Local;

use rcss::{
    compiler::{ process_rule, process_variable, process_media_query, process_function_definition },
    errors::{ RCSSError, display_error },
};

#[derive(Parser)]
#[grammar = "rcss.pest"]
pub struct RCSSParser;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Define and parse command line arguments
    let matches = Command::new("RCSS Compiler")
        .version("1.0.0")
        .about("Compiles RCCS files to CSS")
        .arg(Arg::new("input").help("Input directory to process").required(true).index(1))
        .arg(
            Arg::new("minify")
                .short('m')
                .long("minify")
                .help("Minify the output CSS")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print verbose processing information")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Get input and output file paths
    let input_path = matches.get_one::<String>("input").unwrap();

    let verbose = matches.get_flag("verbose");
    let human_readable = !matches.get_flag("minify");

    if verbose {
        println!("Reading from {}", input_path);
    }

    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = recommended_watcher(tx)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new(input_path), RecursiveMode::Recursive)?;
    // Block forever, printing out events as they come in

    for res in rx {
        match res {
            Ok(path) => {
                // if file is written to
                if let EventKind::Access(AccessKind::Close(AccessMode::Write)) = path.kind {
                    if path.paths[0].extension().and_then(|s| s.to_str()) == Some("rcss") {
                        let filename_stem = path.paths[0]
                            .file_stem()
                            .and_then(|stem| stem.to_str())
                            .unwrap_or("default");

                        // If you need this path relative to the original file's directory:
                        let relative_css_path = path.paths[0]
                            .parent()
                            .unwrap_or(Path::new("."))
                            .join("../css")
                            .join(filename_stem);

                        let _ = compile(
                            &path.paths[0].display().to_string(),
                            &(relative_css_path.to_str().unwrap().to_string() + ".css"),
                            verbose,
                            human_readable
                        );
                    }
                }
            }

            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn compile(
    input_path: &str,
    output_path: &str,
    verbose: bool,
    human_readable: bool
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    let mut functions = HashMap::new();

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
                        println!("{} {}()", "Definition:".blue().bold(), function.name);
                    }
                    functions.insert(function.name.clone(), function);
                }
            }

            Rule::rule_normal => {
                let rule_css = process_rule(pair, &functions, human_readable, verbose);
                css_output.push_str(&rule_css);
            }

            Rule::media_query => {
                let media_css = process_media_query(pair, &functions, verbose, human_readable);
                css_output.push_str(&media_css);
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

        println!(
            "{} {}",
            format!("CSS written to {}", simplified_path.display()).bright_cyan().bold(),
            format!("@ {}", formatted_time).bright_yellow().bold()
        );
    }

    Ok(())
}
