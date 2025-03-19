use std::error::Error;
use std::fmt::Write;

use crate::{ RCSSParser, Rule };
use pest::Parser;
use pest::iterators::Pair;

pub fn format_rcss(input: &str) -> Result<String, Box<dyn Error>> {
    // Parse the input to validate it
    let pairs = match RCSSParser::parse(Rule::css, input) {
        Ok(p) => p,
        Err(e) => {
            return Err(e.to_string().into());
        }
    };

    let mut output = String::new();
    let indent_level = 0;
    let mut last_was_variable = false;

    for pair in pairs {
        match pair.as_rule() {
            Rule::variable_declaration => {
                if last_was_variable {
                    format_variable(&mut output, pair)?;
                    writeln!(output)?;
                } else {
                    format_variable(&mut output, pair)?;
                    writeln!(output)?;
                }
                last_was_variable = true;
            }
            Rule::rule_normal => {
                if last_was_variable {
                    writeln!(output)?;
                }
                format_rule(&mut output, pair, indent_level)?;
                writeln!(output)?;
                last_was_variable = false;
            }
            Rule::EOI => {}
            _ => {
                writeln!(output, "/* Unhandled rule: {:?} */", pair.as_rule())?;
                last_was_variable = false;
            }
        }
    }

    println!("{}", "-".repeat(50));
    println!("{}", output);

    Ok(output)
}

fn format_variable(output: &mut String, pair: Pair<Rule>) -> Result<(), Box<dyn Error>> {
    let mut inner_pairs = pair.into_inner();
    let name = inner_pairs.next().unwrap().as_str();
    let value = inner_pairs.next().unwrap().as_str();

    write!(output, "let {}: {};", name, value)?;

    Ok(())
}

fn format_rule(
    output: &mut String,
    pair: Pair<Rule>,
    indent_level: usize
) -> Result<(), Box<dyn Error>> {
    let indent = "    ".repeat(indent_level);
    let mut inner_pairs = pair.into_inner();

    // Get selector
    let selector = inner_pairs.next().unwrap().as_str().trim();
    writeln!(output, "{}{} {{", indent, selector)?;

    // Process declarations and nested rules
    for inner_pair in inner_pairs {
        match inner_pair.as_rule() {
            Rule::declaration => {
                let mut decl_pairs = inner_pair.into_inner();
                let property = decl_pairs.next().unwrap().as_str().trim();
                let value = decl_pairs.next().unwrap().as_str().trim();
                writeln!(output, "{}    {}: {};", indent, property, value)?;
            }
            Rule::selector => {
                format_rule(output, inner_pair, indent_level + 1)?;
            }
            _ => {
                writeln!(output, "{}    /* Unhandled rule: {:?} */", indent, inner_pair.as_rule())?;
            }
        }
    }

    writeln!(output, "{}}}", indent)?;

    Ok(())
}
