
use log::{info, warn, as_error};

use pta_types::*;



// TODO: how to isolate pest so clients can just use lib (w/o requiring pest as here)
use pest::{*, iterators::Pair};
use pta_parser::{GenericParser, Rule};


#[derive(Default)]
pub struct LedgerBuilder {
    pl: ParsedLedger
}

impl LedgerBuilder {
    pub fn from_string(self: &mut Self, ledger: &String) -> Result<&mut ParsedLedger, Box<dyn std::error::Error>> {

        self.pl = ParsedLedger::default();

        match GenericParser::parse(Rule::ledger, &ledger) {
            Ok(root) => {
                info!("Successfully parsed with Rule::ledger");
                for pair in root.into_iter() {
                    info!("LedgerBuilder::from_string: root pair is {:}", pair.as_str());
                    self.handle_pair(pair)?;
                } 
            }
    
            Err(err) => {
                warn!(err = as_error!(err); "failed to parse with Rule::ledger");
                return Err(Box::new(err));
            }
        }
    
        return Ok(&mut self.pl);
    }


    fn handle_pair(self: &Self, pair: Pair<'_, Rule>) -> Result<(), Box<dyn std::error::Error>> {

        match pair.as_rule() {
            Rule::comment => {
                info!("Rule::comment: {:?}", pair.as_span().as_str());
            }
            Rule::EOI => { 
                info!("Rule::EOI at {:?}", pair.line_col());
            }
    
            Rule::WHITESPACE => {}
            Rule::acct_descriptor => { dump_pair(&pair); return Ok(()); }
            Rule::acct_separator => { dump_pair(&pair); return Ok(()); }
            Rule::balance_directive => { dump_pair(&pair); return Ok(()); }
            Rule::comment_or_newline => { dump_pair(&pair); return Ok(()); }
            Rule::comment_token => { dump_pair(&pair); return Ok(()); }
            Rule::currency => { dump_pair(&pair); return Ok(()); }
            Rule::decimal_value => { dump_pair(&pair); return Ok(()); }
            Rule::directive_close => { dump_pair(&pair); return Ok(()); }
            Rule::directive_commodity => { dump_pair(&pair); return Ok(()); }
            Rule::directive_open => { dump_pair(&pair); return Ok(()); }
            Rule::directives => { dump_pair(&pair); return Ok(()); }
            Rule::empty_line => {}
            Rule::iso8601_date_extended => { dump_pair(&pair); return Ok(()); }
            Rule::ledger => { 
                return handle_ledger_rule(&pair);
            }
            Rule::options => { dump_pair(&pair); return Ok(()); }
            Rule::posting_basic => { 
                dump_pair(&pair); return Ok(());
            }
            Rule::posting_indent => { dump_pair(&pair); return Ok(()); }
            Rule::sub_acct => { dump_pair(&pair); return Ok(()); }
            Rule::top_level_acct => { dump_pair(&pair); return Ok(()); }
            Rule::trans_annotation => { dump_pair(&pair); return Ok(()); }
            Rule::trans_description => { dump_pair(&pair); return Ok(()); }
            Rule::trans_description_text => { dump_pair(&pair); return Ok(()); }
            Rule::trans_header => {
                let mut xn = raw_transaction::RawTransaction::default();
                return handle_trans_header(&mut xn, &pair);
            }
            Rule::transaction_block => {
                let mut xn = raw_transaction::RawTransaction::default();
                return handle_trans_block(&mut xn, &pair);
            }
        }
    
        return Ok(());
    
    }
    
}


fn dump_rule_of_pair(p: &Pair<Rule>) {
    info!("RULE: {:?} at {:?}; SPAN: {:?}", &p.as_rule(), &p.line_col(), &p.as_span());
}

// REMOVE:
#[allow(dead_code)]
fn dump_rule(r:&Rule, s:&Span) {
    info!("RULE: {:?}; SPAN: {:?}", &r, &s);
}

fn dump_pair(p:&Pair<Rule>) {
    dump_rule_of_pair(p);
}



fn handle_ledger_rule(pair: & Pair<Rule>) -> Result<(), Box<dyn std::error::Error>> {
    for inner_pair in pair.clone().into_inner() {

        match handle_pair(inner_pair) {
            Ok(_p) => { /* handle_pair does all the work */ }

            Err(err) => {
                warn!(err = as_error!(*err); "handle_pair failed in handle_ledger");
                return Err(err);
            }
        };
    }

    return Ok(());
}

