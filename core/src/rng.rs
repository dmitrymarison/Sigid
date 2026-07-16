// SPDX-License-Identifier: MIT OR Apache-2.0

//! Random number generators

#[cfg(feature = "wyrand")]
pub use wyrand::WyRand as WyRng;

/// Fast but predictable PRNG (Xorshift64)
#[derive(Debug, Clone)]
pub struct Xorshift64 {
    seed: u64,
}

impl Xorshift64 {
    pub fn new(seed: u64) -> Self {
        Self { seed: if seed == 0 { 0x9e3779b97f4a7c15 } else { seed } }
    }
    
    pub fn next(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.seed = x;
        x
    }
}

#[cfg(feature = "secure")]
mod secure_rng {
    use getrandom::getrandom;
    
    pub fn secure_seed() -> u64 {
        let mut buf = [0u8; 8];
        getrandom(&mut buf).unwrap();
        u64::from_be_bytes(buf)
    }
}