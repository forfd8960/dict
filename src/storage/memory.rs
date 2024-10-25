use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{errors::DictError, Value};

use super::Storage;

pub struct MemDB {
    hash: Arc<RwLock<HashMap<String, Value>>>,
}

impl MemDB {
    pub fn new(size: usize) -> Self {
        Self {
            hash: Arc::new(RwLock::new(HashMap::with_capacity(size))),
        }
    }
}

impl Storage for MemDB {
    fn get(&self, key: &str) -> Result<Value, DictError> {
        let rw_lock = self.hash.read().unwrap();
        let val = rw_lock.get(&key.to_string());
        match val {
            Some(v) => Ok(v.clone()),
            None => Err(DictError::KeyNotFound(key.to_string())),
        }
    }

    fn set(&self, key: &str, value: &Value) -> Result<(), DictError> {
        let mut rw_lock = self.hash.write().unwrap();
        rw_lock.insert(key.to_string(), value.clone());
        Ok(())
    }

    fn delete(&self, key: &str) -> Result<(), DictError> {
        let mut rw_lock = self.hash.write().unwrap();
        rw_lock.remove(&key.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inmem_get() -> Result<(), DictError> {
        let db = MemDB::new(100);
        let value = db.get("test-key");
        assert!(value.is_err());
        assert_eq!(
            DictError::KeyNotFound("test-key".to_string()),
            value.err().unwrap()
        );
        Ok(())
    }

    #[test]
    fn test_inmem_set() -> Result<(), DictError> {
        let db = MemDB::new(100);
        db.set(
            "test-key",
            &Value {
                value: b"abc".to_vec(),
            },
        )?;

        let value = db.get("test-key")?;

        assert_eq!(
            value,
            Value {
                value: b"abc".to_vec()
            },
        );
        Ok(())
    }

    #[test]
    fn test_inmem_delete() -> Result<(), DictError> {
        let db = MemDB::new(100);
        db.set(
            "test-key",
            &Value {
                value: b"abc".to_vec(),
            },
        )?;

        let value = db.get("test-key")?;

        assert_eq!(
            value,
            Value {
                value: b"abc".to_vec()
            },
        );

        db.delete("test-key")?;

        let value = db.get("test-key");
        assert_eq!(
            value.err().unwrap(),
            DictError::KeyNotFound("test-key".to_string()),
        );

        Ok(())
    }
}
