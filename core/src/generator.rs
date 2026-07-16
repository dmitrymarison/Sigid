// SPDX-License-Identifier: MIT OR Apache-2.0

//! ID generator

use crate::{SigId26, EPOCH, COUNTER_MAX};

/// ID generator as a pure state machine
#[derive(Debug, Clone)]
pub struct Generator {
    seed: u64,
    last_ms: u64,
    counter: u16,
}

impl Generator {
    /// Create new generator with seed
    pub const fn new(seed: u64) -> Self {
        Self {
            seed,
            last_ms: 0,
            counter: 0,
        }
    }

    /// Reset counter
    pub fn reset(&mut self) {
        self.counter = 0;
        self.last_ms = 0;
    }

    /// Generate ID from timestamp (milliseconds since UNIX_EPOCH)
    pub fn generate(&mut self, ms: u64) -> SigId26 {
        // Вычитаем EPOCH, чтобы хранить относительное время
        let timestamp = ms.saturating_sub(EPOCH);
        
        // Update counter
        if ms == self.last_ms {
            self.counter += 1;
            if self.counter > COUNTER_MAX {
                while ms == self.last_ms {
                    core::hint::spin_loop();
                }
                self.last_ms = ms;
                self.counter = 0;
            }
        } else {
            self.last_ms = ms;
            self.counter = 0;
        }

        let random = self.next_random();
        
        // Simple схема: 48+16+64
        SigId26::raw_new(timestamp, self.counter, random)
    }

    /// Xorshift64 random generator
    fn next_random(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.seed = x;
        x
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(0x9e3779b97f4a7c15)
    }
}