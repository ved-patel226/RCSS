use std::error::Error;

use crate::{ RCSSParser, Rule };
use pest::Parser;

pub fn format_rcss(input: &str) -> Result<(), Box<dyn Error>> {
    // Parse the input to validate it
    let pairs = match RCSSParser::parse(Rule::css, input) {
        Ok(p) => p,
        Err(e) => {
            return Err(e.to_string().into());
        }
    };

    for pair in pairs {
        match pair.as_rule() {
            Rule::variable_declaration => {
                let mut inner_pairs = pair.clone().into_inner();
                let name = inner_pairs.next().unwrap().as_str();
                let value = inner_pairs.next().unwrap().as_str();
                println!("Variable: {} = {}", name, value);
                println!("{:?} - {}", pair.as_rule(), pair.as_str().to_string());
            }

            Rule::rule_normal => {
                println!("{:?} - {}", pair.as_rule(), pair.as_str().to_string().trim());
            }

            Rule::EOI => {}

            _ => {
                println!("Unhandled rule: {:?}", pair.as_rule());
            }
        }
    }

    Ok(())
}
