use std::time::Duration;

use bytes::buf;
use tokio::{io::AsyncReadExt, net::TcpStream, time::timeout};

mod impls;
mod server;
mod traits;
mod dtos;


async fn download_archivo_async(url: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {
    // let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

    // let mut server_vec = ServerVec::fetch_server_vec(url).await?;
    return Ok(());
}

async fn connect_with_server(addr: &str) -> Result<TcpStream, Box<dyn std::error::Error>> {
    match timeout(
        Duration::from_secs(10),
        TcpStream::connect("45.82.80.155:5687"))
    .await {
        Ok(Ok(stream)) => return Ok(stream),
        Ok(Err(e)) => return Err(Box::new(e)),
        Err(e) => return Err(Box::new(e)),
    }
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // let mut server_list = server::server_list::server_list::ServerVec::new();
    // let _ = server_list.load_server_list("./src/server (1).met");

    // let url = "http://upd.emule-security.org/server.met";

    // download_archivo_async(url, url).await;

    let tcp_strem_result = connect_with_server("45.82.80.155:5687").await;

    let protocol: u8 = 0xE3u8;
    let size : u32 = 1234;
    let user_hash: [u8; 16] = [87, 153, 220, 14, 174, 112, 241, 49, 156, 84, 157, 15, 250, 34, 111, 86];
    let client_id: u32 = 0x0u32;
    let tcp_port: u16 = 4662u16;
    let tag_count: u32 = 4;
    let name_tag: &str = "Pablo";
    let version: u32 = 0x3Cu32;
    let tcp_port_2: u32 = 4662u32;
    let flags: u32 = 0x01u32;


    let mut tcp_stream = match tcp_strem_result {
        Ok(stream) => {
            print!("Connected.");
            stream
        }
        Err(e) => {
            println!("Error {}", e);
            return Err(e);
        }
    };

    let mut buf = [0u8; 32];
    let ip = tcp_stream.local_addr().unwrap().ip();
    let result = tcp_stream.read(&mut buf);

    match ip {
        std::net::IpAddr::V4(v4) => println!("{:03?}", v4.octets()),
        std::net::IpAddr::V6(v6) => println!("{:02X?}", v6.octets()),
    }

    Ok(())
}
