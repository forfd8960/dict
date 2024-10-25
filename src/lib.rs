pub mod command;
pub mod errors;
pub mod storage;

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    pub value: Vec<u8>,
}
