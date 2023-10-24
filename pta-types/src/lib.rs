#[derive(Default, Clone)]
pub struct FilePosition {
    pub line: usize,
    pub col: usize        // TODO: u16? u32 is probably overkill
}

#[derive(Default, Clone)]
pub struct ParserInfo {
    pub position: FilePosition,
}


pub mod parsed_ledger;
pub use parsed_ledger::*;

pub mod raw_transaction;
pub use raw_transaction::*;