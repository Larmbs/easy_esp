use easy_esp::{Request, RequestHandler, Server, Response, ServerCMD};
use std::net::SocketAddr;


// Creating a obj that implements request handler for chat room
struct ChatRoomHandler {
    clients: Vec<SocketAddr>,
}
impl ChatRoomHandler {
    fn new() -> Self {
        ChatRoomHandler {
            clients: vec![],
        }
    }
}
impl RequestHandler for ChatRoomHandler {
    fn handle_request(&mut self, request: Request, origin: SocketAddr) -> (Response, Option<ServerCMD>) {

        let response = format!("Ok");

        let cmd = ServerCMD::SendAll(format!("{}: {}", origin, request));
        (response, Some(cmd))
    }
    
    fn client_connect(&mut self, addr: SocketAddr) -> Option<ServerCMD> {
        // Adds client to client list
        self.clients.push(addr);
        
        Some(ServerCMD::SendAll(format!("{} Connected to chat", addr)))
    }
    
    fn client_disconnect(&mut self, addr: SocketAddr) -> Option<ServerCMD> {
        // Removes addr if it matches disconnected one
        self.clients.retain(|&x| x != addr);

        Some(ServerCMD::SendAll(format!("{} Disconnect from chat", addr)))
    }
}

#[tokio::main]
async fn main() {
    // Getting a socket addr obj
    let addr: SocketAddr = "127.0.0.1:5555".parse().expect("Could not parse ip addr");

    // Creating instance of a handler
    let handler = ChatRoomHandler::new();

    // Creating a server obj
    let mut server = Server::new(addr, handler);

    // Start server listener
    server.listen().await;
}
