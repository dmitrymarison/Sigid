// SPDX-License-Identifier: MIT OR Apache-2.0

use sigid_core::*;
use std::str::FromStr;

#[test]
fn test_generate_id() {
    let mut gen = Generator::new(0x123456789abcdef);
    let id = gen.generate(1234567890).unwrap();
    let s = id.to_string();
    assert_eq!(s.len(), 26);
    assert!(id.is_valid());
}

#[test]
fn test_parse_id() {
    let mut gen = Generator::new(0x123456789abcdef);
    let id1 = gen.generate(1234567890).unwrap();
    let s = id1.to_string();

    println!("Parsing ID: '{}' (length: {})", s, s.len());
    assert_eq!(s.len(), 26);

    let id2 = SigId26::from_str(&s).unwrap();
    assert_eq!(id1.as_bytes(), id2.as_bytes());
}

#[test]
fn test_timestamp() {
    let ts = 1234567890;
    let id = SigId26::raw_new(ts, 0, 0);
    assert_eq!(id.timestamp(), ts);
}

#[test]
fn test_counter_increment() {
    let id1 = SigId26::raw_new(1234567890, 64, 12345);
    let id2 = SigId26::raw_new(1234567890, 65, 12345);

    assert_eq!(id1.counter(), 64);
    assert_eq!(id2.counter(), 65);
}

#[test]
fn test_worker_id() {
    let id = SigId26::raw_new(1234567890, 0, 12345);
    assert_eq!(id.worker_id(), 0);
}

#[test]
fn test_different_timestamps() {
    let ts1 = 1234567890;
    let ts2 = 1234567891;

    let id1 = SigId26::raw_new(ts1, 0, 12345);
    let id2 = SigId26::raw_new(ts2, 0, 12345);

    assert_ne!(id1.timestamp(), id2.timestamp());
}

#[test]
fn test_id_format() {
    let id = SigId26::raw_new(1234567890, 1, 12345);
    let s = id.to_string();

    assert_eq!(s.len(), 26);

    let valid_chars = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
    for c in s.chars() {
        assert!(valid_chars.contains(c), "Invalid character: {}", c);
    }
}

#[test]
fn test_debug_encode() {
    let mut gen = Generator::new(0x123456789abcdef);
    let id = gen.generate(1234567890).unwrap();
    let s = id.to_string();
    assert_eq!(s.len(), 26);
    println!("Generated ID: {}", s);
}

#[test]
fn test_timestamp_roundtrip() {
    let ts = 1234567890;
    let id = SigId26::raw_new(ts, 0, 0);
    assert_eq!(id.timestamp(), ts);
}

#[test]
fn test_generator_timestamp() {
    let mut gen = Generator::new(0x123456789abcdef);
    let ms = 1700000000000;
    let id = gen.generate(ms).unwrap();
    assert_eq!(id.timestamp(), ms - EPOCH);
    assert_eq!(id.timestamp_ms(), ms);
}

#[test]
fn test_raw_new_roundtrip() {
    let ts = 1234567890;
    let counter = 42;
    let random = 0x123456789abcd;

    let id = SigId26::raw_new(ts, counter, random);

    assert_eq!(id.timestamp(), ts);
    assert_eq!(id.counter(), counter);
    assert_eq!(id.random(), random);
}

#[test]
fn test_no_collisions_single_thread() {
    let mut gen = Generator::new(0x123456789abcdef);
    let mut ids = std::collections::HashSet::new();
    let count = 100_000;

    for i in 0..count {
        let id = gen.generate(1234567890 + i / 1000).unwrap();
        let s = id.to_string();
        assert!(!ids.contains(&s), "Collision detected!");
        ids.insert(s);
    }
}

#[test]
fn test_counter_overflow() {
    let mut gen = Generator::new(0x123456789abcdef);

    // ФИКС: Берем время из далекого будущего (3000-й год),
    // чтобы живые часы компьютера (2026 год) гарантированно оказались в прошлом!
    // 3000-01-01 00:00:00 UTC в миллисекундах
    let static_ms = 32503680000000u64;

    // COUNTER_MAX = 16383. Делаем ровно COUNTER_MAX + 1 успешных генераций
    for i in 0..=COUNTER_MAX {
        let result = gen.generate(static_ms);
        assert!(result.is_ok(), "Unexpected error at iteration {}", i);
    }

    // Следующий вызов в ТУ ЖЕ миллисекунду ОБЯЗАН выдать CounterOverflow!
    // Так как SystemTime::now() вернет 2026 год, а self.last_ms равен 3000-му году,
    // условие "now_ms <= self.last_ms" внутри wait_for_next_ms сработает как true,
    // и генератор выдаст заветную ошибку CounterOverflow!
    let result = gen.generate(static_ms);

    assert!(
        result.is_err(),
        "Expected CounterOverflow error after {} generated IDs",
        COUNTER_MAX + 1
    );

    match result {
        Err(Error::CounterOverflow) => {
            // Ожидаемая ошибка - тест пройден!
        }
        _ => panic!("Expected CounterOverflow error, got {:?}", result),
    }
}
