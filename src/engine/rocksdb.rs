use engine::CFName;
use rocksdb::{ColumnFamilyDescriptor, MergeOperands, Options, DB};
use std::sync::Arc;

pub struct RocksEngine {
    db: Arc<DB>,
}

impl RocksEngine {
    pub fn new(path: &str, cfs: &[&str], cfs_opts: Option<Vec<CFOptions>>) -> Result<RocksEngine> {
        info!("RocksEngine: creating for path {}", path);
        let mut opts = Options::default();
        let db = Arc::new(DB::open_cf(db_opt, path, cfs))?;
        Ok(RocksEngine { db })
    }
}

impl Engine for RocksEngine {
    fn get(&self, cf: &str, key: Key) -> Result<Option<Value>> {
        let handle = self.db.cf_handle(cf)?;
        let v = box_try!(self.db.get_cf(handle, key));
        Ok(v.map(|v| v.to_vec()))
    }
    fn put(&self, cf: &str, key: Key, value: Value) -> Result<()> {
        let handle = self.db.cf_handle(cf)?;
        self.db.put_cf(handle, key, value)
    }
    fn delete(&self, cf: &str, key: Key) -> Result<()> {
        let handle = self.db.cf_handle(cf)?;
        self.db.delete_cf(handle, key, value)
    }
}
