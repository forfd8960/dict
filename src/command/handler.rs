use crate::{errors::DictError, storage::Storage, Value};

use super::{Command, CommandHandle};

pub struct CommandHandler {
    store: Box<dyn Storage>,
}

impl CommandHandler {
    pub fn new(store: impl Storage + 'static) -> Self {
        Self {
            store: Box::new(store),
        }
    }
}

impl CommandHandle for CommandHandler {
    fn get(&self, cmd: Command) -> Result<Value, DictError> {
        let result = match cmd {
            Command::Get(key) => self.store.get(&key),
            _ => Err(DictError::InvalidCommand(format!("expect get command"))),
        };
        result
    }

    fn set(&self, cmd: Command) -> Result<(), DictError> {
        let result = match cmd {
            Command::Set(key, value) => self.store.set(&key, &value),
            _ => Err(DictError::InvalidCommand(format!("expect set command"))),
        };

        result
    }

    fn delete(&self, cmd: Command) -> Result<(), DictError> {
        let result = match cmd {
            Command::Delete(key) => self.store.delete(&key),
            _ => Err(DictError::InvalidCommand(format!("expect delete command"))),
        };

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::memory::MemDB;

    use super::*;

    #[test]
    fn test_handle_get() {
        let store = MemDB::new(100);
        let handler = CommandHandler::new(store);

        let result = handler.get(Command::Get("test-key".to_string()));
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            DictError::KeyNotFound("test-key".to_string())
        );

        let result = handler.get(Command::Delete("test-key".to_string()));
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            DictError::InvalidCommand("expect get command".to_string())
        );
    }

    #[test]
    fn test_handle_set() {
        let store = MemDB::new(100);
        let handler = CommandHandler::new(store);

        let result = handler.set(Command::Set(
            "test-key".to_string(),
            Value {
                value: b"1000".to_vec(),
            },
        ));
        assert!(result.is_ok());

        let result = handler.get(Command::Get("test-key".to_string()));
        assert!(result.is_ok());
        assert_eq!(
            result.ok().unwrap(),
            Value {
                value: b"1000".to_vec(),
            },
        );
    }

    #[test]
    fn test_handle_delete() {
        let store = MemDB::new(100);
        let handler = CommandHandler::new(store);

        let result = handler.set(Command::Set(
            "test-key".to_string(),
            Value {
                value: b"1000".to_vec(),
            },
        ));
        assert!(result.is_ok());

        let result = handler.delete(Command::Delete("test-key".to_string()));
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), ());

        let value: Result<Value, DictError> = handler.get(Command::Get("test-key".to_string()));
        assert!(value.is_err());
        assert_eq!(
            value.err().unwrap(),
            DictError::KeyNotFound("test-key".to_string())
        );
    }
}
