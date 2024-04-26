
/*
 * Types that show
 *
*/
pub type Request = String;
pub type Response = String;

pub trait Handler {
    fn handle_request(&self, message: Request) -> Response;
}