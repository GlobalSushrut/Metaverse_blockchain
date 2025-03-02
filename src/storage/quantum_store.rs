use std::collections::HashMap;
use rocksdb::{DB, Options};

pub struct QuantumStore {
    db: DB,
    entangled_pairs: HashMap<Vec<u8>, Vec<u8>>
}

impl QuantumStore {
    pub fn new(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path)?;

        Ok(Self {
            db,
            entangled_pairs: HashMap::new()
        })
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        self.db.put(key, value)?;
        Ok(())
    }

    pub fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, Box<dyn std::error::Error>> {
        Ok(self.db.get(key)?)
    }
}
