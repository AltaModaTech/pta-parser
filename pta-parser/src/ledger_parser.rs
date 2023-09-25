// Copyright (C) 2023, AltaModa Technologies, LLC. All rights reserved.
//
// This project is licensed under the terms of the MIT license (cf. LICENSE file in root).
//


#[derive(Parser)]
#[grammar = "./grammars/ledger.pest"]
pub struct LedgerParser;


#[cfg(test)]
mod parser_tests {

    use super::*;
    use pest::{Parser, iterators::Pairs};
    use rstest::rstest;


    mod acct_desc {
        use super::*;

        #[rstest]
        #[case ("a")]
        #[case ("a1")]
        #[case ("a:a")]
        #[case ("a1:a")]
        #[case ("a1:a1")]
        #[case ("a:123")]       // subaccts beginning w/number
        #[case ("a1:sub:123")]
        #[case ("asset")]
        #[case ("asset:property")]
        #[case ("asset:property:real")]
        fn can_parse_acct_descriptor(#[case] acct_desc: &str) {

            let pairs = LedgerParser::parse(
                Rule::acct_descriptor, acct_desc)
                .unwrap_or_else(|e| panic!("{}", e));

            // Parsing succeeded; ensure at least 1 pair was returned
            assert!(pairs.len() > 0);
        }


        #[rstest]
        // NOTE: invalid lead char in first acct segment ("1bad") fails top_level_acct rule & is included in verify_top_level_acct_parsing_error cases.
        #[case ("a1:b@d")]
        #[case ("bad1:")] // invalid: ends with acct descriptor separator (colon)
        #[should_panic(expected = "expected acct_descriptor")]
        #[ignore = "unexpectedly parses without error"]
        fn verify_acct_descriptor_parsing_error(#[case] bad_acct_desc: &str) {

            LedgerParser::parse(
                Rule::acct_descriptor, bad_acct_desc)
                .unwrap_or_else(|e| panic!("{}", e));

            // should never reach this code since all cases should result in panic
            println!("Test case '{}' should fail to parse!", bad_acct_desc);
            assert!(false);
        }



        #[rstest]
        #[case ("1")]   // invalid: number as lead char
        #[case ("1b")]
        #[case ("1-b")] // invalid: non-alphanumeric char
        #[case ("1b-")]
        #[should_panic(expected = "expected top_level_acct")]
        fn verify_top_level_acct_parsing_error(#[case] bad_top_level_acct: &str) {

            LedgerParser::parse(
                Rule::top_level_acct, bad_top_level_acct)
                .unwrap_or_else(|e| panic!("{}", e));

            // should never reach this code since all cases should result in panic
            println!("Test case '{}' should fail to parse!", bad_top_level_acct);
            assert!(false);

        }
    }



    mod decimal {
        use super::*;


        #[rstest]
        #[case ("0.00000001")]
        #[case ("1.23")]
        #[case ("123.456")]
        #[case ("-123.456789012")]  // negative values
        #[case ("-0.00000001")]
        fn can_parse_decimal_value(#[case] dec: &str) {

            let pairs = LedgerParser::parse(
                Rule::decimal_value, dec)
                .unwrap_or_else(|e| panic!("{}", e));

            // Parsing succeeded; ensure at least 1 pair was returned
            assert!(pairs.len() > 0);
        }


        #[rstest]
        #[case ("0.")]      // fractional segment missing
        #[case ("-0.")]
        #[case ("123")]
        #[case ("-123")]
        #[case (".12")]     // whole segment missing
        #[case ("-.12")]
  
        #[should_panic(expected = "expected decimal_value")]
        fn verify_decimal_value_error(#[case] bad_dec: &str) {

            LedgerParser::parse(
                Rule::decimal_value, bad_dec)
                .unwrap_or_else(|e| panic!("{}", e));

            // should never reach this code since all cases should result in panic
            println!("Test case '{}' should fail to parse!", bad_dec);
            assert!(false);
        }

    }



    mod iso8601 {
        use super::*;

        #[rstest]
        #[case ("1900-01-01")]
        #[case ("2015-12-31")]
        fn can_parse_iso8601_date_extended(#[case] year: &str) {

            let pairs = LedgerParser::parse(
                Rule::iso8601_date_extended, year)
                .unwrap_or_else(|e| panic!("{}", e));

            // Parsing succeeded; ensure at least 1 pair was returned
            assert!(pairs.len() > 0);
        }


        #[rstest]
        #[case ("000-01-01")]   // Year out of range
        #[case ("99990-01-01")]
        #[case ("01-01")]       // year segment missing

        #[case ("1999")]        // month segment missing
        #[case ("1999-")]
        #[case ("0000-00-01")]  // Month out of range
        #[case ("0000-13-01")]

        #[case ("1999-12")]     // day segment missing
        #[case ("1999-12-")]
        #[case ("0000-01-00")]  // Day out of range
        #[case ("0000-01-32")]

        #[case ("000o-01-01")]  // Invalid chars
        #[case ("1999-0x-12")]
        #[case ("1999-12-0x")]

