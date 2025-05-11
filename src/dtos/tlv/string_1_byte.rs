use std::io::{Cursor, Read, Write};

use crate::traits::cursable::Cursable;

pub struct String1Byte {
    value: u8
}

impl Cursable for String1Byte {

    fn write(&mut self, cursor: &mut Cursor<&mut [u8]>) {

        let mut buffer = [0u8; size_of::<String1Byte>()];
        cursor.read_exact(&mut buffer).expect("Failed to read Float4Byte data");
        self.value = u8::from_le_bytes(buffer);
    }

    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) {
        
        let buf = u8::to_le_bytes(self.value);

        let _ = cursor.write(&buf);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{dtos::tlv::{integer_1_byte::Integer1Byte, string_1_byte::String1Byte}, traits::cursable::Cursable};

    #[test]
    fn test_integer_1_byte_should_read() {

        let mut subject = String1Byte {
            value: 140u8,
        };

        let mut buf = [0u8; 1];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.read(&mut cursor);

        println!("probando {:02?}", buf);

        assert_eq!(buf[0], 140u8);
        // assert_eq!(buf[1], 0xE9);
        // assert_eq!(buf[2], 0xF6);
        // assert_eq!(buf[3], 0x42);
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