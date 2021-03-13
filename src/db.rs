mod memtable;

use memtable::MemTable;

pub struct Db {
    memtable: MemTable,
}

impl Db {
    pub fn get(ro: ReadOptions, key: Vec<u8>) {}
    pub fn put(wo: WriteOptions, key: Vec<u8>, value: Vec<u8>) {}
    pub fn delete(wo: WriteOptions, key: Vec<u8>) {}
}
