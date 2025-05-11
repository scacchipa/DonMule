use std::io::{Cursor, Error};

pub trait Cursable {
    fn write(&mut self, cursor: &mut Cursor<Vec<u8>>) -> Result<usize, Error>;
    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>) -> Result<usize, Error>;
}