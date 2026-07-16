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
//! let gen = Generator::new()
//!     .with_worker_id(WorkerId::new(42).unwrap())
//!     .build();
//!
//! let id = gen.generate(1234567890);
//! let uuid = id.to_uuid();
//! println!("UUID: {}", uuid);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, missing_debug_implementations)]

pub mod worker;
pub mod inspect;

#[cfg(feature = "uuid")]
pub mod convert;

pub use worker::WorkerId;
pub use inspect::{inspect, InspectResult};

#[cfg(feature = "uuid")]
pub use convert::{SigIdUuidExt};

#[cfg(feature = "ulid")]
pub use convert::ulid::SigIdUlidExt;

/// Re-export core and flexible types
pub use sigid_core::{SigId26, Error, Result, EPOCH, COUNTER_MAX};
pub use sigid_flexible::{Generator as FlexibleGenerator, Alphabet};

/// Enterprise generator builder
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

    /// Set worker ID
    pub fn with_worker_id(mut self, worker_id: WorkerId) -> Self {
        self.worker_id = worker_id;
        self
    }

    /// Set worker ID from environment (std only)
    #[cfg(feature = "std")]
    pub fn with_worker_id_from_env(mut self) -> Self {
        self.worker_id = WorkerId::zero_config();
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

    /// Build the generator
    pub fn build(self) -> Self {
        self
    }

    /// Generate ID from timestamp
    pub fn generate(&self, ms: u64) -> SigId26 {
        let mut core_gen = sigid_core::Generator::new(self.worker_id.value() as u64);
        core_gen.generate(ms)
    }

    /// Generate ID with current time (std only)
    #[cfg(feature = "std")]
    pub fn now(&self) -> SigId26 {
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
        let gen = Generator::new();
        let id = gen.generate(1234567890);
        assert!(id.is_valid());
    }

    #[test]
    fn test_generator_with_worker_id() {
        let worker = WorkerId::new(42).unwrap();
        let gen = Generator::new().with_worker_id(worker);
        assert_eq!(gen.worker_id(), worker);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_generator_worker_from_env() {
        let gen = Generator::new().with_worker_id_from_env();
        assert!(gen.worker_id().value() <= 1023);
    }

    #[test]
    #[cfg(feature = "uuid")]
    fn test_generator_uuid_conversion() {
        use super::SigIdUuidExt;
        let gen = Generator::new();
        let id = gen.generate(1234567890);
        let uuid = id.to_uuid();
        let converted = SigId26::from_uuid(uuid);
        assert_eq!(id.as_bytes(), converted.as_bytes());
    }
}