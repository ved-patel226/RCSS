mod rcss {
    pub mod compiler;
    pub mod errors;

    pub mod process_x {
        pub mod function;
        pub mod media_query;
        pub mod variable;
        pub mod keyframes;
        pub mod import;
    }
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
use std::time::Instant;
use walkdir::WalkDir;

use rcss::{
    errors::{ RCSSError, display_error },
    process_x::{
        function::{ process_function_definition, process_function_call },
        media_query::process_media_query,
        variable::process_variable,
        keyframes::process_keyframes,
        import::process_import,
    },
    compiler::{ process_rule, MetaDataValue, Variables },
};

#[derive(Parser)]
#[grammar = "rcss.pest"]
pub struct RCSSParser;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("RCSS")
        .version("0.1.1")
        .about("Bringing Rust to CSS")
        .long_about(
            "For more information and to contribute, visit: https://github.com/ved-patel226/RCSS"
        )
        .arg(Arg::new("folder").help("Input directory to watch").required(true).index(1))
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Print verbose processing information")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    // Get input and output file paths
    let input_path = matches.get_one::<String>("folder").unwrap();
    let input_dir = Path::new(input_path);
    // Create canonical input path for consistent comparison
    let canonical_input_dir = input_dir.canonicalize()?;

    // Create the base CSS output directory adjacent to the input directory
    let css_output_dir = canonical_input_dir.parent().unwrap_or(&canonical_input_dir).join("css");

    let verbose = matches.get_flag("verbose");
    let human_readable = true;

    println!("Reading from {}", input_path);

    // File Name -> Meta Data
    let mut meta_data_to_file: HashMap<
        String,
        HashMap<String, HashMap<String, MetaDataValue>>
    > = HashMap::new();

    let mut rcss_files = Vec::new();

    for entry in WalkDir::new(input_path)
        .into_iter()
        .filter_map(|e| e.ok()) {
        if
            entry
                .path()
                .extension()
                .and_then(|s| s.to_str()) == Some("rcss")
        {
            rcss_files.push(entry.path().to_path_buf());
        }
    }

    // Compile all detected RCSS files on startup
    println!("Found {} RCSS files, compiling...", rcss_files.len());
    for file_path in &rcss_files {
        let canonical_file = file_path.canonicalize()?;

        // Get the relative path from the input directory to this file
        let relative_path = pathdiff
            ::diff_paths(&canonical_file, &canonical_input_dir)
            .unwrap_or_else(||
                canonical_file
                    .strip_prefix(&canonical_input_dir)
                    .unwrap_or(Path::new(""))
                    .to_path_buf()
            );

        let filename_stem = file_path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("default");

        // Create output directory with the same structure as input
        let mut output_file_path = css_output_dir.clone();
        if let Some(parent) = relative_path.parent() {
            output_file_path.push(parent);
        }

        // Create directories if they don't exist
        fs::create_dir_all(&output_file_path)?;

        // Add filename to the path
        output_file_path.push(filename_stem);
        output_file_path.set_extension("css");

        let res = compile(
            &file_path.display().to_string(),
            &output_file_path.to_string_lossy(),
            &canonical_input_dir,
            &meta_data_to_file,
            verbose,
            human_readable,
            true
        );

        if let Ok(meta_data) = res {
            if let Ok(canonical_path) = file_path.canonicalize() {
                meta_data_to_file.insert(canonical_path.display().to_string(), meta_data);
            }
        }
    }

    println!("Initial compilation complete. Watching for changes...");

    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = recommended_watcher(tx)?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new(input_path), RecursiveMode::Recursive)?;

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

                        let canonical_file = path.paths[0].canonicalize()?;

                        // Get the relative path from the input directory to this file
                        let relative_path = pathdiff
                            ::diff_paths(&canonical_file, &canonical_input_dir)
                            .unwrap_or_else(||
                                canonical_file
                                    .strip_prefix(&canonical_input_dir)
                                    .unwrap_or(Path::new(""))
                                    .to_path_buf()
                            );

                        // Create output directory with the same structure as input
                        let mut output_file_path = css_output_dir.clone();
                        if let Some(parent) = relative_path.parent() {
                            output_file_path.push(parent);
                        }

                        // Create directories if they don't exist
                        fs::create_dir_all(&output_file_path)?;

                        // Add filename to the path
                        output_file_path.push(filename_stem);
                        output_file_path.set_extension("css");

                        let res = compile(
                            &path.paths[0].display().to_string(),
                            &output_file_path.to_string_lossy(),
                            &canonical_input_dir,
                            &meta_data_to_file,
                            verbose,
                            human_readable,
                            false
                        );

                        if let Ok(meta_data) = res {
                            meta_data_to_file.insert(
                                canonical_file.display().to_string(),
                                meta_data
                            );
                        }
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
    canonical_input_dir: &Path,
    meta_data_to_file: &HashMap<String, HashMap<String, HashMap<String, MetaDataValue>>>,
    verbose: bool,
    human_readable: bool,
    initial_compile: bool
) -> std::result::Result<
    HashMap<String, HashMap<String, MetaDataValue>>,
    Box<dyn std::error::Error>
> {
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

    let pairs = match RCSSParser::parse(Rule::rcss, &unparsed_css) {
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
    // let mut variables = HashMap::new();

    let mut meta_data: HashMap<String, HashMap<String, MetaDataValue>> = HashMap::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::import_statement => {
                if initial_compile {
                    continue;
                }

                let res = process_import(
                    &meta_data_to_file,
                    &mut meta_data,
                    canonical_input_dir,
                    &pair,
                    verbose
                )
                    .map_err(|_| {
                        let pair_str = pair.as_str().to_string(); // Clone the string before the move
                        RCSSError::ImportError {
                            path: pair_str,
                            message: "Couldn't find file".to_string(),
                        }
                    })
                    .map_err(|e| {
                        display_error(&e);
                        e
                    });

                if let Ok(meta) = res {
                    meta_data = meta.clone();
                }
            }

            Rule::variable_declaration => {
                let var = process_variable(pair);

                if let Some((name, value)) = var {
                    if verbose {
                        println!("{} {} = \"{}\"", "Variable:".blue().bold(), name, value);
                    }
                    // variables.insert(name, value);
                    meta_data
                        .entry("variables".to_string())
                        .or_insert_with(HashMap::new)
                        .insert(name.clone(), MetaDataValue::Variables(Variables { name, value }));
                }
            }

            Rule::function_definition => {
                let function_def = process_function_definition(pair).ok_or_else(||
                    Box::<dyn std::error::Error>::from("Failed to process function definition")
                )?;

                if verbose {
                    println!("{} {}()", "Function Definition:".blue().bold(), function_def.name);
                }

                meta_data
                    .entry("functions".to_string())
                    .or_insert_with(HashMap::new)
                    .insert(function_def.name.clone(), MetaDataValue::Function(function_def));
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

    if let Some(variables) = meta_data.get("variables") {
        for (name, value) in variables {
            if let MetaDataValue::Variables(var) = value {
                css_output = css_output.replace(&("&".to_string() + &name), &var.value);
            }
        }
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
            return Err(
                Box::new(
                    std::io::Error::new(std::io::ErrorKind::Other, "Undeclared variable error")
                )
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

    Ok(meta_data)
}
