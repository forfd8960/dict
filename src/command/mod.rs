use crate::{errors::DictError, Value};

pub mod handler;
pub mod handler_v1;

#[derive(Debug)]
pub enum Command {
    Get(String),
    Set(String, Value),
    Delete(String),
}

pub trait CommandHandle {
    fn get(&self, cmd: Command) -> Result<Value, DictError>;
    fn set(&self, cmd: Command) -> Result<(), DictError>;
    fn delete(&self, cmd: Command) -> Result<(), DictError>;
}
