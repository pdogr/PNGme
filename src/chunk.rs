use crate::chunk_type::ChunkType;
use crate::Error;
use crate::Result;
extern crate crc32fast;
use crc32fast::Hasher;

use std::{
    convert::{TryFrom, TryInto},
    fmt::{Display, Formatter},
};
#[derive(Debug)]
pub enum ChunkErr {
    ChunksLengthLess,
    ChunkCRCInvalid,
    ChunkLengthInvalid,
}
impl std::error::Error for ChunkErr {}
impl Display for ChunkErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            ChunkErr::ChunksLengthLess => write!(f, "Size of chunks must be atleat 12"),
            ChunkErr::ChunkCRCInvalid => write!(f, "Crc in chunk does not match actual crc"),
            ChunkErr::ChunkLengthInvalid => {
                write!(f, "Length in chunk does not match actual length")
            }
        }
    }
}
#[derive(Debug)]
pub struct Chunk {
    chunk_type: ChunkType,
    data: Box<[u8]>,
    crc: u32,
}
impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(chunks: &[u8]) -> Result<Self> {
        if chunks.len() < 12 {
            return Err(Box::new(ChunkErr::ChunksLengthLess));
        }
        let (expected_length, tail) = chunks.split_at(4);
        let expected_length = u32::from_be_bytes(expected_length.try_into()?);
        let length = (tail.len() - 8) as u32;
        if expected_length != length {
            return Err(Box::new(ChunkErr::ChunkLengthInvalid));
        }
        let (type_and_data, expected_crc) = tail.split_at(tail.len() - 4);
        let mut hasher = Hasher::new();
        hasher.update(type_and_data);
        let crc = hasher.finalize();
        let expected_crc = u32::from_be_bytes(expected_crc.try_into()?);
        if expected_crc != crc {
            return Err(Box::new(ChunkErr::ChunkCRCInvalid));
        }
        let (chunk_type_bytes, data) = type_and_data.split_at(4);
        let chunk_type = ChunkType::try_from(TryInto::<[u8; 4]>::try_into(chunk_type_bytes)?)?;
        Ok(Self {
            chunk_type,
            data: Box::from(data),
            crc,
        })
    }
}
impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let mut hasher = Hasher::new();
        hasher.update(
            chunk_type
                .bytes()
                .iter()
                .chain(data.iter())
                .cloned()
                .collect::<Vec<u8>>()
                .as_slice(),
        );
        Self {
            chunk_type,
            data: data.into_boxed_slice(),
            crc: hasher.finalize(),
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
        self.crc
    }
    pub fn data_as_string(&self) -> Result<String> {
        match std::str::from_utf8(&self.data()) {
            Ok(s) => Ok(String::from(s)),
            Err(e) => Err(Box::new(e)),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc.to_be_bytes().iter())
            .cloned()
            .collect::<Vec<_>>()
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            std::str::from_utf8(&self.data()).map_err(|_| std::fmt::Error)?
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
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

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

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

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
