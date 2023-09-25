extern crate pta_parser;

use pta_parser::LedgerParser;

fn main() {
    // TODO: implement useful CLI, e.g., 
    //  - exec with path of file to parse
    //  - optionally output parse results (should be equivalent to input file)

    println!("\nNOTICE: This CLI is under development...\n");

    // instantiate parser to ensure expected accessibility
    let _ = LedgerParser {};
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