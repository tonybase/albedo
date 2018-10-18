pub type Key = &[u8];
pub type Value = &[u8];
pub struct KVPair(Key, Value);

pub trait Engine {
    fn get(&self, cf: &str, key: Key) -> Result<Option<Value>>;
    fn put(&self, cf: &str, key: Key, value: Value) -> Result<()>;
    fn delete(&self, cf: &str, key: Key) -> Result<()>;
}
