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
        todo!()
    }

    fn set(&self, cmd: Command) -> Result<(), DictError> {
        todo!()
    }

    fn delete(&self, cmd: Command) -> Result<(), DictError> {
        todo!()
    }
}
