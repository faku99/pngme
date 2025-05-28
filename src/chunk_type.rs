use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChunkType {
    // type_code: [u8; 4],
    type_code: u32,
}

#[allow(non_camel_case_types)]
enum ChunkTypeBits {
    SAFE_TO_COPY = 0x00000020,
    RESERVED = 0x00002000,
    PRIVATE = 0x00200000,
    ANCILLARY = 0x20000000,
}

#[allow(dead_code)]
impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        u32::to_be_bytes(self.type_code)
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid() // NOTE: Unsure about this one
    }

    pub fn is_critical(&self) -> bool {
        (self.type_code & ChunkTypeBits::ANCILLARY as u32) == 0
    }

    pub fn is_public(&self) -> bool {
        (self.type_code & ChunkTypeBits::PRIVATE as u32) == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.type_code & ChunkTypeBits::RESERVED as u32) == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (self.type_code & ChunkTypeBits::SAFE_TO_COPY as u32) != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 4]) -> anyhow::Result<Self> {
        if value.map(|c| c.is_ascii_alphabetic()).contains(&false) {
            anyhow::bail!("Invalid value")
        }
        Ok(Self {
            type_code: u32::from_be_bytes(value),
        })
    }
}

impl FromStr for ChunkType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        let mut bytes = s.bytes();

        let (Some(a), Some(b), Some(c), Some(d), None) = (
            bytes.next(),
            bytes.next(),
            bytes.next(),
            bytes.next(),
            bytes.next(),
        ) else {
            anyhow::bail!("Invalid length")
        };

        Self::try_from([a, b, c, d])
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            String::from_utf8(Vec::<u8>::from(u32::to_be_bytes(self.type_code))).unwrap()
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
