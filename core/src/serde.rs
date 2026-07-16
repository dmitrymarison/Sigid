// SPDX-License-Identifier: MIT OR Apache-2.0

//! Serde support for SigId26

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize, Serializer, Deserializer};
#[cfg(feature = "serde")]
use crate::SigId26;
#[cfg(feature = "serde")]
use core::str::FromStr;
#[cfg(feature = "serde")]
use alloc::string::String;
#[cfg(feature = "serde")]
use alloc::string::ToString;

#[cfg(feature = "serde")]
impl Serialize for SigId26 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for SigId26 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}