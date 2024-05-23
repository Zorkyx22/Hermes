use std::error::Error;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt};
use crate::datatypes::{InputMode, ChatMessage, MessageType, UserAction};

/// App holds the state of the application
pub struct App {
    /// Current value of the input box
    pub input: String,
    /// Position of cursor in the editor area.
    pub character_index: usize,
    /// Current input mode
    pub input_mode: InputMode,
    /// History of recorded messages
    pub messages: Vec<ChatMessage>,
    /// Connected Room Stream
    pub server: TcpStream,
    /// User's handle
    pub username: String,
}

impl App {
    pub const fn new(server_input: TcpStream, user: String) -> Self {
        Self {
            server: server_input,
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            character_index: 0,
            username: user,
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    pub fn byte_index(&mut self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    pub fn reset_cursor(&mut self) {
        self.character_index = 0;
    }
    fn build_rich_message(&mut self) -> ChatMessage {
        ChatMessage::new(self.username.clone(), self.input.clone(), MessageType::Chat)
    }

    pub async fn submit_message(&mut self) -> Result<(), Box<dyn Error>>{
        let message = serde_json::to_string(&self.build_rich_message())?;
        self.server.write_all(&message.into_bytes()).await?;
        self.input.clear();
        self.reset_cursor();
        Ok(())
    }
    
    pub fn update_message_queue(&mut self) { 
        let mut data = vec![0; 1024];
        match self.server.try_read(&mut data){
            Ok(_n) => {
                // Process the data here
                let raw_data = std::str::from_utf8(&data[..]).expect("error parsing received message").to_string();
                let message: ChatMessage = serde_json::from_str(&raw_data).expect(&format!("error deserializing json {}", raw_data.clone()));
                self.messages.push(message);
            },
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data was available, so we wait and try again
            },
            Err(_) => {
                // Other errors. I'm not yet sure how to handle them.
            }
        };
    }

    pub async fn send_system_message(&mut self, action: UserAction) -> Result<(), Box<dyn Error>> {
        let mut message: String = String::new();
        match action {
            UserAction::Join => {message = format!("-=[{} has joined!]=-", self.username.clone());}
            UserAction::Leave => {message = format!("-=[{} has left!]=-", self.username.clone());}
        }
        let msg_object = serde_json::to_string(&ChatMessage::new(self.username.clone(), message, MessageType::System)).expect("JSON Serialization error");
        self.server.write_all(&msg_object.into_bytes()).await?;
        Ok(())
    }

}
