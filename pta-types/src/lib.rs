#[derive(Default)]
pub struct FilePosition {
    pub line: usize,
    pub col: usize        // TODO: u16? u32 is probably overkill
}

#[derive(Default)]
pub struct ParserInfo {
    pub position: FilePosition,
}


#[derive(Default)]
pub struct RawAccountDescriptor {
    pub path: String,
    pub pinfo: ParserInfo,
}


#[derive(Default)]
pub struct RawTransaction {
    pub date: String,
    pub anno: String,
    pub desc: String,
    pub postings: Vec<RawPosting>,
    pub comment: String,
    pub pinfo: ParserInfo,
}

#[derive(Default)]
pub struct RawPosting {
    pub acct: RawAccountDescriptor,
    pub value: f64,
    pub comment: String,
    pub pinfo: ParserInfo,
}
