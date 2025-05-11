

use std::io::{Cursor, Error};

use crate::{dtos::tlv::{integer_1_byte::Integer1Byte, integer_4_byte::Integer4Byte}, traits::cursable::Cursable};

/**
 * All messages have a 6 byte header that has the following structure:
 * 1. protocol - A single byte protocol ID - 0xE3 for eDonkey and 0xC5 for eMule.
 * 2. size - 4 byte message size - the message size in bytes not including the header.
 * 3. type - A single byte type - a unique message ID.
 */
pub struct Header {
    protocol: Integer1Byte,
    msg_size: Integer4Byte
}

impl Cursable for Header {
    fn read(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {
        let mut size = self.protocol.read(cursor)?;
        size += self.msg_size.read(cursor)?;

        return Ok(size);
    }

    fn write(&mut self, cursor: &mut std::io::Cursor<&mut [u8]>) -> Result<usize, Error> {
        let mut size = self.protocol.write(cursor)?;
        size += self.msg_size.write(cursor)?;
        return Ok(size);
    }
}