// SPDX-License-Identifier: MIT OR Apache-2.0

//! Prefix handling

use core::fmt;

/// Prefix for ID (compile-time or runtime)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Prefix<const P: &'static str>;

impl<const P: &'static str> Prefix<P> {
    /// Get prefix string
    pub const fn as_str(&self) -> &'static str {
        P
    }

    /// Get prefix length
    pub const fn len(&self) -> usize {
        P.len()
    }

    /// Check if prefix is empty
    pub const fn is_empty(&self) -> bool {
        P.is_empty()
    }
}

impl<const P: &'static str> fmt::Display for Prefix<P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(P)
    }
}

/// Runtime prefix (allocates)
#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimePrefix {
    inner: String,
}

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
impl fmt::Display for RuntimePrefix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner)
    }
}