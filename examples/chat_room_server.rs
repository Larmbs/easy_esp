use easy_esp::{Server, ServerCMD, message::{Message, create_json_message}, handler::RequestHandler};
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
    fn handle_request(&mut self, request: Message, origin: SocketAddr) -> (Message, Option<ServerCMD>) {

        let send_all_msg = create_json_message(format!("{}: {}", origin, request.body), None);
        let cmd: ServerCMD = ServerCMD::SendAll(send_all_msg);

        let send_back_msg = create_json_message(String::new(), None);
        (send_back_msg, Some(cmd))
    }
    
    fn client_connect(&mut self, addr: SocketAddr) -> Option<ServerCMD> {
        // Adds client to client list
        self.clients.push(addr);

        let msg = create_json_message(format!("{} Connected to chat", addr), None);
        Some(ServerCMD::SendAll(msg))
    }
    
    fn client_disconnect(&mut self, addr: SocketAddr) -> Option<ServerCMD> {
        // Removes addr if it matches disconnected one
        self.clients.retain(|&x| x != addr);

        let msg = create_json_message(format!("{} Disconnect from chat", addr), None);
        Some(ServerCMD::SendAll(msg))
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
