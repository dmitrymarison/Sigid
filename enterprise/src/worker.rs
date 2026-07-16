// SPDX-License-Identifier: MIT OR Apache-2.0

//! Worker ID management for distributed systems

use core::fmt;

/// 10-bit Worker ID (0-1023)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WorkerId(u16);

impl WorkerId {
    /// Maximum worker ID value (10 bits)
    pub const MAX: u16 = 0x3FF;

    /// Create new worker ID with validation
    pub fn new(id: u16) -> Option<Self> {
        if id <= Self::MAX {
            Some(Self(id))
        } else {
            None
        }
    }

    /// Create worker ID without validation (use with caution)
    pub const unsafe fn new_unchecked(id: u16) -> Self {
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

        // Try POD_NUM (Kubernetes)
        if let Ok(val) = env::var("POD_NUM") {
            if let Ok(id) = val.parse::<u16>() {
                return Self::new(id);
            }
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
        Self::new((hash & 0x3FF) as u16)
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
        Self::from_env()
            .or_else(Self::from_mac)
            .unwrap_or_else(|| {
                // Last resort: use random
                use std::collections::hash_map::RandomState;
                use std::hash::{BuildHasher, Hasher};
                let hash = RandomState::new().build_hasher().finish();
                unsafe { Self::new_unchecked((hash & 0x3FF) as u16) }
            })
    }

    /// Generate worker ID for no_std environments
    #[cfg(not(feature = "std"))]
    pub fn from_seed(seed: u64) -> Self {
        unsafe { Self::new_unchecked((seed & 0x3FF) as u16) }
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
        assert!(WorkerId::new(0).is_some());
        assert!(WorkerId::new(1023).is_some());
        assert!(WorkerId::new(1024).is_none());
        assert!(WorkerId::new(u16::MAX).is_none());
    }

    #[test]
    fn test_worker_id_from_u16() {
        let id = WorkerId::from(123);
        assert_eq!(id.value(), 123);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_worker_id_from_hostname() {
        let id = WorkerId::from_hostname("test-host-1");
        assert!(id.is_some());
        assert!(id.unwrap().value() <= 1023);
    }

    #[test]
    fn test_worker_id_value() {
        let id = WorkerId::new(42).unwrap();
        assert_eq!(id.value(), 42);
    }
}