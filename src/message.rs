use serde_derive::{Deserialize, Serialize};


#[derive(Deserialize, Serialize)]
pub enum Message<'a> {
    /// ##Request Type
    /// requests store a request type being either Get or Post
    /// also stores content type to expect
    REQUEST(&'a str, &'a str),
    /// Response message type
    /// stores a status code as its first
    /// part in message and follows http
    /// status code guidelines
    RESPONSE(u16),
}
