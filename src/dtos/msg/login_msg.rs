use std::io::{Cursor, Error, ErrorKind, Read, Write};

use crate::{
    dtos::
        tlv::{
            integer_1_byte::Integer1Byte, integer_2_byte::Integer2Byte,
            integer_4_byte::Integer4Byte, string_2_byte_length::String2ByteLength, tlv::Tlv, tlv_value::TlvValue,
        }
    ,
    traits::cursable::Cursable,
};

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

macro_rules! invalid_data_error {
    ($msg: expr) => {
        Error::new(ErrorKind::InvalidData, $msg)
    };
}

macro_rules! extract_tlv_value {
    ($tag: expr, $variant: path) => {
        match $tag.tlv_value {
            $variant(val) => val,
            _ => return Err(invalid_data_error!("Wrong type in Tlv"))
        }
    };
}

pub struct LoginMsg {
    pub packet_type: Integer1Byte,
    pub user_hash: [u8; 16],
    pub client_ip: [u8; 4],
    pub client_port: Integer2Byte,
    pub name_tag: String2ByteLength,
    pub version_tag: Integer4Byte,
    pub port_tag: Integer4Byte,
    pub flag_tag: Integer4Byte,
}

impl Cursable for LoginMsg {
    fn write(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {
        let mut size = self.packet_type.write(cursor)?;
        size += cursor.write(&self.user_hash)?;
        size += cursor.write(&self.client_ip)?;
        size += self.client_port.write(cursor)?;

        let mut tag_count = Integer4Byte::new(2);
        size += tag_count.write(cursor)?;
        
        size += self.name_tag.write(cursor)?;
        size += self.version_tag.write(cursor)?;
        size += self.port_tag.write(cursor)?;
        size += self.flag_tag.write(cursor)?;
        return Ok(size);
    }

    fn read(&mut self, cursor: &mut std::io::Cursor<&mut [u8]>) -> Result<usize, Error> {
        
        let mut size = self.packet_type.read(cursor)?;
        size += cursor.read(&mut self.user_hash)?;
        size += cursor.read(&mut self.client_ip)?;
        size += self.client_port.read(cursor)?;

        let mut tag_count = Integer4Byte::new(0);
        size += tag_count.read(cursor)?;

        for _ in 0..(tag_count.value) {

            let mut tag = Tlv::empty();
            size += tag.read(cursor)?;

            match tag.tlv_name.value.as_slice() {
                [0x01] => self.name_tag = extract_tlv_value!(tag, TlvValue::DescString),
                [0x11] => self.version_tag = extract_tlv_value!(tag, TlvValue::Integer4Byte),
                [0x0F] => self.port_tag = extract_tlv_value!(tag, TlvValue::Integer4Byte),
                [0x20] => self.flag_tag = extract_tlv_value!(tag, TlvValue::Integer4Byte),
                _ => return Err(Error::new(ErrorKind::InvalidData, "Wrong type in Tlv"))
            }
        }

        return Ok(size);
    }

    fn len(&self) -> usize {
        return 
            self.packet_type.len() +
            self.user_hash.len() +
            self.client_ip.len() +
            self.client_port.len() +
            self.name_tag.len() +
            self.version_tag.len() +
            self.port_tag.len() +
            self.flag_tag.len()
    }
}


