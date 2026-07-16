// SPDX-License-Identifier: MIT OR Apache-2.0

//! Type-safe ID wrapper
//!
//! # Bit Layout (Единая схема: 48+16+14+50)
//!
//! - Bytes 0-5: Timestamp (48 bits)
//! - Bytes 6-7: Worker ID (16 bits)
//! - Bytes 8-15: Counter (14 bits) + Random (50 bits)
//!
//! For Simple version, Worker ID is always 0.

use crate::{
    decode_crockford32, encode_crockford32, Error, ALPHABET, COUNTER_MAX, EPOCH, ID_LENGTH,
    TIMESTAMP_MAX, WORKER_MAX,
};
use core::fmt;
use core::str::FromStr;

/// 26-character identifier in Crockford Base32 format
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct SigId26 {
    bytes: [u8; ID_LENGTH],
}

impl SigId26 {
    /// Create ID from raw bytes with validation
    pub fn from_bytes(bytes: [u8; ID_LENGTH]) -> Result<Self, Error> {
        for &b in &bytes {
            match b {
                b'0'..=b'9' => {}
                b'A'..=b'Z' => {
                    let mut found = false;
                    for &ch in ALPHABET {
                        if ch == b {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return Err(Error::InvalidCharacter);
                    }
                }
                _ => return Err(Error::InvalidCharacter),
            }
        }
        Ok(Self { bytes })
    }

    /// Create from raw 16-byte array (128 bits)
    pub fn from_raw_bytes(raw: [u8; 16]) -> Self {
        let mut encoded = [0u8; 26];
        encode_crockford32(&raw, &mut encoded);
        Self { bytes: encoded }
    }

    /// Create from raw components (единая схема: 48+16+14+50)
    ///
    /// # Arguments
    /// * `timestamp` - Milliseconds since EPOCH (2020-01-01), 48 bits max
    /// * `counter` - Counter, 14 bits max (0-16383)
    /// * `random` - Random value, 50 bits max
    ///
    /// # Note
    /// Для Simple версии используется worker_id = 0.
    /// Для Enterprise версии используйте `raw_new_with_worker`.
    pub fn raw_new(timestamp: u64, counter: u16, random: u64) -> Self {
        Self::raw_new_with_worker(timestamp, 0, counter, random)
    }

    /// Create from raw components with Worker ID (единая схема: 48+16+14+50)
    ///
    /// # Layout
    /// - Bytes 0-5: Timestamp (48 bits)
    /// - Bytes 6-7: Worker ID (16 bits)
    /// - Bytes 8-15: Counter (14 bits) + Random (50 bits)
    ///
    /// Byte layout:
    /// - Byte 8: Counter bits 13-8 (6 bits)
    /// - Byte 9: Counter bits 7-2 (6 bits) + Random bits 49-48 (2 bits)
    /// - Byte 10: Random bits 47-40 (8 bits)
    /// - Byte 11: Random bits 39-32 (8 bits)
    /// - Byte 12: Random bits 31-24 (8 bits)
    /// - Byte 13: Random bits 23-16 (8 bits)
    /// - Byte 14: Random bits 15-8 (8 bits)
    /// - Byte 15: Random bits 7-0 (8 bits)
    ///
    /// # Arguments
    /// * `timestamp` - Milliseconds since EPOCH (2020-01-01), 48 bits max
    /// * `worker_id` - Worker ID, 16 bits max (0-65535), для Simple версии = 0
    /// * `counter` - Counter, 14 bits max (0-16383)
    /// * `random` - Random value, 50 bits max
    pub fn raw_new_with_worker(timestamp: u64, worker_id: u16, counter: u16, random: u64) -> Self {
        let ts = timestamp & TIMESTAMP_MAX; // 48 bits
        let worker = worker_id & WORKER_MAX; // 16 bits
        let cnt = counter & COUNTER_MAX; // 14 bits
        let rand = random & 0x3FFFFFFFFFFFF; // 50 bits (2^50 - 1)

        let mut raw = [0u8; 16];

        // Timestamp: 48 bits (bytes 0-5)
        raw[0] = ((ts >> 40) & 0xFF) as u8;
        raw[1] = ((ts >> 32) & 0xFF) as u8;
        raw[2] = ((ts >> 24) & 0xFF) as u8;
        raw[3] = ((ts >> 16) & 0xFF) as u8;
        raw[4] = ((ts >> 8) & 0xFF) as u8;
        raw[5] = (ts & 0xFF) as u8;

        // Worker ID: 16 bits (bytes 6-7)
        raw[6] = ((worker >> 8) & 0xFF) as u8;
        raw[7] = (worker & 0xFF) as u8;

        // Counter: 14 bits (bytes 8-9)
        // Байт 8: старшие 6 бит counter (биты 13-8)
        raw[8] = ((cnt >> 6) & 0xFF) as u8;
        // Байт 9: младшие 6 бит counter (биты 7-2) + старшие 2 бита random (биты 49-48)
        raw[9] = ((cnt & 0x3F) << 2) as u8;
        raw[9] |= ((rand >> 48) & 0x03) as u8;

        // Random: оставшиеся 48 бит (bytes 10-15)
        raw[10] = ((rand >> 40) & 0xFF) as u8;
        raw[11] = ((rand >> 32) & 0xFF) as u8;
        raw[12] = ((rand >> 24) & 0xFF) as u8;
        raw[13] = ((rand >> 16) & 0xFF) as u8;
        raw[14] = ((rand >> 8) & 0xFF) as u8;
        raw[15] = (rand & 0xFF) as u8;

        Self::from_raw_bytes(raw)
    }

    /// Get raw bytes
    pub const fn as_bytes(&self) -> &[u8; ID_LENGTH] {
        &self.bytes
    }

    /// Get raw 16-byte array
    pub fn as_raw_bytes(&self) -> [u8; 16] {
        let mut raw = [0u8; 16];
        let _ = decode_crockford32(&self.bytes, &mut raw);
        raw
    }

    /// Extract timestamp (48 bits) - returns milliseconds since EPOCH (2020)
    pub fn timestamp(&self) -> u64 {
        let raw = self.as_raw_bytes();
        ((raw[0] as u64) << 40)
            | ((raw[1] as u64) << 32)
            | ((raw[2] as u64) << 24)
            | ((raw[3] as u64) << 16)
            | ((raw[4] as u64) << 8)
            | (raw[5] as u64)
    }

    /// Extract timestamp in milliseconds since UNIX_EPOCH (1970)
    pub fn timestamp_ms(&self) -> u64 {
        self.timestamp() + EPOCH
    }

    /// Extract counter (14 bits) - единая схема
    pub fn counter(&self) -> u16 {
        let raw = self.as_raw_bytes();
        let high = (raw[8] as u16) << 6;
        let low = (raw[9] >> 2) as u16;
        high | low
    }

    /// Extract random (50 bits) - единая схема
    pub fn random(&self) -> u64 {
        let raw = self.as_raw_bytes();
        let b9 = (raw[9] & 0x03) as u64;

        (b9 << 48)
            | ((raw[10] as u64) << 40)
            | ((raw[11] as u64) << 32)
            | ((raw[12] as u64) << 24)
            | ((raw[13] as u64) << 16)
            | ((raw[14] as u64) << 8)
            | (raw[15] as u64)
    }

    /// Extract worker ID (16 bits)
    pub fn worker_id(&self) -> u16 {
        let raw = self.as_raw_bytes();
        ((raw[6] as u16) << 8) | (raw[7] as u16)
    }

    /// Check if ID is valid
    pub fn is_valid(&self) -> bool {
        for &b in &self.bytes {
            match b {
                b'0'..=b'9' => {}
                b'A'..=b'Z' => {
                    let mut found = false;
                    for &ch in ALPHABET {
                        if ch == b {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }
}

impl fmt::Display for SigId26 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = core::str::from_utf8(&self.bytes).map_err(|_| fmt::Error)?;
        f.write_str(s)
    }
}

impl fmt::Debug for SigId26 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SigId26(")?;
        fmt::Display::fmt(self, f)?;
        write!(f, ")")
    }
}

impl FromStr for SigId26 {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cleaned = s.trim();

        if cleaned.len() != ID_LENGTH {
            return Err(Error::InvalidLength);
        }

        let mut raw = [0u8; 16];
        decode_crockford32(cleaned.as_bytes(), &mut raw)?;

        let mut encoded = [0u8; 26];
        encode_crockford32(&raw, &mut encoded);

        Ok(Self { bytes: encoded })
    }
}

impl AsRef<[u8]> for SigId26 {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_raw_new_simple() {
        let ts = 1234567890;
        let counter = 42;
        let random = 0x123456789abcd;

        let id = SigId26::raw_new(ts, counter, random);
        assert_eq!(id.timestamp(), ts, "timestamp mismatch");
        assert_eq!(id.worker_id(), 0, "worker_id should be 0 for Simple");
        assert_eq!(id.counter(), counter, "counter mismatch");
        assert_eq!(id.random(), random, "random mismatch");
    }

    #[test]
    fn test_raw_new_with_worker() {
        let ts = 1234567890;
        let worker = 0x1234;
        let counter = 42;
        let random = 0x123456789abcd;

        let id = SigId26::raw_new_with_worker(ts, worker, counter, random);

        assert_eq!(id.timestamp(), ts, "timestamp mismatch");
        assert_eq!(id.worker_id(), worker, "worker_id mismatch");
        assert_eq!(id.counter(), counter, "counter mismatch");
        assert_eq!(id.random(), random, "random mismatch");
    }

    #[test]
    fn test_worker_id_bounds() {
        let id = SigId26::raw_new_with_worker(1234567890, u16::MAX, 0, 0);
        assert_eq!(id.worker_id(), u16::MAX);
    }

    #[test]
    fn test_counter_bounds() {
        let id = SigId26::raw_new_with_worker(1234567890, 0, 16383, 0);
        assert_eq!(id.counter(), 16383);

        let id = SigId26::raw_new_with_worker(1234567890, 0, 16384, 0);
        assert_eq!(id.counter(), 0);
    }

    #[test]
    fn test_random_bounds() {
        let max_random = 0x3FFFFFFFFFFFF;
        let id = SigId26::raw_new_with_worker(1234567890, 0, 0, max_random);
        assert_eq!(id.random(), max_random);

        let id = SigId26::raw_new_with_worker(1234567890, 0, 0, 0xFFFFFFFFFFFFFFFF);
        assert_eq!(id.random(), 0x3FFFFFFFFFFFF);
    }

    #[test]
    fn test_from_str_roundtrip() {
        let id1 = SigId26::raw_new(1234567890, 42, 0x123456789abcd);
        let s = id1.to_string();
        let id2 = SigId26::from_str(&s).unwrap();
        assert_eq!(id1.as_bytes(), id2.as_bytes());
    }

    #[test]
    fn test_enterprise_roundtrip() {
        let ts = 1234567890;
        let worker = 0x1234;
        let counter = 42;
        let random = 0x123456789abcd;

        let id1 = SigId26::raw_new_with_worker(ts, worker, counter, random);
        let s = id1.to_string();
        let id2 = SigId26::from_str(&s).unwrap();

        assert_eq!(id1.as_bytes(), id2.as_bytes(), "bytes mismatch");
        assert_eq!(
            id2.worker_id(),
            worker,
            "worker_id mismatch after roundtrip"
        );
        assert_eq!(id2.counter(), counter, "counter mismatch after roundtrip");
        assert_eq!(id2.random(), random, "random mismatch after roundtrip");
    }

    #[test]
    fn test_all_max() {
        let max_random = 0x3FFFFFFFFFFFF;
        let id = SigId26::raw_new_with_worker(TIMESTAMP_MAX, u16::MAX, COUNTER_MAX, max_random);

        assert_eq!(id.timestamp(), TIMESTAMP_MAX);
        assert_eq!(id.worker_id(), u16::MAX);
        assert_eq!(id.counter(), COUNTER_MAX);
        assert_eq!(id.random(), max_random);
    }

    #[test]
    fn test_all_zero() {
        let id = SigId26::raw_new_with_worker(0, 0, 0, 0);

        assert_eq!(id.timestamp(), 0);
        assert_eq!(id.worker_id(), 0);
        assert_eq!(id.counter(), 0);
        assert_eq!(id.random(), 0);
    }

    #[test]
    fn test_simple_vs_enterprise_consistency() {
        // Simple версия (worker_id = 0)
        let simple = SigId26::raw_new(1234567890, 42, 0x123456789abcd);
        // Enterprise версия с worker_id = 0
        let enterprise = SigId26::raw_new_with_worker(1234567890, 0, 42, 0x123456789abcd);

        assert_eq!(
            simple.as_bytes(),
            enterprise.as_bytes(),
            "Simple and Enterprise with worker_id=0 should be identical"
        );
    }
}
