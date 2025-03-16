mod ccss {
    pub mod compiler;
}

use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::io::Write;
use std::collections::HashMap;
use std::path::PathBuf;
use clap::{ Arg, Command };

use ccss::compiler::{ process_rule, process_variable };

#[derive(Parser)]
#[grammar = "ccss.pest"]
pub struct CCSSParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define and parse command line arguments
    let matches = Command::new("CCSS Compiler")
        .version("1.0.0")
        .about("Compiles CCSS files to CSS")
        .arg(Arg::new("input").help("Input CCSS file to process").required(true).index(1))
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output CSS file (defaults to input filename with .css extension)")
                .value_name("FILE")
        )
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

    let output_path = match matches.get_one::<String>("output") {
        Some(path) => path.to_string(),
        _ => {
            // Default output: replace .ccss with .css or add .css extension
            let input_pathbuf = PathBuf::from(input_path);
            let stem = input_pathbuf.file_stem().unwrap_or_default().to_string_lossy();
            let parent = input_pathbuf
                .parent()
                .unwrap_or_else(|| std::path::Path::new(""))
                .to_string_lossy();
            let parent_str = if parent.is_empty() {
                "".to_string()
            } else {
                format!("{}/", parent)
            };
            format!("{}{}.css", parent_str, stem)
        }
    };

    let verbose = matches.get_flag("verbose");
    let human_readable = !matches.get_flag("minify");

    // Read the CCSS file
    if verbose {
        println!("Reading from {}", input_path);
    }
    let unparsed_css = fs::read_to_string(input_path)?;

    // Parse the CCSS content
    let pairs = CCSSParser::parse(Rule::css, &unparsed_css)?;

    let mut css_output = String::new();
    let mut variables = HashMap::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::variable_declaration => {
                let var = process_variable(pair);

                if let Some((name, value)) = var {
                    if verbose {
                        println!("Found variable: {} = {}", name, value);
                    }
                    variables.insert(name, value);
                }
            }

            Rule::rule_normal => {
                let rule_css = process_rule(pair, human_readable);
                css_output.push_str(&rule_css);
            }

            Rule::EOI => {}

            _ => {
                if verbose {
                    println!("Unhandled rule: {:?}", pair.as_rule());
                }
            }
        }
    }

    for (name, value) in variables {
        css_output = css_output.replace(&("$".to_string() + &name), &value);
    }

    let mut output_file = fs::File::create(&output_path)?;
    output_file.write_all(css_output.as_bytes())?;

    println!("CSS written to {}", output_path);

    Ok(())
}
