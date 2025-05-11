use crate::traits::cursable::Cursable;

use super::{float_4_byte::Float4Byte, integer_4_byte::Integer4Byte, string_2_byte_length::String2ByteLength};

#[derive(Debug)]
pub enum TlvValue {
    DescString(String2ByteLength),
    Integer4Byte(Integer4Byte),
    Float4Byte(Float4Byte),
}

impl PartialEq for TlvValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DescString(l0), Self::DescString(r0)) => l0 == r0,
            (Self::Integer4Byte(l0), Self::Integer4Byte(r0)) => l0 == r0,
            (Self::Float4Byte(l0), Self::Float4Byte(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Cursable for TlvValue {
    fn read(&mut self, cursor: &mut std::io::Cursor<&mut [u8]>) {
        match self {
            TlvValue::DescString(value) => value.read(cursor),
            TlvValue::Integer4Byte(value) => value.read(cursor),
            TlvValue::Float4Byte(value) => value.read(cursor),
        }
    }

    fn write(&mut self, cursor: &mut std::io::Cursor<&mut [u8]>) {
        match self {
            TlvValue::DescString(value) => value.write(cursor),
            TlvValue::Float4Byte(value) => value.write(cursor),
            TlvValue::Integer4Byte(value) => value.write(cursor),
        }
    }
}