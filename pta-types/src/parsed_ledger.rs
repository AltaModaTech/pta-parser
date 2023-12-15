// Copyright (C) 2023, AltaModa Technologies, LLC. All rights reserved.
//
// This project is licensed under the terms of the MIT license (cf. LICENSE file in root).
//

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