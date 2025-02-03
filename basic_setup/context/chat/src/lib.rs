/*
*                                         Chat  app
*
*       This is a simple chat application that allows users to send messages to the chat.
*       It is a basic example of how to use a chat application on the Calimero blockchain.
*
*   
*   meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> get_all_messages
*
*   meroctl --node-name node1 call --as <EXECUTOR_ID> <CONTEXT_ID> send_message --args '{"sender": "id", "content": "message text"}'
*/

use calimero_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use calimero_sdk::types::Error;
use calimero_sdk::{app, env};
use calimero_storage::collections::Vector;
use serde::{Serialize, Deserialize};

#[app::state]
#[derive(Default, BorshSerialize, BorshDeserialize)]
#[borsh(crate = "calimero_sdk::borsh")]
struct ChatApp {
    messages: Vector<Message>,
    message_sequence: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Serialize, Deserialize)]
struct Message {
    content: String,
    sender: String,
    sequence: u64,
}

#[app::logic]
impl ChatApp {
    #[app::init]
    pub fn init() -> ChatApp {
        env::log("Initializing minimal chat application");
        ChatApp {
            messages: Vector::new(),
            message_sequence: 0,
        }
    }

    /// Send a text message - CORRECTED ACCOUNT ID METHOD
    pub fn send_message(&mut self, sender: String, content: String) -> Result<u64, Error> {
        if content.len() > 280 {
            return Err(Error::msg("Message too long (max 280 characters)"));
        }
    
        let sequence = self.message_sequence;
        let message = Message {
            content: Self::sanitize_input(&content),
            sender, 
            sequence,
        };
        
        self.messages.push(message.clone())?;
        self.message_sequence += 1;
        
        env::log(&format!(
            "EVENT: message_sent,sender:{},sequence:{}",
            message.sender, message.sequence
        ));
    
        Ok(sequence)
    }

    /// Get all messages
    pub fn get_all_messages(&self) -> Result<Vec<Message>, Error> {
        let mut result = Vec::new();
        let len = self.messages.len()?;

        for i in 0..len {
            if let Ok(msg_opt) = self.messages.get(i) {
                if let Some(msg) = msg_opt {
                    result.push(msg);
                }
            }
        }
        Ok(result)
    }

    /// Sanitize input
    fn sanitize_input(input: &str) -> String {
        input
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('&', "&amp;")
            .replace('\n', "<br>")
    }
}
