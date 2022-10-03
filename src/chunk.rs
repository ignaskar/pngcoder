use core::fmt;
use std::fmt::{Display, Formatter};
use crc::Crc;

use crate::chunk_type::ChunkType;
use crate::{Result, Error, MAX_CHUNK_LEN};

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        if data.len() > MAX_CHUNK_LEN as usize {
            panic!("Max chunk length exceeded. CURRENT: {} MAX: {}", data.len(), MAX_CHUNK_LEN)
        }

        Self {
            chunk_type,
            data,
        }
    }

    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        let hasher = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        let mut digest = hasher.digest();
        digest.update(&self.chunk_type.bytes());
        digest.update(&self.data);
        digest.finalize()
    }

    pub fn data_as_string(&self) -> Result<String> {
        let s = std::str::from_utf8(&self.data)?;
        Ok(s.to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data.iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect()
    }
}

#[derive(Debug)]
enum ChunkError {
    InvalidChunkLength(usize),
    InvalidChunkType,
    CrcMismatch(u32, u32)
}

impl Display for ChunkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ChunkError::InvalidChunkLength(length) => {
                write!(f, "Invalid chunk length: {length}. Must be at least 12 bytes.")
            }
            ChunkError::InvalidChunkType => {
                write!(f, "Invalid chunk type detected.")
            }
            ChunkError::CrcMismatch(expected, actual) => {
                write!(f, "Invalid CRC detected. Expected: {expected}, actual: {actual}")
            }
        }
    }
}

impl std::error::Error for ChunkError {}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        if value.len() < 12 { return Err(Box::new(ChunkError::InvalidChunkLength(value.len()))) }
        let data_length = u32::from_be_bytes(value[0..4].try_into()?);
        let chunk_type_bytes: [u8; 4] = [
            value[4],
            value[5],
            value[6],
            value[7],
        ];

        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;
        if !chunk_type.is_valid() { return Err(Box::new(ChunkError::InvalidChunkType)) }

        let data = value[8..(8 + data_length as usize)].to_vec();
        let chunk = Chunk::new(chunk_type, data);

        let crc = u32::from_be_bytes(
            value[8 + data_length as usize..12 + data_length as usize].try_into()?
        );

        let calculated_crc = chunk.crc();

        if crc != calculated_crc { return Err(Box::new(ChunkError::CrcMismatch(crc, calculated_crc))); }

        Ok(chunk)
    }
}

fn validate_crc(chunk: &Chunk, bytes: &[u8], offset: usize) -> Result<()> {
    let crc_bytes = &bytes[offset..];
    let actual_crc = chunk.crc();
    let expected_crc = u32::from_be_bytes(crc_bytes.try_into()?);

    if actual_crc != expected_crc { return Err(Box::new(ChunkError::CrcMismatch(expected_crc, actual_crc))); }

    Ok(())
}

impl Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn get_chunk_data(data_length: u32, chunk_type: &[u8], message_bytes: &[u8], crc: u32) -> Vec<u8> {
        data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect()
    }

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = get_chunk_data(data_length, chunk_type, message_bytes, crc);

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!".as_bytes().to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = get_chunk_data(data_length, chunk_type, message_bytes, crc);

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = get_chunk_data(data_length, chunk_type, message_bytes, crc);

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = get_chunk_data(data_length, chunk_type, message_bytes, crc);

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}