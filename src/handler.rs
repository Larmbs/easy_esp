use super::ServerCMD;

/*
 * Types that show
 *
*/
pub type Request = String;
pub type Response = String;

pub trait Handler {
    fn handle_request(&self, message: Request) -> (Response, Option<ServerCMD>);
}

pub struct TestHandler {}

impl TestHandler {
    pub fn new() -> Self {
        TestHandler {}
    }
}
impl Handler for TestHandler {
    fn handle_request(&self, message: Request) -> (Response, Option<ServerCMD>) {
        println!("[Handler] received a message ({})", message);

        (String::from("Hi back"), Some(ServerCMD::SendAll("This is sent to everyone".to_string())))
    }
}

unsafe impl Sync for TestHandler {}
unsafe impl Send for TestHandler {}
