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
