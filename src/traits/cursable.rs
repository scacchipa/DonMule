use std::io::Cursor;

pub trait Cursable {
    fn read(&mut self, cursor: &mut Cursor<&mut [u8]>);
    fn write(&mut self, cursor: &mut Cursor<&mut [u8]>);
}