use serde_derive::{Serialize, Deserialize};
use serde_json;

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Message {
    pub header: Header,
    pub body: String,
}

impl Message {
    pub fn new(header: Header, body: String) -> Self {
        Message {
            header,
            body
        }
    }
}

#[derive(Clone, Debug)]
#[derive(Serialize, Deserialize)]
pub struct Header {
    pub status: u64,
    pub content_type: String,
    pub authorization: Option<String>,
}

impl Header {
    pub fn new(status: u64, content_type: String, authorization: Option<String>) -> Self {
        Header {
            status,
            content_type,
            authorization,
        }
    }
}

pub fn parse_message_from_bytes(raw_message: &[u8]) -> serde_json::Result<Message> {
    serde_json::from_slice(raw_message)
}

pub fn convert_to_json(message: Message) -> String {
    serde_json::to_string(&message).unwrap()
}

pub fn create_json_message(body: String, status: u64, header: Option<Header>) -> Message {
    if let Some(header) = header {
        Message::new(header, body)
    } else {
        let header = Header::new(status, "json".to_string(), None);
        Message::new(header, body)
    }
}
