# Sigid

[![Crates.io](https://img.shields.io/crates/v/sigid-core.svg)](https://crates.io/crates/sigid-core)
[![Documentation](https://docs.rs/sigid-core/badge.svg)](https://docs.rs/sigid-core)
[![License: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![no_std](https://img.shields.io/badge/no__std-compatible-brightgreen.svg)](https://docs.rs/sigid-core)

> **Sig**il + **ID** — Identificadores únicos con cero dependencias.

---

## Índice

- [Características](#-características)
- [Inicio Rápido](#-inicio-rápido)
- [Módulos](#-módulos)
- [Arquitectura](#-arquitectura)
- [Instalación](#-instalación)
- [Licencia](#-licencia)

---

## Características

- **🔰 Tres Capas**: Core ➔ Flexible ➔ Enterprise. Elija solo el nivel de abstracción que necesite.
- **⚡ Cero Dependencias**: El módulo `Core` no tiene dependencias externas y funciona de forma nativa en `no_std`.
- **🎯 26 Caracteres**: Codificado en formato **Crockford Base32**, altamente legible por humanos.
- **📦 128 Bits**: Estructura unificada de Marca de tiempo (48) + ID de nodo/Worker (16) + Contador (14) + Aleatorio (50).
- **🕐 Ordenable Cronológicamente**: Ordenable lexicográficamente por hora de creación (compatible con índices de bases de datos).
- **🔒 Seguro para Hilos**: Arquitectura lock-free que utiliza operaciones atómicas para un rendimiento extremo.
- **🌐 Listo para WASM**: Funciona sin configuración adicional en entornos de navegador mediante `getrandom/js`.
- **🔄 Tipado Seguro**: Encapsulado en el tipo especializado `SigId26` en lugar de cadenas de texto crudas e inseguras.
- **✅ Protección de Suma de Verificación**: Validación opcional según ISO 7064 Mod 37,36 con corrección automática de errores comunes de escritura (`O` ➔ `0`, `I`/`L` ➔ `1`).
- **🎨 Altamente Configurable**: Ajuste de longitud, alfabetos personalizados, prefijos de texto estáticos/dinámicos y componentes de diseño binario.

---

## Arquitectura

### Esquema de Bits (128 bits)

```text
┌────────────────────────┬──────────────────┬──────────────┬────────────────┐
│   Marca de tiempo(48)  │ ID de Worker(16) │ Contador(14) │  Aleatorio(50) │
└────────────────────────┴──────────────────┴──────────────┴────────────────┘
                                         │
                                         ▼
                           Crockford Base32 (26 caracteres)
                                         │
                                         ▼
                            "01GJ8X9Z000000000000000000"
```

### Estructura del Proyecto

```text
sigid/
├── core/          # Núcleo de la máquina de estados, no_std y cero dependencias
├── flexible/      # Personalizador de diseño (alfabetos, prefijos, sumas de control)
└── enterprise/    # Motor distribuido para clústeres con escalado en la nube
```

---

## Inicio Rápido

Añada el crate unificado a su `Cargo.toml`:

```toml
[dependencies]
sigid = "1.0.0"
```

### 1. Simple (Core)
```rust
use sigid_core::{Generator, SigId26};

fn main() {
    let mut gen = Generator::new(0x123456789abcdef);
    let id = gen.generate(1234567890).unwrap();
    println!("{}", id); // Imprime: 0004K5G2T8000FS803B5D7G1PG
}
```

### 2. Flexible (Flexible)
```rust
use sigid_flexible::{Generator, Alphabet};

fn main() {
    let mut gen = Generator::new()
        .length(24)
        .alphabet(Alphabet::Base64)
        .prefix("user_")
        .build();

    let id = gen.generate(1234567890).unwrap();
    println!("{}", id); // Imprime: user_...
}
```

### 3. Industrial (Enterprise)
```rust
use sigid_enterprise::{Generator, WorkerId, SigIdUuidExt};

fn main() {
    let mut gen = Generator::new()
        .with_worker_id(WorkerId::new(42).unwrap())
        .build();

    let id = gen.generate(1234567890).unwrap();
    
    // Conversión de coste cero al formato estándar UUIDv7
    let uuid = id.to_uuid(); 
}
```

---

## Especificación de Módulos

| Módulo | Descripción | Dependencias | Entorno |
| :--- | :--- | :---: | :---: |
| **`sigid-core`** | Núcleo ultrarrápido y minimalista | **0** | `no_std` / `WASM` |
| **`sigid-flexible`** | Constructor de alfabetos y prefijos personalizados | Opcional (`serde`) | `std` / `no_std` |
| **`sigid-enterprise`** | Motor nativo de la nube para clústeres | Opcional (`uuid`) | `std` |

---

## Instalación de Capas Específicas

### Solo Núcleo (Minimalismo absoluto):
```toml
[dependencies]
sigid-core = "1.0.0"
```

### Funciones de Personalización y Serialización:
```toml
[dependencies]
sigid-flexible = { version = "1.0.0", features = ["serde"] }
```

### Infraestructura en Clúster Enterprise:
```toml
[dependencies]
sigid-enterprise = { version = "1.0.0", features = ["full"] }
```

---

## Licencia

Este proyecto está bajo una licencia doble a su elección:
* **MIT License** ([LICENSE-MIT](LICENSE-MIT))
* **Apache License, Version 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

---

## Traducciones

* [Русский](README_RU.md)
* [English](README.md)
* [中文](README_ZH.md)

---

Si le gusta este proyecto, por favor apóyelo con una estrella en GitHub: [github.com/dmitrymarison/sigid](https://github.com/dmitrymarison/sigid)
