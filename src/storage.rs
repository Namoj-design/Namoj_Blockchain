use sled::{Db, IVec};

#[derive(Clone)]
pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new(path: &str) -> Result<Self, sled::Error> {
        let db = sled::open(path)?;
        Ok(Storage { db })
    }

    pub fn put(&self, key: &str, value: &[u8]) -> Result<(), sled::Error> {
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Option<IVec> {
        self.db.get(key).ok().flatten()
    }
}
