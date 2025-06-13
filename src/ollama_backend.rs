use crate::chat_backend::{ChatBackend, Message};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
    Client,
};
use futures_util::StreamExt;
use std::io::{self, Write};
use async_trait::async_trait;
use colored::Colorize;

/// Chat backend that talks to a locally running Ollama server via OpenAI-compatible API.
pub struct OllamaBackend {
    client: Client<OpenAIConfig>,
    model: String,
    color: bool,
}

impl OllamaBackend {
    /// Create a new backend targeting the given model.
    pub fn new(model: impl Into<String>, color: bool) -> Self {
        let config = OpenAIConfig::new()
            .with_api_base("http://media:11434/v1")
            .with_api_key("none");
        let client = Client::with_config(config);
        Self { client, model: model.into(), color }
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
            .stream(true)
            .build()?;

        let mut stream = self.client.chat().create_stream(request).await?;
        let mut reply = String::new();
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            if let Some(content) = chunk
                .choices
                .first()
                .and_then(|c| c.delta.content.clone())
            {
                if self.color {
                    print!("{}", content.green());
                } else {
                    print!("{}", content);
                }
                io::stdout().flush()?;
                reply.push_str(&content);
            }
        }
        println!();
        Ok(reply)
    }
}
