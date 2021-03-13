use std::cmp::Ordering;

/// `Block` is the smallest unit of data
/// it contains a meta data about the block, the key & the value
///
/// Layout :
///
/// ```text
/// +------------+----------------+--------------+---------------+-------------+
/// | CRC () | deleted (bool) | version(u32) | key (Vec<u8>) | value (u32) |
/// +------------+----------------+--------------+---------------+-------------+
/// ```
struct DataBlock {
    pub id: u128,
    pub deleted: bool,
    pub key: Vec<u8>,
    pub value: Option<Vec<u8>>,
}

impl Eq for Block {}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Block {}
