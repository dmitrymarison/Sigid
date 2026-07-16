# Sigid

[![Crates.io](https://img.shields.io/crates/v/sigid-core.svg)](https://crates.io/crates/sigid-core)
[![Documentation](https://docs.rs/sigid-core/badge.svg)](https://docs.rs/sigid-core)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![no_std](https://img.shields.io/badge/no__std-compatible-brightgreen.svg)](https://docs.rs/sigid-core)

> **Sig**il + **ID** — 零外部依赖的高性能唯一标识符生成器。

---

## 目录

- [核心特性](#-核心特性)
- [快速开始](#-快速开始)
- [模块规范](#-模块规范)
- [架构设计](#-架构设计)
- [安装指南](#-安装指南)
- [开源协议](#-开源协议)

---

## 核心特性

- **🔰 三层架构**: 提供 Core ➔ Flexible ➔ Enterprise 三个抽象层，随需选用。
- **⚡ 零外部依赖**: `Core` 核心库无任何外部依赖，原生支持 `no_std` 裸机和智能合约环境。
- **🎯 26位字符**: 采用人类易读的 **Crockford Base32** 编码格式。
- **📦 128位紧凑布局**: 由时间戳 (48位) + 节点 ID (16位) + 计数器 (14位) + 随机数 (50位) 组成。
- **🕐 按时间排序**: 字符自然支持字典序排列，对数据库索引极其友好。
- **🔒 线程安全**: 采用 Lock-free（无锁）架构及原子操作，具备极高的并发吞吐量。
- **🌐 WASM 支持**: 通过启用 `getrandom/js` 特性，原生支持在浏览器等 WebAssembly 环境中运行。
- **🔄 类型安全**: 使用专用的 `SigId26` 结构体进行封装，避免了使用原始字符串带来的逻辑漏洞。
- **✅ 容错校验**: 支持可选的 ISO 7064 Mod 37,36 校验码，解析时可自动纠正人类输入常见误（如 `O` ➔ `0`, `I`/`L` ➔ `1`）。
- **🎨 深度自定义**: 支持动态调节长度、自定义字母表、静态/动态文本前缀以及底层二进制字段的拆分。

---

## 架构设计

### 位宽布局 (128 bits)

```text
┌────────────────────────┬──────────────────┬──────────────┬────────────────┐
│       时间戳 (48)      │   节点编码 (16)  │ 计数器 (14)  │   随机数 (50)  │
└────────────────────────┴──────────────────┴──────────────┴────────────────┘
                                         │
                                         ▼
                           Crockford Base32 (26位字符)
                                         │
                                         ▼
                            "01GJ8X9Z000000000000000000"
```

### 项目目录结构

```text
sigid/
├── core/          # 零依赖、no_std 的核心状态机逻辑
├── flexible/      # 支持自定义字母表、前缀、校验和的定制化层
└── enterprise/    # 专为微服务集群设计的分布式高并发引擎
```

---

## 快速开始

在您的 `Cargo.toml` 中添加统一封装库：

```toml
[dependencies]
sigid = "1.0.0"
```

### 1. 基础版 (Core)
```rust
use sigid::core::{Generator, SigId26};

fn main() {
    let mut gen = Generator::new(0x123456789abcdef);
    let id = gen.generate(1234567890).unwrap();
    println!("{}", id); // 输出: 0004K5G2T8000FS803B5D7G1PG
}
```

### 2. 定制版 (Flexible)
```rust
use sigid::flexible::{Generator, Alphabet};

fn main() {
    let mut gen = Generator::new()
        .length(24)
        .alphabet(Alphabet::Base64)
        .prefix("user_")
        .build();

    let id = gen.generate(1234567890).unwrap();
    println!("{}", id); // 输出: user_...
}
```

### 3. 企业版 (Enterprise)
```rust
use sigid::enterprise::{Generator, WorkerId, SigIdUuidExt};

fn main() {
    let mut gen = Generator::new()
        .with_worker_id(WorkerId::new(42).unwrap())
        .build();

    let id = gen.generate(1234567890).unwrap();
    
    // 零成本（Zero-cost）直接转换为标准 UUIDv7 格式
    let uuid = id.to_uuid(); 
}
```

---

## 模块规范

| 模块名称 | 描述说明 | 依赖项数量 | 支持运行环境 |
| :--- | :--- | :---: | :---: |
| **`sigid-core`** | 极致精简、超高性能的核心逻辑 | **0** | `no_std` / `WASM` |
| **`sigid-flexible`** | 灵活的自定义前缀与字母表构造器 | 可选 (`serde`) | `std` / `no_std` |
| **`sigid-enterprise`** | 云原生分布式集群级别生成器 | 可选 (`uuid`) | `std` |

---

## 安装指定抽象层

### 仅引入核心核心库 (极致精简):
```toml
[dependencies]
sigid-core = "1.0.0"
```

### 引入定制化与序列化功能:
```toml
[dependencies]
sigid-flexible = { version = "1.0.0", features = ["serde"] }
```

### 引入企业级分布式基础设施引擎:
```toml
[dependencies]
sigid-enterprise = { version = "1.0.0", features = ["full"] }
```

---

## 开源协议

本项目采用双重开源协议授权，您可以自由选择其一：
* **MIT License** ([LICENSE-MIT](LICENSE-MIT))
* **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

---

## 其他语言版本

* [Русский](README_RU.md)
* [English](README.md)
* [Español](README_ES.md)

---

如果您喜欢这个项目，欢迎在 GitHub 上为它点亮一颗星星：[github.com/dmitrymarison/sigid](https://github.com/dmitrymarison/sigid)