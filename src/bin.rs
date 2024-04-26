use mylib;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:5555".parse().unwrap();
    let mut server = mylib::Server::new(addr, mylib::TestHandler::new());
    server.listen().await;
}
