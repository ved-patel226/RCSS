use std::fs;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use crate::MetaData;
use std::time::Instant;

use crate::{ error::{ RCSSError, display_error }, Result };

#[derive(Parser)]
#[grammar = "rcss.pest"]
pub struct RCSSParser;

pub fn compile(
    input_path: &str,
    output_path: &str,
    project_meta_data: &HashMap<String, Vec<MetaData>>,
    verbose: bool,
    initial_compile: bool
) -> Result<()> {
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

    Ok(())
}
