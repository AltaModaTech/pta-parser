#[cfg(test)] use super::*;
#[cfg(test)] use rstest::rstest;



#[cfg(test)]
mod posting {
    use super::*;

    #[rstest]
    #[case ("  Assets:subacct1    1.0000")]
    #[case ("\tEquity   \t -1.0000")]
    fn can_parse_posting_basic(#[case] base: &str) {

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

            assert!(get_pairs(Rule::posting_basic, &tc).len() > 0);
        }
    }


    #[rstest]
    #[case ("   Assets:subacct1    1.0000")]    // Too many leading spaces or tabs
    #[case (" \tEquity   \t -1.0000")]
    #[case ("\t Equity   \t -1.0000")]

    #[should_panic(expected = "expected posting_basic")] // matches errors from multiple iso8601 rules
    fn verify_posting_basic_error(#[case] bad_date: &str) {

        LedgerParser::parse(
            Rule::posting_basic, bad_date)
            .unwrap_or_else(|e| panic!("{}", e));

        // should never reach this code since all cases should result in panic
        println!("Test case '{}' should fail to parse!", bad_date);
        assert!(false);
    }
}


#[cfg(test)]
mod trans_block {
    use super::*;


    // An example beancount transaction
    // 2016-01-28 * " Buy BTC"     ; 10:01 am, xfer id 56aa57787199a73d29000650
    //   Assets:Exchanges:Coinbase                     1.03683606 BTC { 381.9697397 USD, 2016-01-28 }
    //   Assets:Bank:AllyChk                        -400.00 USD   ; verified w/register
    //   Liabilities:Fees:Coinbase                     3.96 USD
    //   Liabilities:Fees:Adjustment                   0.00000005 USD
    
    #[rstest]
    // #[ignore = "wip"]
    #[case ("2009-01-09 ! \"Bitcoin launch date\" ;comment \n\tAssets    1.0000 ;posting comment\n\tEquity    -1.0000 \n")]
    #[case ("2009-01-09 ! \"Bitcoin launch date\"\n\tassets    1.0000\n  equity    -1.0000\n")]
    fn can_parse_trans_block(#[case] tblock: &str) {

        let pairs = LedgerParser::parse(
            Rule::transaction_block, &tblock)
            .unwrap_or_else(|e| panic!("{}", e));

        // Parsing succeeded; ensure at least 1 pair was returned
        assert!(pairs.len() > 0);
    }

    #[rstest]
    // #[ignore = "wip"]
    #[case ("2009-01-09 ! \"Bitcoin launch date\"
    ")]
    #[should_panic(expected = "expected transaction_block")]
    fn verify_trans_block_posting_error(#[case] bad_block: &str) {
        LedgerParser::parse(
            Rule::transaction_block, &bad_block)
            .unwrap_or_else(|e| panic!("{}", e));

        // should never reach this code since all cases should result in panic
        println!("Test case '{}' should fail to parse!", bad_block);
        assert!(false);
    }

    // REVIEW: Are these cases duplicative of trans_header tests?
    #[rstest]
    #[ignore = "wip"]
    #[case ("2009-01-09 ! \"Bitcoin launch date\"")]
    #[should_panic(expected = "expected trans_header")]
    fn verify_trans_block_trans_header_error(#[case] bad_block: &str) {
        LedgerParser::parse(
            Rule::transaction_block, &bad_block)
            .unwrap_or_else(|e| panic!("{}", e));

        // should never reach this code since all cases should result in panic
        println!("Test case '{}' should fail to parse!", bad_block);
        assert!(false);
    }

}


//===========
// NOTE: The tests in trans_header can be used by removing the silent indicator ('_') from the relevant pest rules.
//  These rules were silenced to simplify processing in code (matching, etc.), but can be un-silenced for debugging, etc.
//===========

// #[cfg(test)]
// mod trans_header {
//     use super::*;


//     #[rstest]
//     // NOTE: use simple text in case; test function wraps in dbl quotes
//     #[case ("a")]
//     #[case ("description")]
//     #[case (" a description ")]
//     #[case ("\ta description\twith tabs ")]
//     fn can_parse_trans_descr(#[case] descr: &str) {

//         let quoted_descr = format!("\"{}\"", descr);
//         let pairs = LedgerParser::parse(
//             Rule::trans_description, &quoted_descr)
//             .unwrap_or_else(|e| panic!("{}", e));

//         // Parsing succeeded; ensure at least 1 pair was returned
//         assert!(pairs.len() > 0);
//     }


//     #[rstest]
//     // NOTE: use simple text in case; test function wraps in dbl quotes
//     #[case ("")]    // empty - no text
//     #[case ("  ")]  // empty - only ws
//     #[case ("\ta description\twith tabs and\n a newline")]  // newline is invalid
//     #[should_panic(expected = "expected trans_")]
//     fn verify_trans_descr_error(#[case] bad_descr: &str) {

//         let quoted_bad_descr = format!("\"{}\"", bad_descr);
//         LedgerParser::parse(
//             Rule::trans_description, &quoted_bad_descr)
//             .unwrap_or_else(|e| panic!("{}", e));

//         // should never reach this code since all cases should result in panic
//         println!("Test case '{}' should fail to parse!", quoted_bad_descr);
//         assert!(false);
//     }



//     #[rstest]
//     // Verify transaction annotations: !, *, txn
//     #[case ("2009-01-09 ! \"Bitcoin launch date\"")]
//     #[case ("2009-01-09 * \"Bitcoin launch date\"")]
//     #[case ("2009-01-09 txn \"Bitcoin launch date\"")]
//     // whitespace variations
//     #[case ("2010-01-09  *  \"multi whitespace test\"")]
//     #[case ("2011-01-09\t!\t\"tab test\"")]
//     #[case ("2011-01-09\ttxn\t\"tab test\"")]
//     #[case ("2012-01-09 * \"trailing tab test\"\t")]
//     #[case ("2013-01-09 ! \"trailing spaces test\"  ")]
//     #[case ("2014-01-09 ! \"trailing tabs and spaces test\" \t \t\t  ")]
//     // #[ignore = "TBD: handle special chars in transaction description"]
//     // #[case ("2009-01-09 ! \"Special chars in description: !@#$%^&*()-_=+\"")]
//     fn can_parse_trans_header(#[case] base: &str) {

//         // NOTE: addons must end in \n to match rules
//         let addons = [
//             "\n"
//             ," \n"
//             ,"\t\n"
//             ," ; comment 123 ; \n"
//             ,"\t;\tcomment 123 ;\t\n"
//         ];

//         for suffix in addons.iter() {

//             let tc = format!("{}{}", base, suffix);
//             println!("Test case: {}", tc);

//             assert!(get_pairs(Rule::trans_header, &tc).len() > 0);
//         }
        
//     }

//     #[rstest]
//     #[case ("2016-01-28 * \"comment after description w/o whitespace\"; 10:01 am, xfer id 56aa57787199a73d29000650\n")]
//     #[should_panic(expected = "expected trans_header")]
//     fn verify_trans_header_error(#[case] bad_hdr: &str) {

//         let quoted_bad_descr = format!("\"{}\"", bad_hdr);
//         LedgerParser::parse(
//             Rule::trans_header, &quoted_bad_descr)
//             .unwrap_or_else(|e| panic!("{}", e));

//         // should never reach this code since all cases should result in panic
//         println!("Test case '{}' should fail to parse!", quoted_bad_descr);
//         assert!(false);
//     }


// }
