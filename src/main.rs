use pest::Parser;
use pest_derive::Parser;
use std::fs;
use std::io::Write;

#[derive(Parser)]
#[grammar = "ccss.pest"]
pub struct CCSSParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the CSS file
    let css_file_path = "example.ccss";
    let unparsed_css = fs::read_to_string(css_file_path)?;

    // Parse the CSS content
    let pairs = CCSSParser::parse(Rule::css, &unparsed_css)?;

    let mut css_output = String::new();

    for pair in pairs {
        match pair.as_rule() {
            Rule::rule_normal => {
                let rule_css = process_rule(pair, true);
                css_output.push_str(&rule_css);
            }
            _ => {}
        }
    }

    let output_path = "output.css";
    let mut output_file = fs::File::create(output_path)?;
    output_file.write_all(css_output.as_bytes())?;

    println!("CSS written to {}", output_path);

    Ok(())
}

fn process_rule(rule_pair: pest::iterators::Pair<Rule>, human_readable: bool) -> String {
    let inner_pairs = rule_pair.into_inner();
    let mut previous_selector = String::new();
    let mut result = String::new();
    let newline = if human_readable { "\n" } else { "" };
    let space = if human_readable { " " } else { "" };
    let indent_size = 4;

    for pair in inner_pairs {
        match pair.as_rule() {
            Rule::selector => {
                let trimmed_selector = pair.as_str().trim();
                if previous_selector.is_empty() {
                    result.push_str(&format!("{}{}{{{}", trimmed_selector, space, newline));
                } else {
                    result.push_str(
                        &format!(
                            "}}{}{} {}{}{{{}",
                            space,
                            previous_selector.trim(),
                            trimmed_selector,
                            space,
                            newline
                        )
                    );
                }
                previous_selector.push_str(trimmed_selector);
                previous_selector.push_str(space);
            }
            Rule::declaration => {
                result.push_str(
                    &format!("{}{}{}", space.repeat(indent_size), pair.as_str().trim(), newline)
                );
            }
            _ => {}
        }
    }

    result.push_str("}");
    result
}
