// SPDX-License-Identifier: MIT OR Apache-2.0

//! Crockford Base32 encoding/decoding

use crate::Error;
use crate::ALPHABET;

/// Encode 16 bytes to 26 Crockford Base32 characters
pub fn encode_crockford32(raw: &[u8; 16], out: &mut [u8; 26]) {
    let n = u128::from_be_bytes(*raw);

    let mut value = n;
    for i in (0..26).rev() {
        out[i] = ALPHABET[(value & 0x1F) as usize];
        value >>= 5;
    }
}

/// Decode 26 Crockford Base32 characters to 16 bytes
pub fn decode_crockford32(input: &[u8], out: &mut [u8; 16]) -> Result<(), Error> {
    if input.len() != 26 {
        return Err(Error::InvalidLength);
    }

    let mut n: u128 = 0;
    for &byte in input {
        let val = match byte {
            // Опечатки (Crockford нормализация)
            b'0' | b'O' | b'o' => 0,
            b'1' | b'I' | b'i' | b'L' | b'l' => 1,

            // Прямые цифры
            b'2'..=b'9' => (byte - b'0') as u128,

            // Буквы с учетом пропущенных (I, L, O, U)
            b'A' | b'a' => 10,
            b'B' | b'b' => 11,
            b'C' | b'c' => 12,
            b'D' | b'd' => 13,
            b'E' | b'e' => 14,
            b'F' | b'f' => 15,
            b'G' | b'g' => 16,
            b'H' | b'h' => 17,
            b'J' | b'j' => 18, // I пропущена
            b'K' | b'k' => 19,
            b'M' | b'm' => 20, // L пропущена
            b'N' | b'n' => 21,
            b'P' | b'p' => 22, // O пропущена
            b'Q' | b'q' => 23,
            b'R' | b'r' => 24,
            b'S' | b's' => 25,
            b'T' | b't' => 26,
            b'V' | b'v' => 27, // U пропущена
            b'W' | b'w' => 28,
            b'X' | b'x' => 29,
            b'Y' | b'y' => 30,
            b'Z' | b'z' => 31,
            _ => return Err(Error::InvalidCharacter),
        };

        n = (n << 5) | val;
    }

    *out = n.to_be_bytes();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_decode_roundtrip() {
        let original = [
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD,
            0xEE, 0xFF,
        ];

        let mut encoded = [0u8; 26];
        encode_crockford32(&original, &mut encoded);

        let mut decoded = [0u8; 16];
        let result = decode_crockford32(&encoded, &mut decoded);
        assert!(result.is_ok(), "Decode failed: {:?}", result);
        assert_eq!(original, decoded);
    }

    #[test]
    fn test_encode_known() {
        let input = [0u8; 16];
        let mut output = [0u8; 26];
        encode_crockford32(&input, &mut output);

        let s = core::str::from_utf8(&output).unwrap();
        assert_eq!(s, "00000000000000000000000000");
    }

    #[test]
    fn test_decode_known() {
        let input = b"00000000000000000000000000";
        let mut output = [0u8; 16];
        let result = decode_crockford32(input, &mut output);
        assert!(result.is_ok());
        assert_eq!(output, [0u8; 16]);
    }

    #[test]
    fn test_encode_specific() {
        let input = [
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E,
            0x0F, 0x10,
        ];
        let mut output = [0u8; 26];
        encode_crockford32(&input, &mut output);

        let mut decoded = [0u8; 16];
        let result = decode_crockford32(&output, &mut decoded);
        assert!(result.is_ok());
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_crockford_alphabet() {
        let alphabet = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

        for &c in alphabet {
            let input = [c; 26];
            let mut output = [0u8; 16];
            let result = decode_crockford32(&input, &mut output);
            assert!(result.is_ok(), "Failed to decode char: {}", c as char);
        }
    }

    #[test]
    fn test_decode_lenient() {
        // Проверяем опечатки - создаем строку из одного символа повторенного 26 раз
        let test_chars = [b'O', b'I', b'L', b'B'];

        for &c in &test_chars {
            let input = [c; 26];
            let mut output = [0u8; 16];
            let result = decode_crockford32(&input, &mut output);
            assert!(result.is_ok(), "Failed to decode char: {}", c as char);
        }
    }
}
