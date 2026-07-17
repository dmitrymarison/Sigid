# Sigid

[![Crates.io](https://img.shields.io/crates/v/sigid-core.svg)](https://crates.io/crates/sigid-core)
[![Documentation](https://docs.rs/sigid-core/badge.svg)](https://docs.rs/sigid-core)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![no_std](https://img.shields.io/badge/no__std-compatible-brightgreen.svg)](https://docs.rs/sigid-core)

> **Sig**il + **ID** — Уникальные идентификаторы с нулевыми зависимостями

---

## Оглавление

- [Возможности](#-возможности)
- [Быстрый старт](#-быстрый-старт)
- [Модули](#-модули)
- [Архитектура](#-архитектура)
- [Установка](#-установка)
- [Лицензия](#-лицензия)

---

## Возможности

- **🔰 Три слоя**: Core → Flexible → Enterprise
- **⚡ Ноль зависимостей**: Core версия не имеет зависимостей, работает в `no_std`
- **🎯 26 символов**: Crockford Base32, читаемый человеком
- **📦 128 бит**: Время (48) + Worker ID (16) + Счетчик (14) + Случайное (50)
- **🕐 Сортировка по времени**: Лексикографически сортируемые по времени создания
- **🔒 Потокобезопасность**: Lock-free, атомарные операции
- **🌐 WASM Ready**: Работает в браузерах с `getrandom/js`
- **🔄 Типобезопасность**: `SigId26` вместо сырых строк
- **✅ Контрольная сумма**: Опциональная валидация ISO 7064 Mod 37,36
- **🎨 Гибкая настройка**: Длина, алфавит, префиксы, компоненты

---

## Быстрый старт

Добавьте в `Cargo.toml`:

```toml
[dependencies]
sigid = "1.0.0"

---

## Архитектура

### Битовая схема (128 бит)

```text
┌────────────────────────┬──────────────────┬──────────────┬────────────────┐
│     Timestamp (48)     │  Worker ID (16)  │ Counter (14) │  Random (50)   │
└────────────────────────┴──────────────────┴──────────────┴────────────────┘
                                         │
                                         ▼
                           Crockford Base32 (26 символов)
                                         │
                                         ▼
                            "01GJ8X9Z000000000000000000"
```

### Структура проекта

```text
sigid/
├── core/          # Zero-dependency, no_std ядро
├── flexible/      # Настраиваемый генератор рантайм-компонентов
└── enterprise/    # Промышленный распределенный движок с поддержкой кластеров
```

---


### 1. Простой (Core)
```rust
use sigid_core::{Generator, SigId26};

fn main() {
    let mut gen = Generator::new(0x123456789abcdef);
    let id = gen.generate(1234567890).unwrap();
    println!("{}", id); // Выведет: 0004K5G2T8000FS803B5D7G1PG
}
```

### 2. Гибкий (Flexible)
```rust
use sigid_flexible::{Generator, Alphabet};

fn main() {
    let mut gen = Generator::new()
        .length(24)
        .alphabet(Alphabet::Base64)
        .prefix("user_")
        .build();

    let id = gen.generate(1234567890).unwrap();
    println!("{}", id); // Выведет: user_...
}
```

### 3. Промышленный (Enterprise)
```rust
use sigid_enterprise::{Generator, WorkerId, SigIdUuidExt};

fn main() {
    let mut gen = Generator::new()
        .with_worker_id(WorkerId::new(42).unwrap())
        .build();

    let id = gen.generate(1234567890).unwrap();
    
    // Экспорт в системный формат UUIDv7 за 0 наносекунд
    let uuid = id.to_uuid(); 
}
```

---

## Спецификация модулей

| Модуль | Описание | Зависимости | Среда |
| :--- | :--- | :---: | :---: |
| **`sigid-core`** | Минималистичное, сверхбыстрое ядро | **0** | `no_std` / `WASM` |
| **`sigid-flexible`** | Конструктор кастомных алфавитов и префиксов | Опционально (`serde`) | `std` / `no_std` |
| **`sigid-enterprise`** | Отказоустойчивый кластерный генератор | Опционально (`uuid`) | `std` |

---

## Установка конкретных слоев

### Только ядро (Абсолютный минимализм):
```toml
[dependencies]
sigid-core = "1.0.0"
```

### С поддержкой кастомизации и сериализации:
```toml
[dependencies]
sigid-flexible = { version = "1.0.0", features = ["serde"] }
```

### Полная Enterprise-версия для облачной инфраструктуры:
```toml
[dependencies]
sigid-enterprise = { version = "1.0.0", features = ["full"] }
```

---

## Лицензия

Проект распространяется под двойной лицензией по вашему выбору:
* **MIT License** ([LICENSE-MIT](LICENSE-MIT))
* **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

---

## Переводы (Translations)

* [English](README.md)
* [Español](README_ES.md)
* [中文](README_ZH.md)

---

Если вам понравился проект, поставьте звезду на GitHub: [github.com/dmitrymarison/sigid](https://github.com/dmitrymarison/sigid)
