use async_trait::async_trait;

/// Represents a single chat message with a role and content.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct Message {
    pub role: String,
    pub content: String,
}

/// Trait for chat backends that can send and receive messages.
#[async_trait]
pub trait ChatBackend {
    /// Send the conversation history and receive the assistant's reply as a string.
    async fn chat(&self, messages: &[Message]) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
}
