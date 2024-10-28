use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{errors::DictError, Value};

use super::Storage;

#[derive(Debug, Clone)]
pub struct MemDB {
    hash: Arc<RwLock<HashMap<String, Value>>>,
}

impl MemDB {
    pub fn new(size: usize) -> Self {
        Self {
            hash: Arc::new(RwLock::new(HashMap::with_capacity(size))),
        }
    }

    pub fn length(&self) -> usize {
        let rw_read = self.hash.read().unwrap();
        rw_read.len()
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
    use std::thread;

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

    #[test]
    fn test_concurrent_set() -> Result<(), DictError> {
        let db = MemDB::new(100);

        let mut handlers = vec![];
        for idx in 0..100 {
            let db_clone = db.clone();
            let handle = thread::spawn(move || {
                let _ = db_clone.set(
                    &format!("test-key-{}", idx),
                    &Value {
                        value: b"100".to_vec(),
                    },
                );
            });
            handlers.push(handle);
        }

        let _: Vec<_> = handlers.into_iter().map(|x| x.join().unwrap()).collect();
        assert_eq!(db.length(), 100);

        for idx in 0..100 {
            let v = db.get(&format!("test-key-{}", idx))?;
            println!("{}: {:?}", idx, v);
            assert_eq!(
                v,
                Value {
                    value: b"100".to_vec()
                }
            );
        }

        Ok(())
    }
}
