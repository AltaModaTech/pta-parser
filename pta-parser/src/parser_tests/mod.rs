
pub use super::*;
pub use pest::{Parser, iterators::Pairs};
#[cfg(test)] 
pub use rstest::rstest;


mod basics;
mod transaction;


#[cfg(test)]
mod directives {
    use super::*;

    // YYYY-MM-DD open Account [ConstraintCurrency,...] ["BookingMethod"]

    #[rstest]
    #[case (Rule::directive_open,  "2001-09-11 open assets")]
    #[case (Rule::directive_open,  "2001-09-11 open assets:cash")]
    #[case (Rule::directive_open,  "2001-09-11 open Assets1:cash2:3petty")]
    #[case (Rule::directive_close, "2001-09-11 close assets")]
    #[case (Rule::directive_close, "2001-09-11 close assets1:2cash:3petty")]
    #[case (Rule::directive_commodity, "2001-09-11 commodity USD")]
    #[case (Rule::balance_directive,   "2001-09-11 balance assets 123.456 USD")]
    #[case (Rule::balance_directive,   "2001-09-11 balance assets1:2cash -0.456 USD")]
    fn can_parse_misc_directive(#[case] r: Rule, #[case] base: &str) {

        // NOTE: addons must end in \n to match rules
        let addons = [
            "\n"
            ," \n"
            ,"\t\n"
            ," ; comment 123 ; \n"
            ,"\t;\tcomment 123 ;\t\n"
        ];

        for suffix in addons.iter() {

            let tc = format!("{}{}", base, suffix);
            println!("Test case: {}", tc);

            assert!(get_pairs(r, &tc).len() > 0);
        }
    }

}



#[cfg(test)]
mod ledger_file {
    use super::*;

    #[rstest]
    #[case (
    "; an asterisk-based comment
    * Accounts
    2001-09-11 open assets
    2001-09-11 open assets:cash\t;comment
    2001-09-12 close assets

    ;; Balance assertions
    2001-09-11 balance assets 123.456 USD


    ;; Misc
    1792-01-01 commodity USD ; US Dollar
    2001-09-11 commodity BTC ; Bitcoin launch date
    
    ")]
    fn can_parse_ledger(#[case] year: &str) {

        let pairs = LedgerParser::parse(
            Rule::ledger, year)
            .unwrap_or_else(|e| panic!("{}", e));

        // Parsing succeeded; ensure at least 1 pair was returned
        assert!(pairs.len() > 0);
    }
}





pub fn get_pairs(r: Rule, content: &str) -> Pairs<'_, Rule> {
    let x = LedgerParser::parse(
        r,
        
            content)
        .unwrap_or_else(|e| panic!("{}", e));

    return x;
}

