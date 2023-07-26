use serde::{Serialize, Deserialize};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Client {
    pub name: String,
    pub addr: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub id: u32,
    pub name: String,
    pub timestamp: String,
    pub content: String,
}

impl Message {
    pub fn new(id: u32, name: String, timestamp: String, content: String) -> Message {
        Message {id, name, timestamp, content}
    }
}
