// core/tests/tests.rs

use std::str::FromStr;
use sigid_core::*;

#[test]
fn test_generate_id() {
    let mut gen = Generator::new(0x123456789abcdef);
    let id = gen.generate(1234567890);
    let s = id.to_string();
    assert_eq!(s.len(), 26);
    assert!(id.is_valid());
}

#[test]
fn test_parse_id() {
    let mut gen = Generator::new(0x123456789abcdef);
    let id1 = gen.generate(1234567890);
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
    // В Simple версии worker_id нет
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
    let id = gen.generate(1234567890);
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
    let ms = 1234567890;
    let id = gen.generate(ms);
    // Теперь генератор не вычитает EPOCH, поэтому timestamp = ms
    assert_eq!(id.timestamp(), ms);
}

#[test]
fn test_raw_new_roundtrip() {
    let ts = 1234567890;
    let counter = 42;
    let random = 0x123456789abcdef;
    
    let id = SigId26::raw_new(ts, counter, random);
    
    assert_eq!(id.timestamp(), ts);
    assert_eq!(id.counter(), counter);
    assert_eq!(id.random(), random);
}