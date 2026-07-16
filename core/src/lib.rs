// SPDX-License-Identifier: MIT OR Apache-2.0

//! # SigID Core
//!
//! Zero-dependency, no_std core for generating 26-character identifiers
//! in Crockford Base32 format.
//!
//! # Example
//! ```
//! use sigid_core::{Generator, SigId26};
//!
//! let mut gen = Generator::new(0x123456789abcdef);
//! let id = gen.generate(1234567890).unwrap();
//! println!("{}", id);
//! ```

#![no_std]
#![warn(missing_docs, missing_debug_implementations)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

mod checksum;
mod consts;
mod decoder;
mod encoder;
mod generator;
mod id;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "uuid")]
mod uuid_conv;

pub use checksum::{crc16, iso7064_checksum};
pub use consts::*;
pub use decoder::decode_crockford32;
pub use encoder::encode_crockford32;
pub use generator::Generator;
pub use id::SigId26;

/// Result type for no_std compatibility
pub type Result<T> = core::result::Result<T, Error>;

/// Error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// ID has invalid length (must be 26 chars)
    InvalidLength,
    /// ID contains invalid character
    InvalidCharacter,
    /// Checksum verification failed
    ChecksumMismatch,
    /// Timestamp overflow (max 2^48)
    TimestampOverflow,
    /// Counter overflow (max 16383)
    CounterOverflow,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Invalid ID length (must be 26 chars)"),
            Self::InvalidCharacter => write!(f, "Invalid character in ID"),
            Self::ChecksumMismatch => write!(f, "Checksum verification failed"),
            Self::TimestampOverflow => write!(f, "Timestamp overflow (max 2^48)"),
            Self::CounterOverflow => write!(f, "Counter overflow (max 16383)"),
        }
    }
}
