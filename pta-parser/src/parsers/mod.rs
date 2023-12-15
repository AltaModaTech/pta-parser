// Copyright (C) 2023, AltaModa Technologies, LLC. All rights reserved.
//
// This project is licensed under the terms of the MIT license (cf. LICENSE file in root).
//



use pest_derive::*;

pub mod generic {

    use super::*;
    #[derive(Parser)]
    #[grammar = "./grammars/base.pest"]
    #[grammar = "./grammars/generic.pest"]
    pub struct Parser;

}


pub mod beancount {

    use super::*;

    #[derive(Parser)]
    #[grammar = "./grammars/base.pest"]
    #[grammar = "./grammars/beancount.pest"]
    pub struct Parser;

}