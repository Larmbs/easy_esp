
/*
 * Types that show
 *
*/
pub type Request = String;
pub type Response = String;

pub trait Handler {
    fn handle_request(&self, message: Request) -> Response;
}


pub struct TestHandler {}

impl TestHandler {
    pub fn new() -> Self {
        TestHandler {
        }
    }
}
impl Handler for TestHandler {
    fn handle_request(&self, message: Request) -> Response {
        String::from("Hi back")
    }
}

unsafe impl Sync for TestHandler {}
unsafe impl Send for TestHandler {}

