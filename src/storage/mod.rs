use crate::{errors::DictError, Value};

pub mod memory;

pub trait Storage {
    fn get(&self, key: &str) -> Result<Value, DictError>;
    fn set(&self, key: &str, value: &Value) -> Result<(), DictError>;
    fn delete(&self, key: &str) -> Result<(), DictError>;
}
