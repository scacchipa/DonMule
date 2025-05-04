mod impls;
mod traits;
mod server;

fn main() {
    let mut server_list = server::server_list::server_list::ServerVec::new();
    let _ = server_list.load_server_list("./src/server (1).met");
}