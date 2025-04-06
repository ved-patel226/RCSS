use pest::iterators::Pair;
use crate::compile::{ Rule, print_rule };

fn process_variable_declaration(pair: Pair<Rule>) {
    let inner_pairs = pair.into_inner();

    for in_pair in inner_pairs {
        print_rule(in_pair);
    }
}
