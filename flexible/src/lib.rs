// SPDX-License-Identifier: MIT OR Apache-2.0

//! # SigID Flexible
//!
//! Configurable ID generator with custom length, alphabet, and prefixes.

#![no_std]

pub use sigid_core::{SigId26, Error, Result};

/// Placeholder for flexible generator
pub struct FlexibleGenerator;

impl FlexibleGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FlexibleGenerator {
    fn default() -> Self {
        Self::new()
    }
}