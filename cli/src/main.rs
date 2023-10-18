extern crate pta_parser;

// TODO: how to isolate pest so clients can just use lib (w/o requiring pest as here)
use pest::{*, iterators::Pair};
use pta_parser::{LedgerParser, Rule};

fn main() -> Result<(), Box<dyn std::error::Error>> {
        // TODO: CLI improvements 
    //  - exec with path of file to parse
    //  - optionally output parse results (should be equivalent to input file)

    let pb = std::env::current_dir()?;
    println!("Curr dir: {:?}", pb.as_path());

    let p = pb.join("testdata/basic-ledger");

    println!("Reading {:?}", p);

    match std::fs::read_to_string(p) {
        Ok(ledger) => {
            println!("Read string length: {}", ledger.len());
            return main_parse(&ledger);
        }

        Err(e) => {
            println!("ERR: {}", e);
            return Err(Box::new(e));
        }
    }

}



#[allow(dead_code)] // allows switching b/t mains in primary main above
fn main_parse(ledger: &String) -> Result<(), Box<dyn std::error::Error>> {

    match LedgerParser::parse(Rule::ledger, &ledger) {
        Ok(root) => {
            for pair in root.into_iter() {
                // println!("\n{:?}", pair.as_span());
                // println!("\n{:?}", pair.as_rule());

                match pair.as_rule() {
                    Rule::comment => {
                        dump_pair(&pair);
                    }

                    Rule::EOI => { dump_pair(&pair); }
                    Rule::WHITESPACE => { dump_pair(&pair); }
                    Rule::acct_descriptor => { dump_pair(&pair); }
                    Rule::acct_separator => { dump_pair(&pair); }
                    Rule::balance_directive => { dump_pair(&pair); }
                    Rule::comment_or_newline => { dump_pair(&pair); }
                    Rule::comment_token => { dump_pair(&pair); }
                    Rule::currency => { dump_pair(&pair); }
                    Rule::decimal_value => { dump_pair(&pair); }
                    Rule::directive_close => { dump_pair(&pair); }
                    Rule::directive_commodity => { dump_pair(&pair); }
                    Rule::directive_open => { dump_pair(&pair); }
                    Rule::directives => { dump_pair(&pair); }
                    Rule::empty_line => { dump_pair(&pair); }
                    Rule::iso8601_date_extended => { dump_pair(&pair); }
                    Rule::ledger => { dump_pair(&pair); }
                    Rule::options => { dump_pair(&pair); }
                    Rule::posting_basic => { dump_pair(&pair); }
                    Rule::posting_indent => { dump_pair(&pair); }
                    Rule::sub_acct => { dump_pair(&pair); }
                    Rule::top_level_acct => { dump_pair(&pair); }
                    Rule::trans_annotation => { dump_pair(&pair); }
                    Rule::trans_description => { dump_pair(&pair); }
                    Rule::trans_description_text => { dump_pair(&pair); }
                    Rule::trans_header => { dump_pair(&pair); }
                    Rule::transaction_block => { dump_pair(&pair); }
                }
            } 
        }

        Err(e) => {
            println!("ERR: {}", e);
            return Err(Box::new(e));
        }
    }

    return Ok(());
}


fn dump_rule(r:&Rule, s:&Span) {
    println!("\nRULE: {:?}", &r);
    println!("\n{:?}", &s);
}

fn dump_pair(p:&Pair<Rule>) {
    dump_rule(&p.as_rule(), &p.as_span());
}



fn handle_pair(pair: Pair<'_, Rule>) {
    match pair.as_rule() {
        Rule::comment => { dump_pair(&pair); }
        Rule::EOI => { dump_pair(&pair); }
        Rule::WHITESPACE => { dump_pair(&pair); }
        Rule::acct_descriptor => { dump_pair(&pair); }
        Rule::acct_separator => { dump_pair(&pair); }
        Rule::balance_directive => { dump_pair(&pair); }
        Rule::comment_or_newline => { dump_pair(&pair); }
        Rule::comment_token => { dump_pair(&pair); }
        Rule::currency => { dump_pair(&pair); }
        Rule::decimal_value => { dump_pair(&pair); }
        Rule::directive_close => { dump_pair(&pair); }
        Rule::directive_commodity => { dump_pair(&pair); }
        Rule::directive_open => { dump_pair(&pair); }
        Rule::directives => { dump_pair(&pair); }
        Rule::empty_line => { dump_pair(&pair); }
        Rule::iso8601_date_extended => { dump_pair(&pair); }
        Rule::ledger => { dump_pair(&pair); }
        Rule::options => { dump_pair(&pair); }
        Rule::posting_basic => { dump_pair(&pair); }
        Rule::posting_indent => { dump_pair(&pair); }
        Rule::sub_acct => { dump_pair(&pair); }
        Rule::top_level_acct => { dump_pair(&pair); }
        Rule::trans_annotation => { dump_pair(&pair); }
        Rule::trans_description => { dump_pair(&pair); }
        Rule::trans_description_text => { dump_pair(&pair); }
        Rule::trans_header => { dump_pair(&pair); }
        Rule::transaction_block => { dump_pair(&pair); }
    }
}



#[cfg(test)]
mod cli_tests {

    use pta_parser::LedgerParser;

    #[test]
    fn can_create_parser() {
        // simply verifies that the parser can be instantiated, ensuring accessibility
        let _ = LedgerParser{};
    }
}