use crate::{errors::DictError, storage::Storage, Value};

use super::{Command, CommandHandle};

pub struct CommandHandler<'a, T: Storage + 'a> {
    store: &'a T,
}

impl<'a, T: Storage> CommandHandler<'a, T> {
    pub fn new(store: &'a T) -> Self {
        Self { store }
    }
}

impl<'a, T: Storage> CommandHandle for CommandHandler<'a, T> {
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
