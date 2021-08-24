use std::convert::TryFrom;

use crate::{chunk_type::ChunkType, Error};

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
    data_length: u32,
    crc: u32,
}

impl Chunk {
    pub fn length(&self) -> u32 {
        self.data_length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        todo!()
    }
    pub fn data(&self) -> &[u8] {
        todo!()
    }
    pub fn crc(&self) -> u32 {
        todo!()
    }
    pub fn data_as_string(&self) -> Result<String, Error> {
        todo!()
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {}
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
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    //     #[test]
    //     fn test_chunk_type() {
    //         let chunk = testing_chunk();
    //         assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    //     }

    //     #[test]
    //     fn test_chunk_string() {
    //         let chunk = testing_chunk();
    //         let chunk_string = chunk.data_as_string().unwrap();
    //         let expected_chunk_string = String::from("This is where your secret message will be!");
    //         assert_eq!(chunk_string, expected_chunk_string);
    //     }

    //     #[test]
    //     fn test_chunk_crc() {
    //         let chunk = testing_chunk();
    //         assert_eq!(chunk.crc(), 2882656334);
    //     }

    //     #[test]
    //     fn test_valid_chunk_from_bytes() {
    //         let data_length: u32 = 42;
    //         let chunk_type = "RuSt".as_bytes();
    //         let message_bytes = "This is where your secret message will be!".as_bytes();
    //         let crc: u32 = 2882656334;

    //         let chunk_data: Vec<u8> = data_length
    //             .to_be_bytes()
    //             .iter()
    //             .chain(chunk_type.iter())
    //             .chain(message_bytes.iter())
    //             .chain(crc.to_be_bytes().iter())
    //             .copied()
    //             .collect();

    //         let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

    //         let chunk_string = chunk.data_as_string().unwrap();
    //         let expected_chunk_string = String::from("This is where your secret message will be!");

    //         assert_eq!(chunk.length(), 42);
    //         assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    //         assert_eq!(chunk_string, expected_chunk_string);
    //         assert_eq!(chunk.crc(), 2882656334);
    //     }

    //     #[test]
    //     fn test_invalid_chunk_from_bytes() {
    //         let data_length: u32 = 42;
    //         let chunk_type = "RuSt".as_bytes();
    //         let message_bytes = "This is where your secret message will be!".as_bytes();
    //         let crc: u32 = 2882656333;

    //         let chunk_data: Vec<u8> = data_length
    //             .to_be_bytes()
    //             .iter()
    //             .chain(chunk_type.iter())
    //             .chain(message_bytes.iter())
    //             .chain(crc.to_be_bytes().iter())
    //             .copied()
    //             .collect();

    //         let chunk = Chunk::try_from(chunk_data.as_ref());

    //         assert!(chunk.is_err());
    //     }

    //     #[test]
    //     pub fn test_chunk_trait_impls() {
    //         let data_length: u32 = 42;
    //         let chunk_type = "RuSt".as_bytes();
    //         let message_bytes = "This is where your secret message will be!".as_bytes();
    //         let crc: u32 = 2882656334;

    //         let chunk_data: Vec<u8> = data_length
    //             .to_be_bytes()
    //             .iter()
    //             .chain(chunk_type.iter())
    //             .chain(message_bytes.iter())
    //             .chain(crc.to_be_bytes().iter())
    //             .copied()
    //             .collect();

    //         let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

    //         let _chunk_string = format!("{}", chunk);
    //     }
    // }
}
