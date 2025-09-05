use sled::{Db, IVec};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("sled error: {0}")]
    Sled(#[from] sled::Error),
}

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn open(path: &str) -> Result<Self, StorageError> {
        let db = sled::open(path)?;
        Ok(Storage { db })
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<IVec>, StorageError> {
        Ok(self.db.get(key)?)
    }

    pub fn set(&self, key: &[u8], val: &[u8]) -> Result<(), StorageError> {
        self.db.insert(key, val)?;
        self.db.flush()?;
        Ok(())
    }
}