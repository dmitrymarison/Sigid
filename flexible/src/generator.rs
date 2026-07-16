// SPDX-License-Identifier: MIT OR Apache-2.0

//! Flexible ID generator

use crate::Alphabet;
use sigid_core::{Generator as CoreGenerator};

/// Flexible ID generator builder
#[derive(Debug, Clone)]
pub struct Generator {
    length: usize,
    alphabet: Alphabet,
    prefix: Option<&'static str>,
    include_checksum: bool,
}

impl Generator {
    /// Create new generator with default settings
    pub fn new() -> Self {
        Self {
            length: 26,
            alphabet: Alphabet::default(),
            prefix: None,
            include_checksum: false,
        }
    }

    /// Set ID length (default: 26)
    pub fn length(mut self, length: usize) -> Self {
        self.length = length.clamp(12, 64);
        self
    }

    /// Set alphabet (default: Crockford32)
    pub fn alphabet(mut self, alphabet: Alphabet) -> Self {
        self.alphabet = alphabet;
        self
    }

    /// Set prefix (compile-time) - requires std
    #[cfg(feature = "std")]
    pub fn prefix(mut self, prefix: &'static str) -> Self {
        self.prefix = Some(prefix);
        self
    }

    /// Enable checksum
    pub fn with_checksum(mut self, enabled: bool) -> Self {
        self.include_checksum = enabled;
        self
    }

    /// Build the generator (consumes builder)
    pub fn build(self) -> Self {
        self
    }

    /// Generate ID from timestamp
    #[cfg(feature = "std")]
    pub fn generate(&self, ms: u64) -> String {
        let mut core_gen = CoreGenerator::new(0x9e3779b97f4a7c15);
        let id = core_gen.generate(ms);
        
        let id_str = id.to_string();
        let trimmed = if id_str.len() > self.length {
            &id_str[..self.length]
        } else {
            &id_str
        };

        // Apply custom alphabet
        let result = if self.alphabet != Alphabet::Crockford32 {
            self.apply_alphabet(trimmed)
        } else {
            trimmed.to_string()
        };

        // Add checksum if enabled
        let result = if self.include_checksum {
            crate::add_checksum(&result)
        } else {
            result
        };

        // Add prefix
        if let Some(prefix) = self.prefix {
            format!("{}{}", prefix, result)
        } else {
            result
        }
    }

    /// Generate ID from timestamp (no_std version)
    #[cfg(not(feature = "std"))]
    pub fn generate(&self, ms: u64) -> alloc::string::String {
        use alloc::string::String;
        use alloc::string::ToString;
        
        let mut core_gen = CoreGenerator::new(0x9e3779b97f4a7c15);
        let id = core_gen.generate(ms);
        
        let id_str = id.to_string();
        let trimmed = if id_str.len() > self.length {
            &id_str[..self.length]
        } else {
            &id_str
        };

        // Apply custom alphabet
        let result = if self.alphabet != Alphabet::Crockford32 {
            self.apply_alphabet(trimmed)
        } else {
            trimmed.to_string()
        };

        // Add checksum if enabled
        let result = if self.include_checksum {
            crate::add_checksum(&result)
        } else {
            result
        };

        // Add prefix
        if let Some(prefix) = self.prefix {
            format!("{}{}", prefix, result)
        } else {
            result
        }
    }

    /// Apply custom alphabet (placeholder)
    fn apply_alphabet(&self, input: &str) -> String {
        // TODO: Implement alphabet conversion
        // For now, just return as-is
        input.to_string()
    }

    /// Generate ID with current time (requires std)
    #[cfg(feature = "std")]
    pub fn now(&self) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        self.generate(ms)
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
    #[cfg(feature = "std")]
    fn test_generator_default() {
        let gen = Generator::new();
        let id = gen.generate(1234567890);
        assert_eq!(id.len(), 26);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_generator_custom_length() {
        let gen = Generator::new().length(20);
        let id = gen.generate(1234567890);
        assert_eq!(id.len(), 20);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_generator_with_prefix() {
        let gen = Generator::new().prefix("user_");
        let id = gen.generate(1234567890);
        assert!(id.starts_with("user_"));
    }
}