use albedo::engine::RocksEngine;
use rocksdb::{ColumnFamilyDescriptor, MergeOperands, Options, DB};

#[test]
fn test_rocksdb_open() {
    let path = "/tmp/test_rocksdb";
    let cfs = &["test_cf_01"];
    let mut opts = Options::default();
    opts.create_if_missing(true);
    opts.create_missing_column_families(true);
    match RocksEngine::new(path, cfs, opts) {
        Ok(_) => println!("successfully created new column family"),
        Err(e) => panic!("failed to create new column family: {}", e),
    }
}
