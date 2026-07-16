// SPDX-License-Identifier: MIT OR Apache-2.0

//! UUID/ULID conversion (via extension traits)

use sigid_core::SigId26;
use uuid::Uuid;

/// Extension trait for SigID to UUID conversion
pub trait SigIdUuidExt {
    /// Convert SigID to UUID (zero-cost, binary compatible)
    fn to_uuid(&self) -> Uuid;

    /// Convert SigID to RFC 4122 compatible UUIDv7
    ///
    /// Устанавливает версию 7 и вариант RFC 4122 для совместимости.
    /// # Note
    /// При использовании этого метода теряется 6 бит случайности
    /// (заменяются на маркеры версии/варианта).
    fn to_uuid_v7(&self) -> Uuid;

    /// Create SigID from UUID (preserves all bytes)
    fn from_uuid(uuid: Uuid) -> Self;
}

impl SigIdUuidExt for SigId26 {
    fn to_uuid(&self) -> Uuid {
        // Просто оборачиваем 16 байт "как есть" без модификаций
        Uuid::from_bytes(self.as_raw_bytes())
    }

    fn to_uuid_v7(&self) -> Uuid {
        let mut bytes = self.as_raw_bytes();

        // Set UUID version to 7 (RFC 9562)
        // byte 6: clear high nibble, set to 0x70 (binary 0111)
        bytes[6] = (bytes[6] & 0x0F) | 0x70;

        // Set UUID variant to RFC 4122
        // byte 8: clear top two bits, set to 0x80 (binary 10)
        bytes[8] = (bytes[8] & 0x3F) | 0x80;

        Uuid::from_bytes(bytes)
    }

    fn from_uuid(uuid: Uuid) -> Self {
        // Возвращаем 16 байт обратно в sigid
        let bytes = uuid.as_bytes();
        let mut raw = [0u8; 16];
        raw.copy_from_slice(bytes);
        Self::from_raw_bytes(raw)
    }
}

/// Extension trait for SigID to ULID conversion (when feature enabled)
#[cfg(feature = "ulid")]
pub mod ulid {
    use sigid_core::SigId26;
    use ulid::Ulid;

    /// Extension trait for SigID to ULID conversion
    pub trait SigIdUlidExt {
        /// Convert SigID to ULID (zero-cost)
        fn to_ulid(&self) -> Ulid;
        /// Create SigID from ULID
        fn from_ulid(ulid: Ulid) -> Self;
    }

    impl SigIdUlidExt for SigId26 {
        fn to_ulid(&self) -> Ulid {
            let raw = self.as_raw_bytes();
            Ulid::from_bytes(raw)
        }

        fn from_ulid(ulid: Ulid) -> Self {
            let bytes = ulid.to_bytes();
            let mut raw = [0u8; 16];
            raw.copy_from_slice(&bytes);
            Self::from_raw_bytes(raw)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use sigid_core::SigId26;

        #[test]
        fn test_ulid_roundtrip() {
            let original = SigId26::raw_new_with_worker(1234567890, 1, 42, 0x123456789abcdef);
            let ulid = original.to_ulid();
            let converted = SigId26::from_ulid(ulid);
            assert_eq!(original.as_bytes(), converted.as_bytes());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sigid_core::SigId26;

    #[test]
    fn test_uuid_roundtrip() {
        let original = SigId26::raw_new_with_worker(1234567890, 1, 42, 0x123456789abcdef);
        let uuid = original.to_uuid();
        let converted = SigId26::from_uuid(uuid);
        assert_eq!(original.as_bytes(), converted.as_bytes());
    }

    #[test]
    fn test_uuid_binary_compatible() {
        let id = SigId26::raw_new_with_worker(1234567890, 1, 42, 0x123456789abcdef);
        let uuid = id.to_uuid();
        // Проверяем, что байты совпадают
        assert_eq!(uuid.as_bytes(), &id.as_raw_bytes());
    }

    #[test]
    fn test_uuid_version() {
        let id = SigId26::raw_new_with_worker(1234567890, 1, 42, 0x123456789abcdef);
        let uuid = id.to_uuid();
        // UUID версия определяется по байтам из SigId
        // Так как мы не модифицируем биты, это может быть любое значение
        let version = uuid.get_version_num();
        // Просто проверяем, что метод работает и возвращает валидное значение
        assert!(version <= 15);
    }

    #[test]
    fn test_uuid_variant() {
        let id = SigId26::raw_new_with_worker(1234567890, 1, 42, 0x123456789abcdef);
        let uuid = id.to_uuid();
        // Проверяем, что вариант определен
        let variant = uuid.get_variant();
        // Может быть любым, так как мы не модифицируем биты
        assert!(variant == uuid::Variant::RFC4122 || variant == uuid::Variant::NCS);
    }

    #[test]
    fn test_uuid_v7() {
        let id = SigId26::raw_new_with_worker(1234567890, 1, 42, 0x123456789abcdef);
        let uuid = id.to_uuid_v7();

        // Проверяем версию 7
        assert_eq!(uuid.get_version_num(), 7);

        // Проверяем вариант RFC 4122
        assert_eq!(uuid.get_variant(), uuid::Variant::RFC4122);

        // Проверяем, что оригинальные байты не изменились кроме битов версии
        let orig_bytes = id.as_raw_bytes();
        let uuid_bytes = uuid.as_bytes();

        for i in 0..16 {
            if i == 6 || i == 8 {
                continue; // Пропускаем измененные байты
            }
            assert_eq!(orig_bytes[i], uuid_bytes[i], "Byte {} differs", i);
        }
    }
}
