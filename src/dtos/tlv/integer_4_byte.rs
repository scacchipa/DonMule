use std::io::{Cursor, Error, Read, Write};

use crate::traits::cursable::Cursable;

#[derive(Debug)]
pub struct Integer4Byte {
    pub value: u32
}

impl Integer4Byte {
    pub fn new(value: u32) -> Self {
        Integer4Byte { value }
    }
}

impl PartialEq for Integer4Byte {    
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Cursable for Integer4Byte {

    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) -> Result<usize, Error> {

        let mut buffer = [0u8; size_of::<u32>()];
        cursor.read_exact(&mut buffer)?;
        self.value = u32::from_le_bytes(buffer);

        return Ok(buffer.len());
    }

    fn write(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {
        
        return cursor.write(&self.value.to_le_bytes());
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{dtos::tlv::integer_4_byte::Integer4Byte, traits::cursable::Cursable};


    #[test]
    fn test_integer_4_byte_should_write() {

        let mut subject = Integer4Byte {
            value: 0xA9876543u32,
        };

        let mut cursor = Cursor::new(vec![0u8; 4]);

        let size = subject.write(&mut cursor).unwrap();

        let vect = cursor.into_inner();
        assert_eq!(size, 4);
        assert_eq!(vect[0], 0x43u8);
        assert_eq!(vect[1], 0x65u8);
        assert_eq!(vect[2], 0x87u8);
        assert_eq!(vect[3], 0xa9u8);
    }

    #[test]
    fn test_integer_4_byte_should_read() {
        
        let mut subject = Integer4Byte {
            value: 170,
        };

        let mut buf = [0x21u8, 0x43u8, 0x65u8, 0x87u8];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.read(&mut cursor);

        println!("{}", subject.value);

        assert_eq!(subject.value, 0x87654321u32);
    }

}