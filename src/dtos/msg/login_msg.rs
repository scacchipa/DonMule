use std::{io::{Cursor, Error, Write}, vec};

use crate::{dtos::{msg::header::Header, tlv::tlv::Tlv}, traits::cursable::Cursable};

/**
 * Protocol    Integer1Byte              Default: 0xE3
 * Size        Integer4Byte              The size of the message in bytes not including
 *                                         the header and size fields
 * Packet Type [Ubyte; 1]                Default: 0x01. Value of the OP LOGINREQUEST opcode
 * User Hash   [UByte; 16]               Details about user hash can be found in section 1.4
 * Client ID   [Ubyte; 4]                Default: 0x0u. The client ID that is sent on first connection
 *                                         is usually zero.
 * TCP Port    2-Byte number             Default: 4662. The TCP port used by the client, configurable
 * Tag Count   Integer4Byte              Default: 4. The number of tags following in the message
 * Name Tag    TLV (0x02; 0x01)           Default: NA. The user’s nickname (configurable in the software).
 * Version Tag TLV (0x03; 0x11)          Default: 0x3C. The eDonkey version supported by the client.
 * Port Tag    TLV (0x03; 0X0F)          Default: 4662. The TCP port used by the client.
 * Flags Tag 8 TLV (0x03; 0x20)          Default: 0x01. The tag is an integer tag and the tag name is 
 *                                         an integer of value 0x20
 */

struct LoginMsg {
    packet_type: u8,
    user_hash: [u8; 16],
    client_ip: [u8; 4],
    client_port: u16,
    tag_count: u32,
    tags: Vec<Tlv>
}

impl Cursable for LoginMsg {
    fn write(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {

        let vect: Vec<u8> = Vec::new();
        let mut vec_cursor = Cursor::new(vect);

        let mut size = vec_cursor.write(&[self.packet_type])?;
        size += cursor.write(&self.client_ip)?;
        size += cursor.write(&self.client_port.to_le_bytes())?;
        size += cursor.write(&self.tag_count.to_le_bytes())?;
        for tag in self.tags.iter_mut() {
            size += tag.write(cursor)?;
        };

    
        return Ok(size);
    }

    fn read(&mut self, cursor: &mut std::io::Cursor<&mut [u8]>) -> Result<usize, Error> {
        
        return Ok(0);

    }
}