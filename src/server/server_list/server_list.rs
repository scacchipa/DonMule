use std::io::Cursor;

use bytes::Bytes;

use crate::traits::buf_reader_ext::ByteReader;

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

    pub async fn fetch_server_vec(url: &str) -> Result<ServerVec, Box<dyn std::error::Error>> {

        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;

        let mut server_entries = ServerVec::new();
        server_entries.parse_server_stream(bytes)?;

        return Ok(server_entries);
    }

    fn parse_server_stream(&mut self, bytes: Bytes) -> Result<(), Box<dyn std::error::Error>> {
    
        let mut cursor = Cursor::new(bytes);
        let _ = cursor.read_u8()?; //format

        let count = cursor.read_u32_le()?;
        println!("Server entry count: {}", count);

        for _ in 0..count {
            let mut server_entry = ServerEntry::new();

            server_entry.load_entry(&mut cursor);
            self.servers.push(server_entry);
        }

        Ok(())
    }

    pub fn to_string(&mut self) -> String {
        
        let mut string = format!("ServerVec: server count: {}\n", self.servers.len());
        
        for server in self.servers.iter_mut() {
            string += &format!("{}\n", server.to_string());
        }
        
        return string;
    }
}