use std::io::{Cursor, Error};

use crate::{dtos::tlv::{integer_1_byte::Integer1Byte, integer_4_byte::Integer4Byte}, traits::cursable::Cursable};

use super::header::Header;

pub struct MsgContainer<T: Cursable> {
    pub header: Header,
    pub container: T,
}

impl<T: Cursable> MsgContainer<T> {
    pub fn new(protocol: u8, cont: T) -> Self {
        let mut container = MsgContainer { 
            header: Header {
                protocol: Integer1Byte::new(protocol),
                msg_size: Integer4Byte::new(cont.len() as u32),
            },
            container: cont,
        };

        container.header.msg_size = Integer4Byte::new(container.container.len() as u32);
        return container;
    }
}

impl<T: Cursable> Cursable for MsgContainer<T> {
    fn write(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error> {
        let mut size = self.header.write(cursor)?;
        size += self.container.write(cursor)?;

        return Ok(size);
    }

    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) -> Result<usize, Error> {
        let header_size = self.header.read(cursor)?;
        let container_size = self.container.read(cursor)?;


        if self.header.msg_size.value != container_size as u32 {
            self.header.msg_size = Integer4Byte::new(container_size as u32);
        }

        return Ok(header_size + container_size);
    }

    fn len(&self) -> usize {
        return self.header.len() + self.container.len();
    }
}