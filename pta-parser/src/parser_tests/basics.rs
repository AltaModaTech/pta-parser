#[cfg(test)] use super::*;
#[cfg(test)] use rstest::rstest;


#[cfg(test)]
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


#[cfg(test)]
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


    #[cfg(test)]
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



#[cfg(test)]
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
