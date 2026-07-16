// SPDX-License-Identifier: MIT OR Apache-2.0

//! Constants for Crockford Base32
//!
//! # Bit Layout (Единая схема: 48+16+14+50)
//!
//! - Bytes 0-5: Timestamp (48 bits)
//! - Bytes 6-7: Worker ID (16 bits)
//! - Bytes 8-15: Counter (14 bits) + Random (50 bits)

/// Crockford Base32 alphabet
pub const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Crockford Base32 alphabet (lowercase)
pub const ALPHABET_LOWER: &[u8; 32] = b"0123456789abcdefghjkmnpqrstvwxyz";

/// ID length in characters
pub const ID_LENGTH: usize = 26;

/// ID length in bytes (128 bits)
pub const ID_BYTES: usize = 16;

/// Timestamp bits (48 bits)
pub const TIMESTAMP_BITS: u8 = 48;

/// Worker ID bits (16 bits)
pub const WORKER_BITS: u8 = 16;

/// Counter bits (14 bits)
pub const COUNTER_BITS: u8 = 14;

/// Random bits (50 bits)
pub const RANDOM_BITS: u8 = 50;

/// Maximum timestamp value (2^48 - 1)
pub const TIMESTAMP_MAX: u64 = (1u64 << 48) - 1;

/// Maximum worker ID value (2^16 - 1)
pub const WORKER_MAX: u16 = ((1u32 << 16) - 1) as u16;

/// Maximum counter value (2^14 - 1) = 16383
pub const COUNTER_MAX: u16 = ((1u32 << 14) - 1) as u16;

/// Maximum random value (2^50 - 1)
pub const RANDOM_MAX: u64 = (1u64 << 50) - 1;

/// Epoch: 2020-01-01 00:00:00 UTC (milliseconds)
pub const EPOCH: u64 = 1577836800000;
