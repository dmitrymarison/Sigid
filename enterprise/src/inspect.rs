// SPDX-License-Identifier: MIT OR Apache-2.0

//! ID inspection and metadata extraction

use core::fmt;
use core::str::FromStr;
use sigid_core::SigId26;

/// Metadata extracted from an ID
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InspectResult {
    /// Raw ID string
    pub raw: String,
    /// Timestamp (milliseconds since UNIX_EPOCH)
    pub timestamp: u64,
    /// Worker ID (if present)
    pub worker_id: Option<u16>,
    /// Counter value
    pub counter: u16,
    /// Random part
    pub random: u64,
    /// Checksum (if present)
    pub checksum: Option<char>,
    /// Whether checksum is valid
    pub checksum_valid: bool,
    /// Whether ID has a prefix
    pub has_prefix: bool,
    /// Prefix (if present)
    pub prefix: Option<String>,
}

impl InspectResult {
    /// Format timestamp as human-readable (std only)
    #[cfg(feature = "std")]
    pub fn timestamp_human(&self) -> String {
        use std::time::{Duration, UNIX_EPOCH};
        let secs = self.timestamp / 1000;
        let nanos = (self.timestamp % 1000) * 1_000_000;
        let time = UNIX_EPOCH + Duration::new(secs, nanos as u32);
        format!("{:?}", time)
    }

    /// Get timestamp as milliseconds
    pub fn timestamp_ms(&self) -> u64 {
        self.timestamp
    }
}

impl fmt::Display for InspectResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "📋 ID Inspection Report")?;
        writeln!(f, "━━━━━━━━━━━━━━━━━━━━━━━━━━")?;
        writeln!(f, "Raw:     {}", self.raw)?;
        writeln!(f, "Prefix:  {}", self.prefix.as_deref().unwrap_or("None"))?;
        writeln!(f, "Timestamp: {} ms", self.timestamp)?;
        #[cfg(feature = "std")]
        writeln!(f, "         {}", self.timestamp_human())?;
        writeln!(
            f,
            "Worker:  {}",
            self.worker_id
                .map(|w| w.to_string())
                .unwrap_or_else(|| "None".to_string())
        )?;
        writeln!(f, "Counter: {}", self.counter)?;
        writeln!(f, "Random:  0x{:016X}", self.random)?;
        if let Some(ch) = self.checksum {
            writeln!(
                f,
                "Checksum: {} {}",
                ch,
                if self.checksum_valid { "✅" } else { "❌" }
            )?;
        }
        Ok(())
    }
}

/// Inspect an ID and extract all metadata
pub fn inspect(id: &str) -> Option<InspectResult> {
    // Parse ID
    let (prefix, body) = if let Some(pos) = id.rfind(['_', '-', '.']) {
        (Some(&id[..pos + 1]), &id[pos + 1..])
    } else {
        (None, id)
    };

    // Try to parse body as SigId26
    let parsed = match SigId26::from_str(body) {
        Ok(id) => id,
        Err(_) => return None,
    };

    // Check for checksum
    let (checksum, checksum_valid) = if body.len() > 1 {
        let maybe_cs = body.chars().last().unwrap();
        let body_without_cs = &body[..body.len() - 1];
        let expected = sigid_core::iso7064_checksum(body_without_cs.as_bytes());
        if expected as char == maybe_cs {
            (Some(maybe_cs), true)
        } else {
            (Some(maybe_cs), false)
        }
    } else {
        (None, true)
    };

    // Используем timestamp_ms() - она уже возвращает абсолютное время с EPOCH
    // НЕ прибавляем EPOCH повторно!
    let timestamp = parsed.timestamp_ms();

    Some(InspectResult {
        raw: id.to_string(),
        timestamp,
        worker_id: Some(parsed.worker_id()),
        counter: parsed.counter(),
        random: parsed.random(),
        checksum,
        checksum_valid,
        has_prefix: prefix.is_some(),
        prefix: prefix.map(|s| s.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sigid_core::Generator;
    use sigid_core::EPOCH;

    #[test]
    fn test_inspect() {
        let mut gen = Generator::new(0x123456789abcdef);
        let ms = 1700000000000;
        let id = gen.generate(ms).unwrap();
        let id_str = id.to_string();

        let result = inspect(&id_str).unwrap();
        assert_eq!(result.timestamp, ms);
        assert_eq!(result.prefix, None);
    }

    #[test]
    fn test_inspect_with_timestamp() {
        let timestamp = 1700000000000;
        let id = SigId26::raw_new_with_worker(timestamp - EPOCH, 42, 0, 0x123456789abcdef);
        let id_str = id.to_string();

        let result = inspect(&id_str).unwrap();
        assert_eq!(result.timestamp, timestamp);
        assert_eq!(result.worker_id, Some(42));
    }

    #[test]
    fn test_inspect_with_prefix() {
        let timestamp = 1700000000000;
        let id = SigId26::raw_new_with_worker(timestamp - EPOCH, 0, 0, 0);
        let id_str = format!("user_{}", id);
        let result = inspect(&id_str).unwrap();
        assert_eq!(result.prefix, Some("user_".to_string()));
        assert!(result.has_prefix);
    }

    #[test]
    fn test_inspect_roundtrip() {
        let mut gen = Generator::new(0x123456789abcdef);
        let ms = 1700000000000;
        let id = gen.generate(ms).unwrap();
        let id_str = id.to_string();

        let result = inspect(&id_str).unwrap();
        assert_eq!(result.timestamp, ms);
    }
}
