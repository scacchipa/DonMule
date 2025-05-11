use std::io::{Cursor, Read, Write};

use crate::traits::cursable::Cursable;

#[derive(Debug)]
pub struct String2ByteLength {
    length: u16,
    value: Vec<u8>
}

impl String2ByteLength {
    pub fn new(value: Vec<u8>) -> Self {
        String2ByteLength { 
            length: value.len() as u16,
            value
        }
    }
}

impl PartialEq for String2ByteLength {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.value == other.value
    }
}

impl Cursable for String2ByteLength {

    fn write(&mut self, cursor: &mut Cursor<&mut [u8]>) {

        let mut buffer = [0u8; size_of::<u16>()];
        cursor.read_exact(&mut buffer).expect("Failed to read 2-byte length");
        self.length = u16::from_le_bytes(buffer);

        let mut buffer = vec![0u8; usize::from(self.length)];
        cursor.read_exact(&mut buffer).expect("Failed to read String");
        self.value = buffer;
    }

    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) {

        cursor.write(&self.length.to_le_bytes()).expect("Failed to write 2-byte length data");
        cursor.write(&self.value).expect("Failed to write String data");
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use crate::{dtos::tlv::string_2_byte_length::String2ByteLength, traits::cursable::Cursable};


    #[test]
    fn test_string_2_byte_length_should_read() {

        let txt = String::from("abc");

        let mut subject = String2ByteLength {
            length: txt.len() as u16,
            value: txt.into_bytes(),
        };

        let mut buf = [0u8; 5];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.read(&mut cursor);

        assert_eq!(buf[0], 0x03u8);
        assert_eq!(buf[1], 0x00u8);
        assert_eq!(buf[2], 0x61u8);
        assert_eq!(buf[3], 0x62u8);
        assert_eq!(buf[4], 0x63u8);
    }

    #[test]
    fn test_string_2_byte_length_should_write() {
        
        let mut subject = String2ByteLength {
            length: 0u16,
            value: vec!()
        };

        let mut buf = [0x04u8, 0x00u8, 0x65u8, 0x66u8, 0x67, 0x68];
        let mut cursor = Cursor::new(&mut buf[..]);

        subject.write(&mut cursor);

        assert_eq!(subject.length, 0x0004u16);
        assert_eq!(subject.value, "efgh".as_bytes());
    }
}