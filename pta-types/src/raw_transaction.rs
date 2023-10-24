use super::*;


#[derive(Default, Clone)]
pub struct RawAccountDescriptor {
    pub path: String,
    pub pinfo: ParserInfo,
}


#[derive(Default, Clone)]
pub struct RawTransaction {
    pub date: String,
    pub anno: String,
    pub desc: String,
    pub postings: Vec<RawPosting>,
    pub comment: String,
    pub pinfo: ParserInfo,
}

#[derive(Default, Clone)]
pub struct RawPosting {
    pub acct: RawAccountDescriptor,
    pub value: f64,
    pub comment: String,
    pub pinfo: ParserInfo,
}
