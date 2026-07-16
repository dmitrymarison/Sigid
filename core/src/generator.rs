// SPDX-License-Identifier: MIT OR Apache-2.0

//! ID generator
//!
//! # Security Warning
//!
//! The default `Xorshift64` PRNG used in this module is **NOT**
//! cryptographically secure. It is suitable for generating unique IDs
//! where predictability is not a security concern (e.g., database IDs,
//! log entries, session IDs with additional security layers).
//!
//! For security-sensitive applications (e.g., API keys, tokens,
//! password reset links), enable the `secure` feature which uses
//! `getrandom` for cryptographically secure seed generation.

use crate::{Error, SigId26, COUNTER_MAX, EPOCH, TIMESTAMP_MAX, WORKER_MAX};

/// ID generator as a pure state machine
#[derive(Debug, Clone)]
pub struct Generator {
    seed: u64,
    last_ms: u64,
    counter: u16,
    worker_id: u16,
}

impl Generator {
    /// Create new generator with seed
    pub const fn new(seed: u64) -> Self {
        // Защита от нулевого сида для Xorshift
        let seed = if seed == 0 { 0x9e3779b97f4a7c15 } else { seed };
        Self {
            seed,
            last_ms: 0,
            counter: 0,
            worker_id: 0,
        }
    }

    /// Create new generator with seed and worker_id
    pub const fn with_worker_id(seed: u64, worker_id: u16) -> Self {
        let seed = if seed == 0 { 0x9e3779b97f4a7c15 } else { seed };
        Self {
            seed,
            last_ms: 0,
            counter: 0,
            worker_id: worker_id & WORKER_MAX,
        }
    }

    /// Set worker ID (16 bits)
    pub const fn set_worker_id(mut self, worker_id: u16) -> Self {
        self.worker_id = worker_id & WORKER_MAX;
        self
    }

    /// Get current worker ID
    pub const fn get_worker_id(&self) -> u16 {
        self.worker_id
    }

    /// Reset counter
    pub fn reset(&mut self) {
        self.counter = 0;
        self.last_ms = 0;
    }

    /// Generate ID from timestamp (milliseconds since UNIX_EPOCH)
    pub fn generate(&mut self, ms: u64) -> Result<SigId26, Error> {
        let timestamp = ms.saturating_sub(EPOCH);

        // Проверяем переполнение timestamp
        if timestamp > TIMESTAMP_MAX {
            return Err(Error::TimestampOverflow);
        }

        // Update counter
        if ms == self.last_ms {
            // Проверяем достижение лимита ДО инкремента
            if self.counter >= COUNTER_MAX {
                return self.wait_for_next_ms(ms);
            }
            self.counter += 1;
        } else {
            self.last_ms = ms;
            self.counter = 0;
        }

        let random = self.next_random();

        // Используем единую схему с worker_id (48+16+14+50)
        Ok(SigId26::raw_new_with_worker(
            timestamp,
            self.worker_id,
            self.counter,
            random,
        ))
    }

    /// Wait for next millisecond or return error
    fn wait_for_next_ms(&mut self, _ms: u64) -> Result<SigId26, Error> {
        // В no_std окружении нет способа получить время
        #[cfg(not(feature = "std"))]
        {
            return Err(Error::CounterOverflow);
        }

        // Если включена std, пытаемся получить реальное время
        #[cfg(feature = "std")]
        {
            use std::time::{SystemTime, UNIX_EPOCH};

            let now_ms = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|_| Error::TimestampOverflow)?
                .as_millis() as u64;

            // Если живое время меньше или равно last_ms,
            // значит мы не можем перейти на следующую миллисекунду.
            // Возвращаем ошибку!
            if now_ms <= self.last_ms {
                return Err(Error::CounterOverflow);
            }

            // Время перешло на следующую миллисекунду - обновляем состояние
            self.last_ms = now_ms;
            self.counter = 0;
            let timestamp = now_ms.saturating_sub(EPOCH);
            let random = self.next_random();
            Ok(SigId26::raw_new_with_worker(
                timestamp,
                self.worker_id,
                self.counter,
                random,
            ))
        }
    }

    /// Xorshift64 random generator
    fn next_random(&mut self) -> u64 {
        let mut x = self.seed;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.seed = x;
        x
    }
}

impl Default for Generator {
    fn default() -> Self {
        Self::new(0x9e3779b97f4a7c15)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_worker_id() {
        let mut gen = Generator::with_worker_id(0x123456789abcdef, 0x1234);
        assert_eq!(gen.get_worker_id(), 0x1234);

        let id = gen.generate(1234567890).unwrap();
        assert_eq!(id.worker_id(), 0x1234);
    }

    #[test]
    fn test_generator_counter_increment() {
        let mut gen = Generator::new(0x123456789abcdef);
        let ms = 1234567890;

        let id1 = gen.generate(ms).unwrap();
        let id2 = gen.generate(ms).unwrap();

        assert_eq!(id2.counter(), id1.counter() + 1);
    }

    #[test]
    fn test_generator_timestamp() {
        let mut gen = Generator::new(0x123456789abcdef);
        let ms = 1700000000000;
        let id = gen.generate(ms).unwrap();
        assert_eq!(id.timestamp_ms(), ms);
    }

    #[test]
    fn test_counter_overflow() {
        let mut gen = Generator::new(0x123456789abcdef);

        // Берем время из ДАЛЕКОГО БУДУЩЕГО (3000 год),
        // чтобы системное время компьютера (2026 год) гарантированно было меньше!
        // 3000-01-01 00:00:00 UTC в миллисекундах
        let ms = 32503680000000u64;

        // COUNTER_MAX = 16383. Делаем ровно COUNTER_MAX + 1 успешных генераций
        for i in 0..=COUNTER_MAX {
            let result = gen.generate(ms);
            assert!(result.is_ok(), "Unexpected error at iteration {}", i);
        }

        // Следующий вызов в ТУ ЖЕ миллисекунду ОБЯЗАН выдать CounterOverflow,
        // так как SystemTime::now() вернет 2026 год, что меньше 3000-го года!
        let result = gen.generate(ms);

        assert!(
            result.is_err(),
            "Expected CounterOverflow error after {} generated IDs",
            COUNTER_MAX + 1
        );
        match result {
            Err(Error::CounterOverflow) => {}
            _ => panic!("Expected CounterOverflow error, got {:?}", result),
        }
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_generator_now() {
        let mut gen = Generator::new(0x123456789abcdef);
        let id = gen.generate(1700000000000).unwrap();
        assert!(id.is_valid());
    }
}
