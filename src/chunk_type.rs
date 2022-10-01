use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::str;
use crate::{Result, Error};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ChunkType {
    bytes: [u8; 4]
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.bytes
    }

    pub fn is_valid(&self) -> bool {
        for byte in &self.bytes {
            match byte {
                b'a'..=b'z' | b'A'..=b'Z' => continue,
                _ => return false,
            }
        }
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        (b'A'..=b'Z').contains(&self.bytes[0])
    }

    pub fn is_public(&self) -> bool {
        (b'A'..=b'Z').contains(&self.bytes[1])
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        (b'A'..=b'Z').contains(&self.bytes[2])
    }

    pub fn is_safe_to_copy(&self) -> bool {
        (b'a'..=b'z').contains(&self.bytes[3])
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.bytes).unwrap())
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self> {
        if value.len() != 4 {
            return Err(Box::new(ChunkTypeError::InvalidLength(value.len())));
        }

        Ok(Self { bytes: value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let bytes = s.as_bytes();

        if bytes.len() != 4 {
            return Err(Box::new(ChunkTypeError::InvalidLength(bytes.len())));
        }

        let valid_chars = bytes
            .iter()
            .all(|&b| (b'A'..=b'Z').contains(&b) || (b'a'..=b'z').contains(&b));

        if !valid_chars {
            return Err(Box::new(ChunkTypeError::InvalidCharacter));
        }

        let sized: [u8; 4] = [bytes[0], bytes[1], bytes[2], bytes[3]];
        ChunkType::try_from(sized)
    }
}

#[derive(Debug)]
enum ChunkTypeError {
    InvalidLength(usize),
    InvalidCharacter
}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::InvalidLength(length) => {
                write!(f, "Invalid chunk length: {length}. Expected: 4")
            }
            ChunkTypeError::InvalidCharacter => {
                write!(f, "Invalid character detected!")
            }
        }
    }
}

impl std::error::Error for ChunkTypeError {}

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
        let val = &chunk.to_string();
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