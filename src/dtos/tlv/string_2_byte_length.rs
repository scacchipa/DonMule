use std::io::{Cursor, Error, Read, Write};

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

    fn write(&mut self, cursor: &mut Cursor<&mut [u8]>) -> Result<usize, Error> {

        let mut buffer1 = [0u8; size_of::<u16>()];
        cursor.read_exact(&mut buffer1)?;
        self.length = u16::from_le_bytes(buffer1);
        let mut size = 2;

        let mut buffer2: Vec<u8> = vec![0u8; usize::from(self.length)];
        cursor.read_exact(&mut buffer2)?;
        self.value = buffer2;
        size += self.value.len();

        return Ok(size)
    }

    fn read(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {

        let mut size = cursor.write(&self.length.to_le_bytes())?;
        size += cursor.write(&self.value)?;

        return Ok(size);
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

        
        let mut cursor = Cursor::new(vec![0u8; subject.length as usize + 2]);

        subject.read(&mut cursor);


        let vect = cursor.into_inner();
        assert_eq!(vect[0], 0x03u8);
        assert_eq!(vect[1], 0x00u8);
        assert_eq!(vect[2], 0x61u8);
        assert_eq!(vect[3], 0x62u8);
        assert_eq!(vect[4], 0x63u8);
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