        #[case ("1999 12-01")]  // whitespace (ensure atomic rule modifier is used)
        #[case ("1999-12 01")]
        #[case (" 1999-12-01")] // leading space (reqs additional rule)
        #[should_panic(expected = "expected iso8601_")] // matches errors from multiple iso8601 rules
        fn verify_iso8601_date_extended_error(#[case] bad_date: &str) {

            LedgerParser::parse(
                Rule::iso8601_date_extended, bad_date)
                .unwrap_or_else(|e| panic!("{}", e));

            // should never reach this code since all cases should result in panic
            println!("Test case '{}' should fail to parse!", bad_date);
            assert!(false);
        }
    }


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


    mod trans_header {
        use super::*;


        #[rstest]
        // NOTE: use simple text in case; test function wraps in dbl quotes
        #[case ("a")]
        #[case ("description")]
        #[case (" a description ")]
        #[case ("\ta description\twith tabs ")]
        fn can_parse_trans_descr(#[case] descr: &str) {

            let quoted_descr = format!("\"{}\"", descr);
            let pairs = LedgerParser::parse(
                Rule::trans_description, &quoted_descr)
                .unwrap_or_else(|e| panic!("{}", e));

            // Parsing succeeded; ensure at least 1 pair was returned
            assert!(pairs.len() > 0);
        }


        #[rstest]
        // NOTE: use simple text in case; test function wraps in dbl quotes
        #[case ("")]    // empty - no text
        #[case ("  ")]  // empty - only ws
        #[case ("\ta description\twith tabs and\n a newline")]  // newline is invalid
        #[should_panic(expected = "expected trans_")]
        fn verify_trans_descr_error(#[case] bad_descr: &str) {

            let quoted_bad_descr = format!("\"{}\"", bad_descr);
            LedgerParser::parse(
                Rule::trans_description, &quoted_bad_descr)
                .unwrap_or_else(|e| panic!("{}", e));

            // should never reach this code since all cases should result in panic
            println!("Test case '{}' should fail to parse!", quoted_bad_descr);
            assert!(false);
        }



        #[rstest]
        #[case ("2009-01-09 ! \"Bitcoin launch date\"")]
        #[case ("2010-01-09  *  \"multi whitespace test\"")]
        #[case ("2011-01-09\t! \"tab test\"")]
        #[case ("2012-01-09 * \"trailing tab test\"\t")]
        #[case ("2013-01-09 ! \"trailing spaces test\"  ")]
        #[case ("2014-01-09 ! \"trailing tabs and spaces test\" \t \t\t  ")]
        // #[ignore = "TBD: handle special chars in transaction description"]
        // #[case ("2009-01-09 ! \"Special chars in description: !@#$%^&*()-_=+\"")]
        fn can_parse_trans_header(#[case] base: &str) {

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

                assert!(get_pairs(Rule::trans_header, &tc).len() > 0);
            }
            
        }

        #[rstest]
        #[case ("2016-01-28 * \"comment after description w/o whitespace\"; 10:01 am, xfer id 56aa57787199a73d29000650\n")]
        #[should_panic(expected = "expected trans_")]
        fn verify_trans_header_error(#[case] bad_hdr: &str) {

            let quoted_bad_descr = format!("\"{}\"", bad_hdr);
            LedgerParser::parse(
                Rule::trans_header, &quoted_bad_descr)
                .unwrap_or_else(|e| panic!("{}", e));

            // should never reach this code since all cases should result in panic
            println!("Test case '{}' should fail to parse!", quoted_bad_descr);
            assert!(false);
        }


    }


    mod trans_block {
        use super::*;


        // An example beancount transaction
        // 2016-01-28 * " Buy BTC"     ; 10:01 am, xfer id 56aa57787199a73d29000650
        //   Assets:Exchanges:Coinbase                     1.03683606 BTC { 381.9697397 USD, 2016-01-28 }
        //   Assets:Bank:AllyChk                        -400.00 USD   ; verified w/register
        //   Liabilities:Fees:Coinbase                     3.96 USD
        //   Liabilities:Fees:Adjustment                   0.00000005 USD
      
        #[rstest]
        #[ignore = "wip"]
        #[case ("2009-01-09 ! \"Bitcoin launch date\"
          assets:subacct1    1.0000
          equity    -1.0000
        ")]
        fn can_parse_trans_block(#[case] tblock: &str) {

            let quoted_descr = format!("\"{}\"", tblock);
            let pairs = LedgerParser::parse(
                Rule::trans_description, &quoted_descr)
                .unwrap_or_else(|e| panic!("{}", e));

            // Parsing succeeded; ensure at least 1 pair was returned
            assert!(pairs.len() > 0);
        }



    }



    mod directives {
        use super::*;

        // YYYY-MM-DD open Account [ConstraintCurrency,...] ["BookingMethod"]

        #[rstest]
        #[case (Rule::directive_open,  "2001-09-11 open assets")]
        #[case (Rule::directive_open,  "2001-09-11 open assets:cash")]
        #[case (Rule::directive_open,  "2001-09-11 open Assets1:cash2:3petty")]
        #[case (Rule::directive_close, "2001-09-11 close assets")]
        #[case (Rule::directive_close, "2001-09-11 close assets1:2cash:3petty")]
        #[case (Rule::directive_commodity, "2001-09-11 thing USD")]
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


    fn get_pairs(r: Rule, content: &str) -> Pairs<'_, Rule> {
        let x = LedgerParser::parse(
            r,
            
             content)
            .unwrap_or_else(|e| panic!("{}", e));

        return x;
    }


}
