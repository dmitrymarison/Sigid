// SPDX-License-Identifier: MIT OR Apache-2.0

//! Flexible ID generator

use crate::Alphabet;
use sigid_core::{Error, Generator as CoreGenerator, SigId26};

/// Flexible ID generator builder with persistent state
#[derive(Debug, Clone)]
pub struct Generator {
    core: CoreGenerator,
    length: usize,
    alphabet: Alphabet,
    prefix: Option<&'static str>,
    include_checksum: bool,
}

impl Generator {
    /// Create new generator with default settings
    pub fn new() -> Self {
        Self {
            core: CoreGenerator::default(),
            length: 26,
            alphabet: Alphabet::default(),
            prefix: None,
            include_checksum: false,
        }
    }

    /// Create new generator with seed (builder pattern)
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.core = CoreGenerator::new(seed);
        self
    }

    /// Set worker ID (16 bits)
    pub fn worker_id(mut self, worker_id: u16) -> Self {
        self.core = self.core.set_worker_id(worker_id);
        self
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

    /// Generate raw SigId26 (без форматирования) - для enterprise
    pub fn generate_raw(&mut self, ms: u64) -> Result<SigId26, Error> {
        self.core.generate(ms)
    }

    /// Generate ID from timestamp
    #[cfg(feature = "std")]
    pub fn generate(&mut self, ms: u64) -> Result<String, Error> {
        let id = self.core.generate(ms)?;
        self.format_id(&id)
    }

    /// Generate ID from timestamp (no_std version)
    #[cfg(not(feature = "std"))]
    pub fn generate(&mut self, ms: u64) -> Result<alloc::string::String, Error> {
        use alloc::string::String;
        let id = self.core.generate(ms)?;
        self.format_id(&id)
    }

    /// Format ID: trim, apply alphabet, checksum, prefix
    fn format_id(&self, id: &SigId26) -> Result<String, Error> {
        let id_str = id.to_string();
        let trimmed = if id_str.len() > self.length {
            &id_str[..self.length]
        } else {
            &id_str
        };

        let result = if self.alphabet != Alphabet::Crockford32 {
            self.apply_alphabet(trimmed)?
        } else {
            trimmed.to_string()
        };

        let result = if self.include_checksum {
            crate::add_checksum(&result)
        } else {
            result
        };

        if let Some(prefix) = self.prefix {
            Ok(format!("{}{}", prefix, result))
        } else {
            Ok(result)
        }
    }

    /// Apply custom alphabet to ID (оптимизированная версия)
    fn apply_alphabet(&self, input: &str) -> Result<String, Error> {
        let alphabet = self.alphabet.chars();
        let base = alphabet.len();

        // Если алфавит совпадает с Crockford32, возвращаем как есть
        if base == 32 && self.alphabet == Alphabet::Crockford32 {
            return Ok(input.to_string());
        }

        // Декодируем Crockford32 в 16 байт
        let mut bytes = [0u8; 16];
        if sigid_core::decode_crockford32(input.as_bytes(), &mut bytes).is_err() {
            return Err(Error::InvalidCharacter);
        }

        let value = u128::from_be_bytes(bytes);
        let chars: Vec<char> = alphabet.chars().collect();

        // Для алфавитов с размером степени двойки - используем битовые операции
        let is_power_of_two = base & (base - 1) == 0;
        if is_power_of_two && base <= 128 {
            return self.apply_alphabet_power_of_two(value, &chars, base);
        }

        // Для произвольных алфавитов - используем оптимизированный цикл
        self.apply_alphabet_general(value, &chars, base)
    }

    /// Вычисляет целевую длину для данного алфавита
    fn get_target_length(&self, base: usize) -> usize {
        // Если пользователь явно задал длину (не 26), используем её
        if self.length != 26 {
            return self.length;
        }

        // Иначе рассчитываем оптимальную длину для данного алфавита
        // Формула: ceil(log_base(2^128)) = ceil(128 / log2(base))
        match base {
            2 => 128, // Binary
            10 => 39, // Digits: ceil(128 / 3.3219) = 39
            16 => 32, // Hex: 128 / 4 = 32
            32 => 26, // Crockford32: 128 / 5 = 25.6 -> 26
            52 => 23, // Letters: ceil(128 / 5.7004) = 23
            62 => 22, // Alphanumeric: ceil(128 / 5.9542) = 22
            64 => 22, // Base64: 128 / 6 = 21.3 -> 22
            _ => {
                // Для произвольного алфавита рассчитываем минимальную длину
                // Минимальная длина = ceil(log_base(2^128))
                let bits_per_char = (base as f64).log2();
                let len = (128.0 / bits_per_char).ceil() as usize;
                len.max(1)
            }
        }
    }

    /// Оптимизированная версия для алфавитов с размером степени двойки
    fn apply_alphabet_power_of_two(
        &self,
        mut value: u128,
        chars: &[char],
        base: usize,
    ) -> Result<String, Error> {
        let shift = base.trailing_zeros();
        let mask = (base - 1) as u128;
        let target_len = self.get_target_length(base);

        let mut result = String::with_capacity(target_len);

        for _ in 0..target_len {
            let idx = (value & mask) as usize;
            result.push(chars[idx % chars.len()]);
            value >>= shift;
        }

        Ok(result.chars().rev().collect())
    }

    /// Общая версия для произвольных алфавитов
    fn apply_alphabet_general(
        &self,
        mut value: u128,
        chars: &[char],
        base: usize,
    ) -> Result<String, Error> {
        let base_u128 = base as u128;
        let target_len = self.get_target_length(base);

        let mut result = String::with_capacity(target_len);

        for _ in 0..target_len {
            let idx = (value % base_u128) as usize;
            result.push(chars[idx % chars.len()]);
            value /= base_u128;
        }

        Ok(result.chars().rev().collect())
    }

    /// Generate ID with current time (requires std)
    #[cfg(feature = "std")]
    pub fn now(&mut self) -> Result<String, Error> {
        use std::time::{SystemTime, UNIX_EPOCH};
        let ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        self.generate(ms)
    }

    /// Get current worker ID
    pub fn get_worker_id(&self) -> u16 {
        self.core.get_worker_id()
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
        let mut gen = Generator::new();
        let id = gen.generate(1234567890).unwrap();
        assert_eq!(id.len(), 26);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_generator_custom_length() {
        let mut gen = Generator::new().length(20);
        let id = gen.generate(1234567890).unwrap();
        assert_eq!(id.len(), 20);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_generator_with_prefix() {
        let mut gen = Generator::new().prefix("user_");
        let id = gen.generate(1234567890).unwrap();
        assert!(id.starts_with("user_"));
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_generator_with_worker_id() {
        let mut gen = Generator::new().worker_id(0x1234);
        assert_eq!(gen.get_worker_id(), 0x1234);
        let id = gen.generate(1234567890).unwrap();
        assert_eq!(id.len(), 26);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_generate_raw() {
        let mut gen = Generator::new();
        let id = gen.generate_raw(1234567890).unwrap();
        assert!(id.is_valid());
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_apply_alphabet_crockford() {
        let gen = Generator::new().alphabet(Alphabet::Crockford32);
        let mut core_gen = CoreGenerator::new(0x123456789abcdef);
        let id = core_gen.generate(1234567890).unwrap();
        let id_str = id.to_string();

        let result = gen.apply_alphabet(&id_str).unwrap();
        assert_eq!(result.len(), 26);
        let valid_chars = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
        for c in result.chars() {
            assert!(valid_chars.contains(c), "Invalid char: {}", c);
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_apply_alphabet_base64() {
        let gen = Generator::new().alphabet(Alphabet::Base64);
        let mut core_gen = CoreGenerator::new(0x123456789abcdef);
        let id = core_gen.generate(1234567890).unwrap();
        let id_str = id.to_string();

        let result = gen.apply_alphabet(&id_str).unwrap();
        assert_eq!(result.len(), 22);
        let valid_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
        for c in result.chars() {
            assert!(valid_chars.contains(c), "Invalid char: {}", c);
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_apply_alphabet_digits() {
        let gen = Generator::new().alphabet(Alphabet::Digits);
        let mut core_gen = CoreGenerator::new(0x123456789abcdef);
        let id = core_gen.generate(1234567890).unwrap();
        let id_str = id.to_string();

        let result = gen.apply_alphabet(&id_str).unwrap();
        assert_eq!(result.len(), 39);
        for c in result.chars() {
            assert!(c.is_ascii_digit(), "Invalid char: {}", c);
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_apply_alphabet_digits_with_length() {
        let gen = Generator::new().alphabet(Alphabet::Digits).length(12);
        let mut core_gen = CoreGenerator::new(0x123456789abcdef);
        let id = core_gen.generate(1234567890).unwrap();
        let id_str = id.to_string();

        let result = gen.apply_alphabet(&id_str).unwrap();
        assert_eq!(result.len(), 12);
        for c in result.chars() {
            assert!(c.is_ascii_digit(), "Invalid char: {}", c);
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_apply_alphabet_hex() {
        let gen = Generator::new().alphabet(Alphabet::Hex);
        let mut core_gen = CoreGenerator::new(0x123456789abcdef);
        let id = core_gen.generate(1234567890).unwrap();
        let id_str = id.to_string();

        let result = gen.apply_alphabet(&id_str).unwrap();
        assert_eq!(result.len(), 32);
        let valid_chars = "0123456789ABCDEF";
        for c in result.chars() {
            assert!(valid_chars.contains(c), "Invalid char: {}", c);
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_apply_alphabet_letters() {
        let gen = Generator::new().alphabet(Alphabet::Letters);
        let mut core_gen = CoreGenerator::new(0x123456789abcdef);
        let id = core_gen.generate(1234567890).unwrap();
        let id_str = id.to_string();

        let result = gen.apply_alphabet(&id_str).unwrap();
        assert_eq!(result.len(), 23);
        for c in result.chars() {
            assert!(c.is_ascii_alphabetic(), "Invalid char: {}", c);
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_apply_alphabet_alphanumeric() {
        let gen = Generator::new().alphabet(Alphabet::Alphanumeric);
        let mut core_gen = CoreGenerator::new(0x123456789abcdef);
        let id = core_gen.generate(1234567890).unwrap();
        let id_str = id.to_string();

        let result = gen.apply_alphabet(&id_str).unwrap();
        assert_eq!(result.len(), 22);
        for c in result.chars() {
            assert!(c.is_ascii_alphanumeric(), "Invalid char: {}", c);
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_with_seed() {
        let mut gen = Generator::new().with_seed(0x123456789abcdef);
        let id = gen.generate(1234567890).unwrap();
        assert_eq!(id.len(), 26);
    }
}
