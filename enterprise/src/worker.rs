// SPDX-License-Identifier: MIT OR Apache-2.0

//! Worker ID management for distributed systems
//!
//! # Note
//! Worker ID is 16-bit (0-65535) for Enterprise schema.
//! This allows up to 65,536 unique workers in a cluster.

use core::fmt;

/// 16-bit Worker ID (0-65535)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerId(u16);

impl WorkerId {
    /// Maximum worker ID value (16 bits)
    pub const MAX: u16 = 0xFFFF;

    /// Create new worker ID with validation
    pub fn new(id: u16) -> Option<Self> {
        Some(Self(id & Self::MAX))
    }

    /// Create worker ID without validation (use with caution)
    ///
    /// # Safety
    /// This function does not validate that `id` is within the valid range (0-65535).
    /// The caller must ensure that `id` is <= `WorkerId::MAX`.
    /// Using an invalid value may cause unexpected behavior in downstream systems.
    pub const unsafe fn new_unchecked(id: u16) -> Self {
        Self(id & Self::MAX)
    }

    /// Create worker ID from u16 (with mask)
    pub const fn from_u16(id: u16) -> Self {
        Self(id & Self::MAX)
    }

    /// Get raw value
    pub const fn value(&self) -> u16 {
        self.0
    }

    /// Get static worker ID from environment (std only)
    #[cfg(feature = "std")]
    pub fn from_env() -> Option<Self> {
        use std::env;

        // Try SEED_WORKER_ID first
        if let Ok(val) = env::var("SEED_WORKER_ID") {
            if let Ok(id) = val.parse::<u16>() {
                return Self::new(id);
            }
        }

        // Try POD_NAME (Kubernetes) - более стандартно чем POD_NUM
        if let Ok(pod) = env::var("POD_NAME") {
            return Self::from_hostname(&pod);
        }

        // Try HOSTNAME hash
        if let Ok(hostname) = env::var("HOSTNAME") {
            return Self::from_hostname(&hostname);
        }

        None
    }

    /// Generate worker ID from hostname hash (deterministic)
    #[cfg(feature = "std")]
    pub fn from_hostname(hostname: &str) -> Option<Self> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        hostname.hash(&mut hasher);
        let hash = hasher.finish();
        Self::new((hash & 0xFFFF) as u16)
    }

    /// Generate worker ID from MAC address (deterministic)
    #[cfg(feature = "std")]
    pub fn from_mac() -> Option<Self> {
        // Simplified: use hostname as fallback
        std::env::var("HOSTNAME")
            .ok()
            .and_then(|h| Self::from_hostname(&h))
    }

    /// Zero-config worker ID (tries env, then hostname, then random)
    #[cfg(feature = "std")]
    pub fn zero_config() -> Self {
        Self::from_env().or_else(Self::from_mac).unwrap_or_else(|| {
            // Last resort: use random
            use std::collections::hash_map::RandomState;
            use std::hash::{BuildHasher, Hasher};
            let hash = RandomState::new().build_hasher().finish();
            unsafe { Self::new_unchecked((hash & 0xFFFF) as u16) }
        })
    }

    /// Generate worker ID for no_std environments
    #[cfg(not(feature = "std"))]
    pub fn from_seed(seed: u64) -> Self {
        unsafe { Self::new_unchecked((seed & 0xFFFF) as u16) }
    }
}

impl fmt::Display for WorkerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<u16> for WorkerId {
    fn from(id: u16) -> Self {
        Self(id & Self::MAX)
    }
}

impl From<WorkerId> for u16 {
    fn from(id: WorkerId) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_worker_id_bounds() {
        assert_eq!(WorkerId::new(0).unwrap().value(), 0);
        assert_eq!(WorkerId::new(65535).unwrap().value(), 65535);

        let id = WorkerId::new(65535).unwrap();
        assert_eq!(id.value(), 65535);

        let id = WorkerId::from_u16(0xFFFF);
        assert_eq!(id.value(), 0xFFFF);
    }

    #[test]
    fn test_worker_id_from_u16() {
        let id = WorkerId::from(12345);
        assert_eq!(id.value(), 12345);
    }

    #[test]
    fn test_worker_id_from_u16_const() {
        let id = WorkerId::from_u16(0xFFFF);
        assert_eq!(id.value(), 0xFFFF);

        let id = WorkerId::from_u16(0xFFFF);
        assert_eq!(id.value(), 0xFFFF);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_worker_id_from_hostname() {
        let id = WorkerId::from_hostname("test-host-1");
        assert!(id.is_some());
    }

    #[test]
    fn test_worker_id_value() {
        let id = WorkerId::new(42).unwrap();
        assert_eq!(id.value(), 42);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_worker_id_zero_config() {
        let _id = WorkerId::zero_config();
        // u16 всегда <= 65535, проверка не нужна
    }
}
