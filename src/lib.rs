pub mod command;
pub mod errors;
pub mod storage;

#[derive(Debug)]
pub struct Value {
    pub value: Vec<u8>,
}
