// SPDX-License-Identifier: MIT OR Apache-2.0

//! Constants for Crockford Base32

/// Crockford Base32 alphabet (без I, L, O, U)
pub const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

/// Crockford Base32 alphabet (lowercase)
pub const ALPHABET_LOWER: &[u8; 32] = b"0123456789abcdefghjkmnpqrstvwxyz";

/// ID length in characters
pub const ID_LENGTH: usize = 26;

/// ID length in bytes (128 bits)
pub const ID_BYTES: usize = 16;

/// Maximum counter value (2^16 - 1)
pub const COUNTER_MAX: u16 = 65535;

/// Epoch: 2020-01-01 00:00:00 UTC (milliseconds)
pub const EPOCH: u64 = 1577836800000;