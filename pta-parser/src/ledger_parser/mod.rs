// Copyright (C) 2023, AltaModa Technologies, LLC. All rights reserved.
//
// This project is licensed under the terms of the MIT license (cf. LICENSE file in root).
//



use pest_derive::*;

#[derive(Parser)]
#[grammar = "./grammars/base.pest"]
#[grammar = "./grammars/generic.pest"]
pub struct GenericParser;


