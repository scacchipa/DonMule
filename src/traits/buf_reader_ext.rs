use std::io::Error;

pub trait ByteReader {
    fn read_u8(&mut self) -> Result<u8, Error>;
    fn read_u16_be(&mut self) -> Result<u16, Error>;
    fn read_u16_le(&mut self) -> Result<u16, Error>;
    fn read_u32_be(&mut self) -> Result<u32, Error>;
    fn read_u32_le(&mut self) -> Result<u32, Error>;
    fn read_string(&mut self) -> Result<String, Error>;
    fn read_ip(&mut self) -> Result<[u8; 4], Error>;
    fn read_array(&mut self, length: u32) -> Result<Vec<u8>, Error>;
}