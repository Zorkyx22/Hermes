use std::fmt;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

pub enum InputMode {
    Normal,
    Editing,
}
pub enum UserAction {
    Join,
    Leave,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum SystemMessageStatus {
    Error,
    Information,
}
#[derive(Serialize, Deserialize, Clone)]
pub enum MessageType {
    Chat,
    System,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum CommandType {
    Rename,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    time: DateTime<Utc>,
    user: String,
    message: String,
    msg_type: MessageType,
}
impl ChatMessage {
    pub fn new(user: String, message:String, msg_type: MessageType) -> ChatMessage {
        ChatMessage { time: Utc::now(), user: user, message: message, msg_type: msg_type }
    }
}
impl fmt::Display for ChatMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.msg_type {
            MessageType::Chat => {write!(f, "[{}] {} : {}", self.time, self.user, self.message)}
            MessageType::System => {write!(f, "{}", self.message)}
        }
    }
}

