use std::{
    fs::File, io::{BufReader, Error}
};

use crate::traits::buf_reader_ext::BufReaderExt;

use super::server_entry::ServerEntry;

pub struct ServerVec {
    servers: Vec<ServerEntry>,
}

impl ServerVec {
    pub fn new() -> Self {
        ServerVec {
            servers: vec![]
        }
    }

    pub fn load_server_list(&mut self, path: &str) -> Result<(), Error> {
        let result = File::open(path);

        if result.is_err() {
            println!("El archivo no fue abierto {}", "some");
        }

        let mut buf_reader = BufReader::new(result.ok().unwrap());

        let _ = buf_reader.read_byte()?; //format

        let count = buf_reader.read_dword_le()?;

        println!("Server entry count: {}", count);

        for _ in 0..count {
            let mut server_entry = ServerEntry::new();

            server_entry.load_entry(&mut buf_reader);
            self.servers.push(server_entry);
        }

        Ok(())
    }
}