use crate::Value;

#[derive(Debug)]
pub enum Command {
    Get(String),
    Set(String, Value),
    Delete(String),
}
