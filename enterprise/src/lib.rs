// SPDX-License-Identifier: MIT OR Apache-2.0

//! # SigID Enterprise
//!
//! Enterprise-grade ID generator for distributed systems.
//!
//! Features:
//! - Worker ID coordination (static/dynamic)
//! - UUID/ULID conversion
//! - ID inspection and metadata extraction
//!
//! # Example
//! ```
//! use sigid_enterprise::{Generator, WorkerId, SigIdUuidExt};
//!
//! let mut gen = Generator::new()
//!     .with_worker_id(WorkerId::new(42).unwrap())
//!     .build();
//!
//! let id = gen.generate(1234567890).unwrap();
//! let uuid = id.to_uuid();
//! println!("UUID: {}", uuid);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, missing_debug_implementations)]

pub mod inspect;
pub mod worker;

#[cfg(feature = "uuid")]
pub mod convert;

pub use inspect::{inspect, InspectResult};
pub use worker::WorkerId;

#[cfg(feature = "uuid")]
pub use convert::SigIdUuidExt;

#[cfg(feature = "ulid")]
pub use convert::ulid::SigIdUlidExt;

/// Re-export core and flexible types
pub use sigid_core::{Error, Result, SigId26, COUNTER_MAX, EPOCH};
pub use sigid_flexible::{Alphabet, Generator as FlexibleGenerator};

/// Enterprise generator builder with persistent state
#[derive(Debug, Clone)]
pub struct Generator {
    flexible: FlexibleGenerator,
    worker_id: WorkerId,
}

impl Generator {
    /// Create new enterprise generator
    pub fn new() -> Self {
        Self {
            flexible: FlexibleGenerator::new(),
            worker_id: WorkerId::from(0),
        }
    }

    /// Set worker ID (используется в битовой схеме, а не как сид)
    pub fn with_worker_id(mut self, worker_id: WorkerId) -> Self {
        self.worker_id = worker_id;
        self.flexible = self.flexible.worker_id(worker_id.value());
        self
    }

    /// Set seed for the generator
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.flexible = self.flexible.with_seed(seed);
        self
    }

    /// Set worker ID from environment (std only)
    #[cfg(feature = "std")]
    pub fn with_worker_id_from_env(mut self) -> Self {
        self.worker_id = WorkerId::zero_config();
        self.flexible = self.flexible.worker_id(self.worker_id.value());
        self
    }

    /// Set ID length
    pub fn length(mut self, length: usize) -> Self {
        self.flexible = self.flexible.length(length);
        self
    }

    /// Set alphabet
    pub fn alphabet(mut self, alphabet: Alphabet) -> Self {
        self.flexible = self.flexible.alphabet(alphabet);
        self
    }

    /// Enable checksum
    pub fn with_checksum(mut self, enabled: bool) -> Self {
        self.flexible = self.flexible.with_checksum(enabled);
        self
    }

    /// Build the generator (consumes builder)
    pub fn build(self) -> Self {
        self
    }

    /// Generate ID from timestamp
    pub fn generate(&mut self, ms: u64) -> Result<SigId26> {
        self.flexible.generate_raw(ms)
    }

    /// Generate ID with current time (std only)
    #[cfg(feature = "std")]
    pub fn now(&mut self) -> Result<SigId26> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        self.generate(ms)
    }

    /// Get current worker ID
    pub fn worker_id(&self) -> WorkerId {
        self.worker_id
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_new() {
        let mut gen = Generator::new();
        let id = gen.generate(1234567890).unwrap();
        assert!(id.is_valid());
    }

    #[test]
    fn test_generator_with_worker_id() {
        let worker = WorkerId::new(42).unwrap();
        let mut gen = Generator::new().with_worker_id(worker);
        assert_eq!(gen.worker_id(), worker);

        let id = gen.generate(1234567890).unwrap();
        assert_eq!(id.worker_id(), worker.value());
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_generator_worker_from_env() {
        let mut gen = Generator::new().with_worker_id_from_env();
        let worker_val = gen.worker_id().value();

        let id = gen.generate(1234567890).unwrap();
        assert_eq!(id.worker_id(), worker_val);
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_generator_uuid_conversion() {
        use super::SigIdUuidExt;
        let mut gen = Generator::new();
        let id = gen.generate(1234567890).unwrap();
        let uuid = id.to_uuid();
        let converted = SigId26::from_uuid(uuid);
        assert_eq!(id.as_bytes(), converted.as_bytes());
    }

    #[test]
    fn test_generator_timestamp() {
        let mut gen = Generator::new();
        let ms = 1700000000000;
        let id = gen.generate(ms).unwrap();
        assert_eq!(id.timestamp_ms(), ms);
    }

    #[test]
    fn test_generator_counter_increment() {
        let mut gen = Generator::new();
        let ms = 1234567890;

        let id1 = gen.generate(ms).unwrap();
        let id2 = gen.generate(ms).unwrap();

        assert_eq!(id2.counter(), id1.counter() + 1);
    }

    #[test]
    fn test_with_seed() {
        let mut gen = Generator::new().with_seed(0x123456789abcdef);
        let id = gen.generate(1234567890).unwrap();
        assert!(id.is_valid());
    }
}
