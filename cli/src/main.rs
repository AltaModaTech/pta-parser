extern crate pta_ledger;
extern crate pta_parser;


use log::{info, warn, as_error, error};

// TODO: how to isolate pest so clients can just use lib (w/o requiring pest as here)
use pta_ledger::ledger_builder::LedgerBuilder;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: CLI improvements 
    //  - exec with path of file to parse
    //  - optionally output parse results (should be equivalent to input file)

    // TODO: consider flag to use init_timed to include time per line
    pretty_env_logger::init();

    let pb = std::env::current_dir()?;
    let p = pb.join("testdata/basic-ledger");

    info!("Input file: {:?}", p);

    let mut bldr = LedgerBuilder::default();
    match std::fs::read_to_string(p) {
        Ok(ledger) => {
            info!("String length from input: {}", ledger.len());
            match bldr.from_string(&ledger) {
                Ok(_parsed) => {
                    info!("Successfully parsed into ParsedLedger");
                    return Ok(());
                },

                Err(e) => {
                    error!("LedgerBuilder failed with {:}", e);
                    return Err(e);
                }
            }
        }

        Err(e) => {
            warn!(err = as_error!(e); "failed to read file as string");
            return Err(Box::new(e));
        }
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