pub type Key = &[u8];
pub type Value = &[u8];
pub struct KVPair(Key, Value);
pub struct DBOption {
    columnFamily: String
}

pub trait Engine {
    fn get(&self, key: Key, option: DBOption) -> Result<Value>;
    fn put(&self, key: Key, value: Value, option: DBOption) -> Result<()>;
    fn delete(&self, key: Key, option: DBOption) -> Result<()>;
}
