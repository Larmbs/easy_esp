use easy_esp::{Request, RequestHandler, Server, Response, ServerCMD};
use std::net::SocketAddr;


// Creating a obj that implements request handler for chat room
struct ChatRoomHandler {}
impl RequestHandler for ChatRoomHandler {
    fn handle_request(&self, request: Request, origin: SocketAddr) -> (Response, Option<ServerCMD>) {
        let response = format!("Ok");

        let cmd = ServerCMD::SendAll(format!("{}: {}", origin, request));
        (response, Some(cmd))
    }
}

#[tokio::main]
async fn main() {
    // Getting a socket addr obj
    let addr: SocketAddr = "127.0.0.1:5555".parse().expect("Could not parse ip addr");

    // Creating instance of a handler
    let handler = ChatRoomHandler {};

    // Creating a server obj
    let mut server = Server::new(addr, handler);

    // Start server listener
    server.listen().await;
}
