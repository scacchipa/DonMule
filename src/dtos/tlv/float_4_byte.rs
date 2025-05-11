use std::io::{Cursor, Error, Read, Write};

use crate::traits::cursable::Cursable;

#[derive(Debug)]
pub struct Float4Byte {
    value: f32
}

impl Float4Byte {
    pub fn new(value: f32) -> Self {
        Float4Byte { value }
    }
}

impl PartialEq for Float4Byte {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Cursable for Float4Byte {

    fn write(&mut self, cursor: &mut Cursor<&mut [u8]>) -> Result<usize, Error> {

        let mut buffer = [0u8; size_of::<f32>()];
        cursor.read_exact(&mut buffer)?;
        self.value = f32::from_le_bytes(buffer);

        return Ok(buffer.len());
    }

    fn read(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {

        let size = cursor.write(&f32::to_le_bytes(self.value))?;

        return Ok(size);
    }
}

#[cfg(test)]
mod tests {
    use std::{io::Cursor, vec};

    use crate::{dtos::tlv::float_4_byte::Float4Byte, traits::cursable::Cursable};


    #[test]
    fn test_float_4_byte_should_read() {

        let mut subject = Float4Byte {
            value: 123.456f32,
        };

        let vect = vec![0u8; 4];

        let mut cursor = Cursor::new(vect);

        subject.read(&mut cursor);

        let buffer = cursor.into_inner();

        assert_eq!(buffer[0], 0x79);
        assert_eq!(buffer[1], 0xE9);
        assert_eq!(buffer[2], 0xF6);
        assert_eq!(buffer[3], 0x42);
    }

    #[test]
    fn test_float_4_byte_should_write() {
        
        let mut subject = Float4Byte {
            value: 0.0,
        };

        let mut buf = [0x79u8, 0xE9u8, 0xF6, 0x42u8];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.write(&mut cursor);

        println!("{}", subject.value);

        assert_eq!(subject.value, 123.456f32);
    }

}