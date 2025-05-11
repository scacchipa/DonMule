use std::io::{Cursor, Error, Read, Write};

use crate::traits::cursable::Cursable;

pub struct String1Byte {
    value: u8
}

impl Cursable for String1Byte {

    fn write(&mut self, cursor: &mut Cursor<&mut [u8]>) -> Result<usize, Error> {

        let mut buffer = [0u8; size_of::<String1Byte>()];
        cursor.read_exact(&mut buffer)?;
        self.value = u8::from_le_bytes(buffer);

        return Ok(buffer.len());
    }

    fn read(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {
        
        let buf = u8::to_le_bytes(self.value);

        return cursor.write(&buf);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{dtos::tlv::string_1_byte::String1Byte, traits::cursable::Cursable};

    #[test]
    fn test_integer_1_byte_should_read() {

        let mut subject = String1Byte {
            value: 140u8,
        };

        let mut cursor = Cursor::new(vec![0u8, 1]);

        subject.read(&mut cursor);

        let buf = cursor.into_inner();
        assert_eq!(buf[0], 140u8);
    }

    #[test]
    fn test_integer_1_byte_should_write() {
        
        let mut subject = String1Byte {
            value: 170u8,
        };

        let mut buf = [170u8];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.write(&mut cursor);

        println!("{}", subject.value);

        assert_eq!(subject.value, 170u8);
    }
}