use std::fmt::{Display, Formatter};
use crate::chunk_type::{ChunkType, InvalidStringError};
use crc::{Crc};
use crate::{Error, Result};


#[derive(Debug)]
pub struct Chunk{
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let crc = Crc::<u32>::new(&crc::CRC_32_ISO_HDLC);
        Chunk {
            crc: crc.checksum(&[&chunk_type.bytes()[..], &data].concat()),
            chunk_type,
            data
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
        Ok(std::str::from_utf8(&self.data).unwrap().to_string())
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        (self.data.len() as u32).to_be_bytes().into_iter().chain(
            self.chunk_type.bytes().into_iter()).chain(
                self.data.clone().into_iter()).chain(
                    self.crc.to_be_bytes().into_iter()).collect()

    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "A nice chunk")
    }
}

#[derive(Debug)]
enum ChunkError {
    NotEnoughData,
    CorruptedChunkType(InvalidStringError),
    CorruptedLength,
    CorruptedChecksum,
}

impl std::error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotEnoughData => write!(f, "More bytes needed!!!"),
            ChunkError::NotEnoughData => todo!(),
            ChunkError::CorruptedChunkType(internal_error) => write!(f, "{}", internal_error.to_string()),
            ChunkError::CorruptedLength => write!(f, "The length bytes are corrupted"),
            ChunkError::CorruptedChecksum => write!(f, "CRC part is corrupted, and does not represent the actual checksum"),
        }
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 12 {
            return Err(Box::new(ChunkError::NotEnoughData));
        }

        let length = u32::from_be_bytes(value[..4].try_into().unwrap());

        if (length + 12) as usize != value.len() {
            return Err(Box::new(ChunkError::CorruptedLength));
        }

        let chunk_type = ChunkType::try_from(<&[u8] as TryInto<[u8; 4]>>::try_into(&value[4..8]).unwrap())?;
        let chunk_data = &value[8..(8+length) as usize];

        let crc = u32::from_be_bytes(
            (&value[(length + 8) as usize..(length + 12) as usize]).try_into().unwrap());

        let result = Chunk::new(chunk_type, chunk_data.into());

        if result.crc() != crc {
            return Err(Box::new(ChunkError::CorruptedChecksum));
        }

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

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

    #[test]
    pub fn test_as_bytes() {
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

        let chunk_bytes = chunk.as_bytes();

        assert_eq!(chunk_data, chunk_bytes);
    }
}