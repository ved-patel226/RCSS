use std::fs;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use crate::MetaData;
use std::time::Instant;

use crate::{ error::{ RCSSError, display_error }, Result };

use crate::{ rule_normal, variables, functions };

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
    project_meta_data: &HashMap<String, Vec<MetaData>>,
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
                meta_data = functions::process_function_definition(meta_data, pair);
            }

            Rule::rule_normal => {
                meta_data = rule_normal::process_rule_normal(meta_data, pair)?;
            }

            Rule::rule_comment => {}

            Rule::EOI => {}

            _ => {
                println!("{:?} -> {}", pair.as_rule(), pair.as_str());
            }
        }
    }

    println!("{:?}", meta_data);

    Ok(project_meta_data.clone())
}
