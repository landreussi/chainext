use std::collections::BTreeSet;

use chrono::{DateTime, Utc};

use crate::hash::{Hash, Hashable};

/// This is a linked data container inside blockchains, it is generic since we
/// could store anything in it.
pub struct Block<T> {
    index: u32,
    timestamp: i64,
    hash: Hash,
    prev_block_hash: Hash,
    nonce: u64,
    content: BTreeSet<T>,
    difficulty: u128,
}

impl<T> Block<T>
where
    T: Ord,
{
    /// Initialize a new block.
    pub fn new(
        index: u32,
        time: DateTime<Utc>,
        prev_block_hash: Hash,
        content: impl Iterator<Item = T>,
        difficulty: u128,
    ) -> Self {
        Self {
            index,
            timestamp: time.timestamp_millis(),
            hash: Hash::default(),
            prev_block_hash,
            content: content.collect(),
            difficulty,
            nonce: 0,
        }
    }
}

impl<T> Hashable for Block<T>
where
    T: Hashable,
{
    fn bytes(&self) -> Vec<u8> {
        let mut buf = vec![];

        buf.extend(self.index.to_be_bytes());
        buf.extend(self.timestamp.to_be_bytes());
        buf.extend(self.prev_block_hash.bytes());
        buf.extend(self.nonce.to_be_bytes());
        buf.extend(
            self.content
                .iter()
                .flat_map(|content| content.bytes())
                .collect::<Vec<u8>>(),
        );
        buf.extend(self.difficulty.to_be_bytes());

        buf
    }
}

impl<T> Block<T>
where
    T: Hashable,
{
    /// Solve the mathematical problem to issue the block, once it does N
    /// hashing creation, it is a CPU/GPU intensive operation.
    pub fn mine(&mut self) {
        for nonce in 0..u64::MAX {
            self.nonce = nonce;
            let hash = self.hash();
            if self.dificulty_is_higher_than(&hash) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl<T> Block<T> {
    #[inline]
    fn dificulty_is_higher_than(&self, hash: &Hash) -> bool {
        self.difficulty > hash.difficulty()
    }
}
