// SPDX-License-Identifier: MIT OR Apache-2.0

//! # SigID Flexible
//!
//! Configurable ID generator with custom:
//! - Length (12-64 characters)
//! - Alphabet (Crockford32, Base64, custom)
//! - Prefixes (compile-time or runtime)
//! - Checksum (optional)
//!
//! # Example
//! ```
//! use sigid_flexible::{Generator, Alphabet};
//!
//! let mut gen = Generator::new()
//!     .length(24)
//!     .alphabet(Alphabet::Crockford32)
//!     .build();
//!
//! let id = gen.generate(1234567890).unwrap();
//! println!("{}", id);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs, missing_debug_implementations)]

extern crate alloc;

mod alphabet;
mod checksum;
mod generator;

pub use alphabet::Alphabet;
pub use checksum::{add_checksum, verify_checksum};
pub use generator::Generator;

/// Re-export core types
pub use sigid_core::{Error, Result, SigId26, COUNTER_MAX, EPOCH};

#[cfg(feature = "std")]
pub mod prefix {
    //! Prefix handling (std only)
    use std::string::String;

    /// Runtime prefix
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct RuntimePrefix {
        inner: String,
    }

    impl RuntimePrefix {
        /// Create new runtime prefix
        pub fn new(s: impl Into<String>) -> Self {
            Self { inner: s.into() }
        }

        /// Get prefix string
        pub fn as_str(&self) -> &str {
            &self.inner
        }

        /// Get prefix length
        pub fn len(&self) -> usize {
            self.inner.len()
        }

        /// Check if prefix is empty
        pub fn is_empty(&self) -> bool {
            self.inner.is_empty()
        }
    }

    impl core::fmt::Display for RuntimePrefix {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str(&self.inner)
        }
    }
}
