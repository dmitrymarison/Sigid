// SPDX-License-Identifier: MIT OR Apache-2.0

//! Checksum utilities for flexible generator

pub use sigid_core::iso7064_checksum;

/// Add checksum to ID (alloc version for no_std)
#[cfg(feature = "std")]
pub fn add_checksum(id: &str) -> String {
    let checksum = iso7064_checksum(id.as_bytes());
    format!("{}{}", id, checksum as char)
}

/// Add checksum to ID (no_std version)
#[cfg(not(feature = "std"))]
pub fn add_checksum(id: &str) -> alloc::string::String {
    use alloc::string::String;
    let checksum = iso7064_checksum(id.as_bytes());
    format!("{}{}", id, checksum as char)
}

/// Verify checksum
pub fn verify_checksum(id: &str) -> bool {
    if id.len() < 2 {
        return false;
    }
    let (body, checksum) = id.split_at(id.len() - 1);
    let expected = iso7064_checksum(body.as_bytes());
    expected as char == checksum.chars().next().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "std")]
    fn test_checksum_roundtrip() {
        let id = "0123456789ABCDEFGHJKMNPQRSTVWXYZ";
        let with_cs = add_checksum(id);
        assert_eq!(with_cs.len(), id.len() + 1);
        assert!(verify_checksum(&with_cs));
    }
}
