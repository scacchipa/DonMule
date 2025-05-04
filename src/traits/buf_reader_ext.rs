use std::io::Error;

pub trait BufReaderExt {
    fn read_byte(&mut self) -> Result<u8, Error>;
    fn read_word_be(&mut self) -> Result<u16, Error>;
    fn read_word_le(&mut self) -> Result<u16, Error>;
    fn read_dword_be(&mut self) -> Result<u32, Error>;
    fn read_dword_le(&mut self) -> Result<u32, Error>;
    fn read_string(&mut self) -> Result<String, Error>;
    fn read_ip(&mut self) -> Result<[u8; 4], Error>;
    fn read_array(&mut self, length: u32) -> Result<Vec<u8>, Error>;
}