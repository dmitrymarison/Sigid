// SPDX-License-Identifier: MIT OR Apache-2.0

//! Type-safe ID wrapper

use core::fmt;
use core::str::FromStr;
use crate::{Error, ID_LENGTH, encode_crockford32, decode_crockford32, ALPHABET, EPOCH};

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

    /// Create from raw components (Simple схема: 48+16+64)
    pub fn raw_new(timestamp: u64, counter: u16, random: u64) -> Self {
        let mut raw = [0u8; 16];
        
        // Timestamp: 48 bits (bytes 0-5)
        raw[0] = (timestamp >> 40) as u8;
        raw[1] = (timestamp >> 32) as u8;
        raw[2] = (timestamp >> 24) as u8;
        raw[3] = (timestamp >> 16) as u8;
        raw[4] = (timestamp >> 8) as u8;
        raw[5] = timestamp as u8;
        
        // Counter: 16 bits (bytes 6-7)
        raw[6] = (counter >> 8) as u8;
        raw[7] = counter as u8;
        
        // Random: 64 bits (bytes 8-15)
        raw[8] = (random >> 56) as u8;
        raw[9] = (random >> 48) as u8;
        raw[10] = (random >> 40) as u8;
        raw[11] = (random >> 32) as u8;
        raw[12] = (random >> 24) as u8;
        raw[13] = (random >> 16) as u8;
        raw[14] = (random >> 8) as u8;
        raw[15] = random as u8;
        
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
        // timestamp() возвращает относительное время (от EPOCH)
        // Прибавляем EPOCH для получения абсолютного времени
        self.timestamp() + EPOCH
    }

    /// Extract counter (16 bits)
    pub fn counter(&self) -> u16 {
        let raw = self.as_raw_bytes();
        ((raw[6] as u16) << 8) | (raw[7] as u16)
    }

    /// Extract random (64 bits)
    pub fn random(&self) -> u64 {
        let raw = self.as_raw_bytes();
        ((raw[8] as u64) << 56)
            | ((raw[9] as u64) << 48)
            | ((raw[10] as u64) << 40)
            | ((raw[11] as u64) << 32)
            | ((raw[12] as u64) << 24)
            | ((raw[13] as u64) << 16)
            | ((raw[14] as u64) << 8)
            | (raw[15] as u64)
    }

    /// Extract worker_id (в Simple версии нет)
    pub fn worker_id(&self) -> u16 {
        0
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
        // Декодируем строку в 16 байт
        decode_crockford32(cleaned.as_bytes(), &mut raw)?;
        
        // Кодируем обратно в 26 символов для хранения
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