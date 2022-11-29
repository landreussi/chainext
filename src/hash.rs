use std::ops::Range;

/// Indicates if you can convert a type to an vector of bytes, giving the option
/// to hash it either.
// TODO: Split this up, and make it support more algorithms.
pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> Hash {
        crypto_hash::digest(crypto_hash::Algorithm::SHA256, &self.bytes())
            .try_into()
            .unwrap()
    }
}

/// Contains a 32 bytes hash and provides useful methods.
// TODO: Make the length of this be generic;
pub struct Hash([u8; 32]);

impl Default for Hash {
    fn default() -> Self {
        Self([0; 32])
    }
}

impl Hash {
    /// Calculate the difficulty of the hash by fetching the last half of the
    /// buffer, and doing a bitwise OR operation, to get the higher value in
    /// the buffer.
    pub fn difficulty(&self) -> u128 {
        const HEXADECIMAL_RANGE: Range<u8> = 0..16;

        self.0[16..32]
            .into_iter()
            .zip(HEXADECIMAL_RANGE)
            .map(|(byte, hex)| ((*byte) << hex * 8) as u128)
            .fold(0, |acc, byte| acc | byte)
    }

    /// Get the inner type, which is a 32 bytes buffer.
    pub fn bytes(&self) -> [u8; 32] {
        self.0
    }
}

impl TryFrom<Vec<u8>> for Hash {
    type Error = Vec<u8>;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        value.try_into()
    }
}