#[allow(dead_code)]  // TODO: REMOVE allow dead code
fn handle_posting_basic(_xn: &mut raw_transaction::RawTransaction, pair: &Pair<Rule>) -> Result<(), Box<dyn std::error::Error>> {

    match GenericParser::parse(Rule::posting_basic, pair.as_span().as_str()) {
        Ok(_posting) => {
            info!("handling posting_basic");
            // handle_posting_basic(xn, posting);  TODO: fix
        }
        
        Err(e) => {
            warn!(err = as_error!(e); "failed to parse with posting_basic");
            return Err(Box::new(e));
        }
            
    }

    return Ok(());
}

fn handle_trans_header(_: &mut raw_transaction::RawTransaction, _: &Pair<Rule>) -> Result<(), Box<dyn std::error::Error>> {
    info!("handling trans_header...");

    return Ok(());
}

fn handle_trans_block(xn: &mut raw_transaction::RawTransaction, pair: &Pair<Rule>) -> Result<(), Box<dyn std::error::Error>> {
    info!("handling trans_block...");

    xn.pinfo = ParserInfo {
        position: FilePosition {
            line: pair.line_col().0,
            col: pair.line_col().1
        }
    };

    info!("parse with trans_header");
    match GenericParser::parse(Rule::trans_header, &pair.as_span().as_str()) {
        Ok(hdr) => {
            for pair in hdr.into_iter() {
                info!("attempt handle_trans_header on {}", pair.as_span().as_str());
                match handle_trans_header(xn, &pair) {
                    Ok(()) => {
                        // TODO: REVIEW: should anything happen here?
                    }

                    Err(e) => {
                        warn!(err = e; "handle_trans_header failed");
                        return Err(e);
                    }
                }

            }
            // for p in &pair.into_inner() {
            //     handle_posting_basic(&mut xn, &p);
            // }
        }

        Err(e) => {
            warn!(err = as_error!(e); "failed to parse with trans_header");
            return Err(Box::new(e));
        }
    }

    return Ok(());

}




fn handle_pair(pair: Pair<'_, Rule>) -> Result<(), Box<dyn std::error::Error>> {
    match pair.as_rule() {
        Rule::comment => {
            info!("Rule::comment: {:?}", pair.as_span().as_str());
        }
        Rule::EOI => { 
            info!("Rule::EOI at {:?}", pair.line_col());
        }

        Rule::WHITESPACE => {}
        Rule::acct_descriptor => { dump_pair(&pair); return Ok(()); }
        Rule::acct_separator => { dump_pair(&pair); return Ok(()); }
        Rule::balance_directive => { dump_pair(&pair); return Ok(()); }
        Rule::comment_or_newline => { dump_pair(&pair); return Ok(()); }
        Rule::comment_token => { dump_pair(&pair); return Ok(()); }
        Rule::currency => { dump_pair(&pair); return Ok(()); }
        Rule::decimal_value => { dump_pair(&pair); return Ok(()); }
        Rule::directive_close => { dump_pair(&pair); return Ok(()); }
        Rule::directive_commodity => { dump_pair(&pair); return Ok(()); }
        Rule::directive_open => { dump_pair(&pair); return Ok(()); }
        Rule::directives => { dump_pair(&pair); return Ok(()); }
        Rule::empty_line => {}
        Rule::iso8601_date_extended => { dump_pair(&pair); return Ok(()); }
        Rule::ledger => { 
            return handle_ledger_rule(&pair);
        }
        Rule::options => { dump_pair(&pair); return Ok(()); }
        Rule::posting_basic => { dump_pair(&pair); return Ok(()); }
        Rule::posting_indent => { dump_pair(&pair); return Ok(()); }
        Rule::sub_acct => { dump_pair(&pair); return Ok(()); }
        Rule::top_level_acct => { dump_pair(&pair); return Ok(()); }
        Rule::trans_annotation => { dump_pair(&pair); return Ok(()); }
        Rule::trans_description => { dump_pair(&pair); return Ok(()); }
        Rule::trans_description_text => { dump_pair(&pair); return Ok(()); }
        Rule::trans_header => {
            let mut xn = raw_transaction::RawTransaction::default();
            return handle_trans_header(&mut xn, &pair);
        }
        Rule::transaction_block => {
            let mut xn = raw_transaction::RawTransaction::default();
            return handle_trans_block(&mut xn, &pair);
        }
    }

    return Ok(());

}


