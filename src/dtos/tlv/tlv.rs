use crate::traits::cursable::Cursable;

use super::{float_4_byte::Float4Byte, integer_1_byte::Integer1Byte, integer_4_byte::Integer4Byte, string_2_byte_length::String2ByteLength, tlv_value::TlvValue};

pub struct Tlv {
    tlv_type: Integer1Byte,
    tlv_name: String2ByteLength,
    tlv_value: TlvValue,
}

impl Tlv {
    pub fn new(
        tlv_type: Integer1Byte,
        tlv_name: String2ByteLength,
        tlv_value: TlvValue
    ) -> Self { 
        Tlv {tlv_type, tlv_name, tlv_value}
    }
}

impl Cursable for Tlv {
    fn read(&mut self, cursor: &mut std::io::Cursor<&mut [u8]>) {
        self.tlv_type.read(cursor);
        self.tlv_name.read(cursor);
        self.tlv_value.read(cursor);
    }

    fn write(&mut self, cursor: &mut std::io::Cursor<&mut [u8]>) {
        self.tlv_type.write(cursor);
        self.tlv_name.write(cursor);

        self.tlv_value = match self.tlv_type.value {
            0x02u8 => TlvValue::DescString(String2ByteLength::new(Vec::new())),
            0x03u8 => TlvValue::Integer4Byte(Integer4Byte::new(0u32)),
            0x04u8 => TlvValue::Float4Byte(Float4Byte::new(0f32)),
            _ => TlvValue::DescString(String2ByteLength::new(Vec::new())),
        };

        self.tlv_value.write(cursor);
    }
}

mod tests {
    use std::io::Cursor;

    use crate::{dtos::tlv::{integer_1_byte::Integer1Byte, integer_4_byte::Integer4Byte, string_2_byte_length::String2ByteLength, tlv::Tlv, tlv_value::TlvValue}, traits::cursable::Cursable};

    #[test]
    fn test_write_desct_string() {

        let mut tlv = Tlv {
            tlv_type: Integer1Byte::new(0),
            tlv_name: String2ByteLength::new(Vec::new()),
            tlv_value: TlvValue::DescString(String2ByteLength::new(Vec::new())),
        };

        let mut buf1 = [0x03, 0x07, 0x00, b'b', b'i', b't', b'r', b'a', b't', b'e', 0x80, 0x00, 0x00, 0x00];
        let mut cursor1 = Cursor::new(&mut buf1[..]);
        tlv.write(&mut cursor1);
        assert_eq!(0x03u8, tlv.tlv_type.value);
        assert_eq!(String2ByteLength::new(b"bitrate".to_vec()), tlv.tlv_name);
        assert_eq!(TlvValue::Integer4Byte(Integer4Byte::new(0x80u32)), tlv.tlv_value);

        let mut buf2= [0x03u8, 0x01u8, 0x00u8, 0x0Fu8, 0x36u8, 0x12u8, 0x00u8, 0x00u8];
        let mut cursor2 = Cursor::new(&mut buf2[..]);
        tlv.write(&mut cursor2);
        assert_eq!(0x03, tlv.tlv_type.value);
        assert_eq!(String2ByteLength::new(vec![0x0Fu8]), tlv.tlv_name);
        assert_eq!(TlvValue::Integer4Byte(Integer4Byte::new(0x1236u32)), tlv.tlv_value);

        let mut buf3 = [0x02u8, 0x01u8, 0x00u8, 0x01u8, 0x05u8, 0x00u8, b'h', b'e', b'l', b'l', b'o'];
        let mut cursor3 = Cursor::new(&mut buf3[..]);
        tlv.write(&mut cursor3);
        assert_eq!(0x02, tlv.tlv_type.value);
        assert_eq!(String2ByteLength::new(vec![0x01u8]), tlv.tlv_name);
        assert_eq!(TlvValue::DescString(String2ByteLength::new(b"hello".to_vec())), tlv.tlv_value);

        let mut buf4 = [0x03, 0x01, 0x00, 0x02, 0x3D, 0x0F, 0x00, 0x00];
        let mut cursor4 = Cursor::new(&mut buf4[..]);
        tlv.write(&mut cursor4);
        assert_eq!(0x03, tlv.tlv_type.value);
        assert_eq!(String2ByteLength::new(vec![0x02u8]), tlv.tlv_name);
        assert_eq!(TlvValue::Integer4Byte(Integer4Byte::new(0x0F3D)), tlv.tlv_value);
    }

    #[test]
    fn test_read_desct_string() {

        let mut tlv1 = Tlv {
            tlv_type: Integer1Byte::new(0x03u8),
            tlv_name: String2ByteLength::new(b"bitrate".to_vec()),
            tlv_value: TlvValue::Integer4Byte(Integer4Byte::new(0x80u32)),
        };
        let mut buf1 = [0x030u8; 14];
        let mut cursor1 = Cursor::new(&mut buf1[..]);
        tlv1.read(&mut cursor1);
        assert_eq!([0x03, 0x07, 0x00, b'b', b'i', b't', b'r', b'a', b't', b'e', 0x80, 0x00, 0x00, 0x00], buf1);

        let mut tlv2 = Tlv {
            tlv_type: Integer1Byte::new(0x03),
            tlv_name: String2ByteLength::new(vec![0x0Fu8]),
            tlv_value: TlvValue::Integer4Byte(Integer4Byte::new(0x1236u32))
        };
        let mut buf2= [0x00u8; 8];
        let mut cursor2 = Cursor::new(&mut buf2[..]);
        tlv2.read(&mut cursor2);
        assert_eq!([0x03u8, 0x01u8, 0x00u8, 0x0Fu8, 0x36u8, 0x12u8, 0x00u8, 0x00u8], buf2);

        let mut tlv3 = Tlv {
            tlv_type: Integer1Byte::new(0x02),
            tlv_name: String2ByteLength::new(vec![0x01u8]),
            tlv_value: TlvValue::DescString(String2ByteLength::new(b"hello".to_vec())),
        };
        let mut buf3 = [0x0u8; 11];
        let mut cursor3 = Cursor::new(&mut buf3[..]);
        tlv3.read(&mut cursor3);
        assert_eq!([0x02u8, 0x01u8, 0x00u8, 0x01u8, 0x05u8, 0x00u8, b'h', b'e', b'l', b'l', b'o'], buf3);
        
        let mut tlv4 = Tlv {
            tlv_type: Integer1Byte::new(0x03),
            tlv_name: String2ByteLength::new(vec![0x02u8]),
            tlv_value: TlvValue::Integer4Byte(Integer4Byte::new(0x0F3D)),
        };
        let mut buf4 = [0x0u8; 8];
        let mut cursor4 = Cursor::new(&mut buf4[..]);
        tlv4.read(&mut cursor4);
        assert_eq!([0x03u8, 0x01u8, 0x00u8, 0x02u8, 0x3Du8, 0x0Fu8, 0x00u8, 0x00u8], buf4);

    }

}