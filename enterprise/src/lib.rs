// SPDX-License-Identifier: MIT OR Apache-2.0

//! # SigID Enterprise
//!
//! Enterprise-grade ID generator for distributed systems.

#![no_std]

pub use sigid_core::{SigId26, Error, Result};

/// Placeholder for enterprise generator
pub struct EnterpriseGenerator;

impl EnterpriseGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for EnterpriseGenerator {
    fn default() -> Self {
        Self::new()
    }
}