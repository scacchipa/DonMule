use std::io::{Cursor, Read, Write};

use crate::traits::cursable::Cursable;

struct Integer4Byte {
    value: u32
}

impl Cursable for Integer4Byte {

    fn write(&mut self, cursor: &mut Cursor<&mut [u8]>) {

        let mut buffer = [0u8; size_of::<u32>()];
        cursor.read_exact(&mut buffer).expect("Failed to read Float4Byte data");
        self.value = u32::from_le_bytes(buffer);
    }

    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) {
        
        cursor.write(&self.value.to_le_bytes()).expect("Failed to write Float4Byte data");
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{dtos::tlv::integer_4_byte::Integer4Byte, traits::cursable::Cursable};


    #[test]
    fn test_integer_4_byte_should_read() {

        let mut subject = Integer4Byte {
            value: 0xA9876543u32,
        };

        let mut buf = [0u8; 4];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.read(&mut cursor);

        println!("probando {:02X?}", buf);

        assert_eq!(buf[0], 0x43u8);
        assert_eq!(buf[1], 0x65u8);
        assert_eq!(buf[2], 0x87u8);
        assert_eq!(buf[3], 0xa9u8);
    }

    #[test]
    fn test_integer_4_byte_should_write() {
        
        let mut subject = Integer4Byte {
            value: 170,
        };

        let mut buf = [0x21u8, 0x43u8, 0x65u8, 0x87u8];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.write(&mut cursor);

        println!("{}", subject.value);

        assert_eq!(subject.value, 0x87654321u32);
    }

}