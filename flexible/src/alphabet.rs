// SPDX-License-Identifier: MIT OR Apache-2.0

//! Alphabet configuration

/// Supported alphabets for ID generation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alphabet {
    /// Crockford Base32 (default) - 0-9 A-Z without I,L,O,U
    #[default]
    Crockford32,
    /// Base64 URL-safe - A-Z a-z 0-9 -_
    Base64,
    /// Hexadecimal - 0-9 A-F
    Hex,
    /// Digits only - 0-9
    Digits,
    /// Letters only - A-Z a-z
    Letters,
    /// Alphanumeric - A-Z a-z 0-9
    Alphanumeric,
    /// Custom alphabet (static string)
    Custom(&'static str),
}

impl Alphabet {
    /// Get the alphabet string
    pub const fn chars(&self) -> &'static str {
        match self {
            Self::Crockford32 => "0123456789ABCDEFGHJKMNPQRSTVWXYZ",
            Self::Base64 => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
            Self::Hex => "0123456789ABCDEF",
            Self::Digits => "0123456789",
            Self::Letters => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
            Self::Alphanumeric => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            Self::Custom(s) => s,
        }
    }

    /// Get the length of the alphabet
    pub const fn len(&self) -> usize {
        self.chars().len()
    }

    /// Check if alphabet is empty
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if character is in alphabet
    pub const fn contains(&self, c: char) -> bool {
        let chars = self.chars().as_bytes();
        let mut i = 0;
        while i < chars.len() {
            if chars[i] == c as u8 {
                return true;
            }
            i += 1;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alphabet_chars() {
        assert_eq!(Alphabet::Crockford32.chars().len(), 32);
        assert_eq!(Alphabet::Base64.chars().len(), 64);
        assert_eq!(Alphabet::Hex.chars().len(), 16);
        assert_eq!(Alphabet::Digits.chars().len(), 10);
    }

    #[test]
    fn test_alphabet_contains() {
        let alpha = Alphabet::Crockford32;
        assert!(alpha.contains('0'));
        assert!(alpha.contains('A'));
        assert!(!alpha.contains('I')); // I excluded
        assert!(!alpha.contains('L')); // L excluded
        assert!(!alpha.contains('O')); // O excluded
        assert!(!alpha.contains('U')); // U excluded
    }

    #[test]
    fn test_default() {
        let alpha = Alphabet::default();
        assert!(matches!(alpha, Alphabet::Crockford32));
    }
}