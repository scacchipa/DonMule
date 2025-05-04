use std::io::{Cursor, Error, Read};

use bytes::Bytes;

use crate::traits::buf_reader_ext::ByteReader;

impl ByteReader for Cursor<Bytes> {
    fn read_u8(&mut self) -> Result<u8, Error> {
        let mut byte_buffer = [0u8; 1];
        self.read_exact(&mut byte_buffer)?;

        return Ok(byte_buffer[0]);
    }

    fn read_u16_be(&mut self) -> Result<u16, Error> {
        let mut word_buffer = [0u8; 2];
        self.read_exact(&mut word_buffer)?;

        return Ok(u16::from_be_bytes(word_buffer));
    }

    fn read_u16_le(&mut self) -> Result<u16, Error> {
        let mut word_buffer = [0u8; 2];
        self.read_exact(&mut word_buffer)?;

        return Ok(u16::from_le_bytes(word_buffer));
    }

    fn read_u32_be(&mut self) -> Result<u32, Error> {
        let mut dword_buffer = [0u8; 4];
        self.read_exact(&mut dword_buffer)?;

        return Ok(u32::from_be_bytes(dword_buffer));
    }

    fn read_u32_le(&mut self) -> Result<u32, Error> {
        let mut dword_buffer = [0u8; 4];
        self.read_exact(&mut dword_buffer)?;

        return Ok(u32::from_le_bytes(dword_buffer));
    }

    fn read_string(&mut self) -> Result<String, Error> {
        let len = self.read_u16_le()?;

        println!("str len {}", len);

        let mut bytes = vec![0; len as usize];
        self.read_exact(&mut bytes)?;

        return Ok(String::from_utf8(bytes)
            .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8"))?)
    }

    fn read_ip(&mut self) -> Result<[u8; 4], Error> {
        let mut ip = [0u8; 4];
        self.read_exact(&mut ip)?;

        return Ok(ip);
    }

    fn read_array(&mut self, length: u32) -> Result<Vec<u8>, Error> {
        let mut arr = vec![0u8; length as usize];
        self.read_exact(&mut arr)?;

        return Ok(arr)
    }
}