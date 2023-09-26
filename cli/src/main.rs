extern crate pta_parser;

// TODO: how to isolate pest so clients can just use lib (w/o requiring pest as here)
use pest::*;
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

            match LedgerParser::parse(Rule::ledger, &ledger) {
                Ok(pairs) => {
                    println!("LedgerParser produced {} pairs", pairs.len());
                    let mut t = pairs.tokens();
                    while let val = t.next() {
                        match val {
                            Some(val) => {
                                println!("Token: {:?}", val);
                            }

                            None => { break; }
                        }
                    }
                }

                Err(e) => {
                    println!("ERR: {}", e);
                    return Err(Box::new(e));
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



#[cfg(test)]
mod cli_tests {

    use pta_parser::LedgerParser;

    #[test]
    fn can_create_parser() {
        // simply verifies that the parser can be instantiated, ensuring accessibility
        let _ = LedgerParser{};
    }
}