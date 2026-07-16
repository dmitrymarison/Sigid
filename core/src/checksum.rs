// SPDX-License-Identifier: MIT OR Apache-2.0

//! Checksum utilities

/// Calculate CRC-16 for data
pub fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= (byte as u16) << 8;
        for _ in 0..8 {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}

/// Calculate ISO 7064 Mod 37,36 checksum (like ISBN)
pub fn iso7064_checksum(data: &[u8]) -> u8 {
    let mut sum: u16 = 0;
    for &byte in data {
        let val = if byte >= 10 {
            (byte - 10 + 36) as u16
        } else {
            byte as u16
        };
        sum = (sum + val) % 37;
        sum = (sum * 36) % 37;
    }
    sum = (sum * 36) % 37;

    if sum < 10 {
        sum as u8 + b'0'
    } else {
        sum as u8 - 10 + b'A'
    }
}
