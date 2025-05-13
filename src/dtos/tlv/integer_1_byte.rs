use std::io::{Cursor, Error, Read, Write};

use crate::traits::cursable::Cursable;

pub struct Integer1Byte {
    pub value: u8
}

impl Integer1Byte {
    pub fn new(value: u8) -> Self {
        Integer1Byte { value }
    }
}

impl Cursable for Integer1Byte {

    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) -> Result<usize, Error>{

        let mut buffer = [0u8; size_of::<Integer1Byte>()];
        cursor.read_exact(&mut buffer)?;
        self.value = u8::from_le_bytes(buffer);

        return Ok(buffer.len());
    }

    fn write(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {
        
        let buf = u8::to_le_bytes(self.value);

        cursor.write(&buf)?;

        return Ok(buf.len());
    }
}


#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{dtos::tlv::integer_1_byte::Integer1Byte, traits::cursable::Cursable};

    #[test]
    fn test_integer_1_byte_should_write() {

        let mut subject = Integer1Byte { value: 140u8 };

        let mut cursor = Cursor::new( vec![0u8; 1]);

        subject.write(&mut cursor).unwrap();

        let vect = cursor.into_inner();

        assert_eq!(vect[0], 140u8);
    }

    #[test]
    fn test_integer_1_byte_should_read() {
        
        let mut subject = Integer1Byte {
            value: 170,
        };

        let mut buf = [170u8];
        let mut cursor = Cursor::new(&mut buf[..]);

        let size = subject.read(&mut cursor).unwrap();

        println!("{}", subject.value);

        assert_eq!(size, 1);
        assert_eq!(subject.value, 170u8);
    }
}