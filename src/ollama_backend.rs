use crate::chat_backend::{ChatBackend, Message};
use async_openai::{Client, config::OpenAIConfig, types::{
    ChatCompletionRequestAssistantMessageArgs,
    ChatCompletionRequestMessage,
    ChatCompletionRequestSystemMessageArgs,
    ChatCompletionRequestUserMessageArgs,
    CreateChatCompletionRequestArgs,
}};
use async_trait::async_trait;

/// Chat backend that talks to a locally running Ollama server via OpenAI-compatible API.
pub struct OllamaBackend {
    client: Client<OpenAIConfig>,
    model: String,
}

impl OllamaBackend {
    /// Create a new backend targeting the given model.
    pub fn new(model: impl Into<String>) -> Self {
        let config = OpenAIConfig::new()
            .with_api_base("http://localhost:11434/v1")
            .with_api_key("none");
        let client = Client::with_config(config);
        Self { client, model: model.into() }
    }
}

#[async_trait]
impl ChatBackend for OllamaBackend {
    async fn chat(&self, messages: &[Message]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let mut req_messages = Vec::new();
        for m in messages {
            let msg = match m.role.as_str() {
                "system" => ChatCompletionRequestMessage::System(
                    ChatCompletionRequestSystemMessageArgs::default()
                        .content(m.content.clone())
                        .build()?
                ),
                "assistant" => ChatCompletionRequestMessage::Assistant(
                    ChatCompletionRequestAssistantMessageArgs::default()
                        .content(m.content.clone())
                        .build()?
                ),
                _ => ChatCompletionRequestMessage::User(
                    ChatCompletionRequestUserMessageArgs::default()
                        .content(m.content.clone())
                        .build()?
                ),
            };
            req_messages.push(msg);
        }

        let request = CreateChatCompletionRequestArgs::default()
            .model(self.model.clone())
            .messages(req_messages)
            .build()?;

        let response = self.client.chat().create(request).await?;
        let reply = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();
        Ok(reply)
    }
}
