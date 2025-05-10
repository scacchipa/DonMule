use std::io::{Cursor, Read, Write};

use crate::traits::cursable::Cursable;

struct Float4Byte {
    value: f32
}

impl Cursable for Float4Byte {

    fn write(&mut self, cursor: &mut Cursor<&mut [u8]>) {

        let mut buffer = [0u8; size_of::<f32>()];
        cursor.read_exact(&mut buffer).expect("Failed to read Float4Byte data");
        self.value = f32::from_le_bytes(buffer);
    }

    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) {

        let buf = f32::to_le_bytes(self.value);

        let _ = cursor.write(&buf);
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{dtos::tlv::float_4_byte::Float4Byte, traits::cursable::Cursable};


    #[test]
    fn test_float_4_byte_should_read() {

        let mut subject = Float4Byte {
            value: 123.456f32,
        };

        let mut buf = [0u8; 10];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.read(&mut cursor);

        println!("probando {:02X?}", buf);

        assert_eq!(buf[0], 0x79);
        assert_eq!(buf[1], 0xE9);
        assert_eq!(buf[2], 0xF6);
        assert_eq!(buf[3], 0x42);
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