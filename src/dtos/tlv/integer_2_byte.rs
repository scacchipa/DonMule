use std::io::{Cursor, Error, Read, Write};

use crate::traits::cursable::Cursable;

#[derive(Debug)]
pub struct Integer2Byte {
    value: u16
}

impl Integer2Byte {
    pub fn new(value: u16) -> Self {
        Integer2Byte { value }
    }
}

impl PartialEq for Integer2Byte {    
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Cursable for Integer2Byte {

    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) -> Result<usize, Error> {

        let mut buffer = [0u8; size_of::<u16>()];
        cursor.read_exact(&mut buffer);
        self.value = u16::from_le_bytes(buffer);

        return Ok(buffer.len());
    }

    fn write(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {
        
        return cursor.write(&self.value.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{dtos::tlv::{integer_2_byte::Integer2Byte, integer_4_byte::Integer4Byte}, traits::cursable::Cursable};


    #[test]
    fn test_integer_2_byte_should_write() {

        let mut subject = Integer2Byte {
            value: 0xA987u16,
        };

        let mut cursor = Cursor::new(vec![0u8; 4]);

        subject.write(&mut cursor);

        let vect = cursor.into_inner();
        assert_eq!(vect[0], 0x87u8);
        assert_eq!(vect[1], 0xa9u8);
    }

    #[test]
    fn test_integer_2_byte_should_read() {
        
        let mut subject = Integer2Byte {
            value: 170,
        };

        let mut buf = [0x21u8, 0x43u8];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.read(&mut cursor);

        println!("{}", subject.value);

        assert_eq!(subject.value, 0x4321u16);
    }

}