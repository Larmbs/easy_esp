use easy_esp;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:5555".parse().unwrap();
    let mut server = easy_esp::Server::new(addr, easy_esp::ChatRoomHandler::new());
    server.listen().await;
}
