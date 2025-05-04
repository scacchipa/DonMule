use server::server_list::server_list::ServerVec;

mod impls;
mod traits;
mod server;

async fn download_archivo_async(url: &str, dest: &str) -> Result<(), Box<dyn std::error::Error>> {

    let request = reqwest::get(url).await?;
    let content = request.bytes().await?;

    let mut server_vec = ServerVec::fetch_server_vec(url).await?;
    

    println!("Fetched content: {:?}", server_vec.to_string());

    println!("The download was completed: {}", dest);
    Ok(())
}

#[tokio::main]
async fn main() {
    // let mut server_list = server::server_list::server_list::ServerVec::new();
    // let _ = server_list.load_server_list("./src/server (1).met");

    let url = "http://upd.emule-security.org/server.met";

    download_archivo_async(url, url).await;

    
}