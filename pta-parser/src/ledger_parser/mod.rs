// Copyright (C) 2023, AltaModa Technologies, LLC. All rights reserved.
//
// This project is licensed under the terms of the MIT license (cf. LICENSE file in root).
//


use pest_derive::*;

#[derive(Parser)]
#[grammar = "./grammars/ledger.pest"]
pub struct LedgerParser;


#[derive(Default)]
struct RawAccountDescriptor {
    path: String
}


#[derive(Default)]
struct RawTransaction {
    date: String,
    anno: String,
    desc: String,
    postings: Vec<RawPosting>,
    comment: String
}

#[derive(Default)]
struct RawPosting {
    acct: RawAccountDescriptor,
    value: f64,
    comment: String
}


use pest_consume::{Error, match_nodes};
type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

#[pest_consume::parser]
impl LedgerParser {
    fn EOI(_input: Node) -> Result<()> {
        print!("EOI found");
        Ok(())
    }

    fn acct_descriptor(input: Node) -> Result<RawAccountDescriptor> {
        Ok( RawAccountDescriptor { path: input.to_string() })
    }

    fn comment(input: Node) -> Result<String> {
        print!("comment found; {}", input.as_str());
        Ok(input.to_string())
    }

    fn decimal_value(input: Node) -> Result<f64> {
        // Get the string captured by this node
        input.as_str()
            // Convert it into the type we want
            .parse::<f64>()
            // In case of  an error, we use `Node::error` to link the error
            // with the part of the input that caused it
            .map_err(|e| input.error(e))
    }


    fn iso8601_date_extended(input: Node) -> Result<String> {
        Ok(input.to_string())
    }

    fn trans_annotation(input: Node) -> Result<String> {
        Ok( input.to_string() )
    }
    fn trans_description(input: Node) -> Result<String> {
        Ok( input.to_string() )
    }

    fn transaction_block(input: Node) -> Result<RawTransaction> {
        let mut rt = RawTransaction::default();

        match_nodes!(input.into_children();
            [trans_header(hdr)] => {
                // TODO: copy from returned 
                rt = hdr
            },
            [posting_basic(pb)] => rt.postings.push(pb),
            [comment(c)] => rt.comment = c,
        );

        Ok(rt)
    }

    // trans_header = @{
    //     iso8601_date_extended
    //     ~ WHITESPACE+
    //     ~ trans_annotation
    //     ~ WHITESPACE+
    //     ~ trans_description
    //     ~ comment_or_newline
    // }
    fn trans_header(input: Node) -> Result<RawTransaction> {
        let mut rt = RawTransaction::default();

        match_nodes!(input.into_children();
            [iso8601_date_extended(ide)] => rt.date = ide,
            [trans_annotation(a)] => rt.anno = a,
            [trans_description(d)] => rt.desc = d,
            [comment(c)] => rt.comment = c,
        );

        Ok(rt)
    }

    // posting_basic       = @{
    //     posting_indent 
    //     ~ acct_descriptor
    //     ~ WHITESPACE+ ~ decimal_value
    //     ~ comment_or_newline
    // }
    fn posting_basic(input: Node) -> Result<RawPosting> {
        let mut p = RawPosting::default();

        match_nodes!(input.into_children();
            [acct_descriptor(ad)] => p.acct = ad,
            [decimal_value(v)] => p.value = v,
            [comment(c)] => p.comment = c,
        );

        Ok(p)
    }

    // directives = { balance_directive | directive_close | directive_commodity | directive_open }

    // fn directives(input: Node) -> Result<String> {
    //     match_nodes!(input.into_children();
    //         [balance_directive(dir)] => dir,
    //         [directive_close(dir)] => dir,

    //     );

    //     Ok("WIP".to_string())
    // }

    fn balance_directive(input: Node) -> Result<String> {
        match_nodes!(input.into_children();
            [iso8601_date_extended(d)] => Ok(d),
        )
    }

    fn directive_close(input: Node) -> Result<String> {
        match_nodes!(input.into_children();
            [iso8601_date_extended(d)] => Ok(d),
        )
    }

}