mod skiplist;
use serde::{Deserialize, Serialize};
use skiplist::SkipList;

const MAX_CAPACITY: usize = 1024 * 1024 * 1024;

struct MemTable {
    skip_list: SkipList<Key>,
    size: usize,
}

// private
impl MemTable {
    fn new() -> MemTable {
        skip_list = SkipList::new(16, 4);
    }

    fn write_to_wal(block: Block) {}

    fn insert_skip_list(k: Vec<u8>) {}
}

// pub
impl MemTable {
    pub fn add(key: Vec<u8>, value: Vec<u8>) {
        self.write_to_wal();
        self.insert_skip_list(key);
    }
}
