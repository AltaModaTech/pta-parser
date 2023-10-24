use super::*;
use raw_transaction;


#[derive(Default)]
pub struct ParsedLedger {
    xns: Vec<raw_transaction::RawTransaction>,
}

impl ParsedLedger {
    pub fn add_transaction(self: &mut Self, xn: raw_transaction::RawTransaction) {
        self.xns.push(xn);
    }
}