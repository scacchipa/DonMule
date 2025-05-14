use std::{io::Cursor, time::Duration};

use dtos::{msg::{header::Header, login_msg::LoginMsg, msg_container::MsgContainer}, tlv::{integer_1_byte::Integer1Byte, integer_2_byte::Integer2Byte, integer_4_byte::Integer4Byte, string_2_byte_length::String2ByteLength}};
use tokio::{io::AsyncReadExt, net::TcpStream, time::timeout};
use traits::cursable::Cursable;

mod impls;
mod server;
mod traits;
mod dtos;


const PROTOCOL_ID_AMULE: u8 = 0xE3;
const PROTOCOL_ID_EMULE: u8 = 0xC5;


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
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut server_list = server::server_list::server_list::ServerVec::new();
    // let _ = server_list.load_server_list("./src/server (1).met");

    // let url = "http://upd.emule-security.org/server.met";

    // download_archivo_async(url, url).await;

    let tcp_strem_result = connect_with_server("45.82.80.155:5687").await?;


    let mut login_msg = LoginMsg {
        packet_type: Integer1Byte { value : 0xE3u8 },
        user_hash: [87, 153, 220, 14, 174, 112, 241, 49, 156, 84, 157, 15, 250, 34, 111, 86],
        client_ip: [0x0u8, 0x0u8, 0x0u8, 0x0u8],
        client_port: Integer2Byte { value: 4662 },
        name_tag: String2ByteLength::new("Pablo".into()),
        version_tag: Integer4Byte::new(0x3cf),
        port_tag: Integer4Byte::new(4662),
        flag_tag: Integer4Byte::new(0x1)
    };

    let mut msg = MsgContainer::<LoginMsg>::new(
        PROTOCOL_ID_AMULE,
        login_msg
    );

    let bytes = vec![0u8, 10];
    let mut cursor = Cursor::new(bytes);

    let size = msg.write(&mut cursor)?;

    println!("Size: {}", size);
    println!("{:02X?}", cursor.into_inner());

    // let mut tcp_stream = match tcp_strem_result {
    //     Ok(stream) => {
    //         print!("Connected.");
    //         stream
    //     }
    //     Err(e) => {
    //         println!("Error {}", e);
    //         return Err(e);
    //     }
    // };

    // let mut buf = [0u8; 32];
    // let ip = tcp_stream.local_addr().unwrap().ip();
    // let result = tcp_stream.read(&mut buf);

    // match ip {
    //     std::net::IpAddr::V4(v4) => println!("{:03?}", v4.octets()),
    //     std::net::IpAddr::V6(v6) => println!("{:02X?}", v6.octets()),
    // }

    Ok(())
}
