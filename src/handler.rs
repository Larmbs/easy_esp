/*
 * Types that show
 *
*/
pub type Request = String;
pub type Response = String;
pub type ServerRequest = String;

pub trait Handler {
    fn handle_request(&self, message: Request) -> (Response, ServerRequest);
}

pub struct TestHandler {}

impl TestHandler {
    pub fn new() -> Self {
        TestHandler {}
    }
}
impl Handler for TestHandler {
    fn handle_request(&self, message: Request) -> (Response, ServerRequest) {
        println!("[Handler] received a message ({})", message);

        (String::from("Hi back"), String::from(""))
    }
}

unsafe impl Sync for TestHandler {}
unsafe impl Send for TestHandler {}
