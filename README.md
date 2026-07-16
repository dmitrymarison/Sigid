# Sigid

[![Crates.io](https://img.shields.io/crates/v/sigid-core.svg)](https://crates.io/crates/sigid-core)
[![Documentation](https://docs.rs/sigid-core/badge.svg)](https://docs.rs/sigid-core)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![no_std](https://img.shields.io/badge/no__std-compatible-brightgreen.svg)](https://docs.rs/sigid-core)

> **Sig**il + **ID** — Zero-dependency unique identifiers.

---

## Table of Contents

- [Features](#-features)
- [Quick Start](#-quick-start)
- [Modules](#-modules)
- [Architecture](#-architecture)
- [Installation](#-installation)
- [License](#-license)

---

## Features

- **🔰 Three Layers**: Core ➔ Flexible ➔ Enterprise. Choose only the abstraction level you need.
- **⚡ Zero Dependencies**: The `Core` crate has zero external dependencies and works natively in `no_std`.
- **🎯 26 Characters**: Encoded in human-readable **Crockford Base32** format.
- **📦 128 Bits**: Layout consists of Timestamp (48) + Worker ID (16) + Counter (14) + Random (50).
- **🕐 Chronologically Sortable**: Lexicographically sortable by time of creation (database index friendly).
- **🔒 Thread-Safe**: Lock-free architecture utilizing atomic operations for extreme throughput.
- **🌐 WASM Ready**: Works out-of-the-box in browser environments via `getrandom/js`.
- **🔄 Type-Safe**: Wrapped in a specialized `SigId26` type instead of unsafe raw strings.
- **✅ Checksum Protection**: Optional validation using ISO 7064 Mod 37,36 with auto-correction for human typing typos (`O` ➔ `0`, `I`/`L` ➔ `1`).
- **🎨 Highly Customizable**: Fine-tune length, custom alphabets, static/dynamic text prefixes, and raw layout components.

---

## Architecture

### Bit Layout (128 bits)

```text
┌────────────────────────┬──────────────────┬──────────────┬────────────────┐
│     Timestamp (48)     │  Worker ID (16)  │ Counter (14) │  Random (50)   │
└────────────────────────┴──────────────────┴──────────────┴────────────────┘
                                         │
                                         ▼
                           Crockford Base32 (26 characters)
                                         │
                                         ▼
                            "01GJ8X9Z000000000000000000"
```

### Directory Structure

```text
sigid/
├── core/          # Zero-dependency, no_std core state machine
├── flexible/      # Layout customizer (alphabets, prefixes, checksums)
└── enterprise/    # Distributed cluster-scale engine with cloud scaling
```

---

## Quick Start

Add the unified umbrella crate to your `Cargo.toml`:

```toml
[dependencies]
sigid = "1.0.0"
```

### 1. Simple (Core)
```rust
use sigid::core::{Generator, SigId26};

fn main() {
    let mut gen = Generator::new(0x123456789abcdef);
    let id = gen.generate(1234567890).unwrap();
    println!("{}", id); // Outputs: 0004K5G2T8000FS803B5D7G1PG
}
```

### 2. Flexible (Flexible)
```rust
use sigid::flexible::{Generator, Alphabet};

fn main() {
    let mut gen = Generator::new()
        .length(24)
        .alphabet(Alphabet::Base64)
        .prefix("user_")
        .build();

    let id = gen.generate(1234567890).unwrap();
    println!("{}", id); // Outputs: user_...
}
```

### 3. Industrial (Enterprise)
```rust
use sigid::enterprise::{Generator, WorkerId, SigIdUuidExt};

fn main() {
    let mut gen = Generator::new()
        .with_worker_id(WorkerId::new(42).unwrap())
        .build();

    let id = gen.generate(1234567890).unwrap();
    
    // Zero-cost conversion to standard UUIDv7 layout
    let uuid = id.to_uuid(); 
}
```

---

## Module Specification

| Module | Description | Dependencies | Environment |
| :--- | :--- | :---: | :---: |
| **`sigid-core`** | Minimalistic, ultra-fast core machine | **0** | `no_std` / `WASM` |
| **`sigid-flexible`** | Custom alphabets and text prefix builder | Optional (`serde`) | `std` / `no_std` |
| **`sigid-enterprise`** | Cloud-native clustered engine | Optional (`uuid`) | `std` |

---

## Layer Installation

### Core-Only (Absolute minimalism):
```toml
[dependencies]
sigid-core = "1.0.0"
```

### Customization and Serialization features:
```toml
[dependencies]
sigid-flexible = { version = "1.0.0", features = ["serde"] }
```

### Enterprise Clustered Infrastructure:
```toml
[dependencies]
sigid-enterprise = { version = "1.0.0", features = ["full"] }
```

---

## License

This project is dually licensed under your choice of:
* **MIT License** ([LICENSE-MIT](LICENSE-MIT))
* **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

---

## Translations

* [Русский](README.md)
* [Español](README_ES.md)
* [中文](README_ZH.md)

---

If you like this project, please give it a star on GitHub: [github.com/dmitrymarison/sigid](https://github.com/dmitrymarison/sigid)