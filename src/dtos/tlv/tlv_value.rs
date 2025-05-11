use std::io::{Cursor, Error};

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
    fn write(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {
        let mut size: usize = 0;

        match self {
            TlvValue::DescString(value) => size += value.write(cursor)?,
            TlvValue::Integer4Byte(value) => size += value.write(cursor)?,
            TlvValue::Float4Byte(value) => size += value.write(cursor)?,
        }

        return Ok(size);
    }

    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) -> Result<usize, Error> {
        let mut size: usize = 0;

        match self {
            TlvValue::DescString(value) => size += value.read(cursor)?,
            TlvValue::Float4Byte(value) => size += value.read(cursor)?,
            TlvValue::Integer4Byte(value) => size += value.read(cursor)?,
        }

        return Ok(size);
    }
}