#[derive(Debug, PartialEq)]
pub enum DictError {
    KeyNotFound(String),
    InvalidCommand(String),
}
