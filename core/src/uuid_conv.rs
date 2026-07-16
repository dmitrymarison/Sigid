// SPDX-License-Identifier: MIT OR Apache-2.0

//! UUID conversion support

#[cfg(feature = "uuid")]
use crate::SigId26;
#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cfg(feature = "uuid")]
impl From<SigId26> for Uuid {
    fn from(id: SigId26) -> Self {
        let raw = id.as_raw_bytes();
        Uuid::from_bytes(raw)
    }
}

#[cfg(feature = "uuid")]
impl TryFrom<Uuid> for SigId26 {
    type Error = crate::Error;

    fn try_from(uuid: Uuid) -> Result<Self, Self::Error> {
        let bytes = uuid.as_bytes();
        let mut raw = [0u8; 16];
        raw.copy_from_slice(bytes);
        Ok(SigId26::from_raw_bytes(raw))
    }
}

#[cfg(test)]
#[cfg(feature = "uuid")]
mod tests {
    use super::*;
    use crate::SigId26;

    #[test]
    fn test_uuid_roundtrip() {
        let original = SigId26::raw_new(1234567890, 42, 0x123456789abcdef);
        let uuid = Uuid::from(original);
        let converted = SigId26::try_from(uuid).unwrap();
        assert_eq!(original.as_bytes(), converted.as_bytes());
    }
}
