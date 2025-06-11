mod cli;
mod chat_backend;
mod ollama_backend;

use clap::Parser;
use std::io::{self, Write};
use chat_backend::{ChatBackend, Message};
use ollama_backend::OllamaBackend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = cli::Cli::parse();

    // Initialize the chat backend using the requested model
    let backend = OllamaBackend::new(cli.model.clone());
    let mut messages: Vec<Message> = Vec::new();

    if let Some(path) = cli.new.as_ref() {
        println!("Starting new conversation at {:?}", path);
    }

    if let Some(path) = cli.load.as_ref() {
        println!("Loading conversation from {:?}", path);
    }

    println!("Using model: {}", cli.model);

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush()?;
        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            break;
        }
        let line = input.trim_end();
        if line == "/exit" {
            break;
        }
        messages.push(Message { role: "user".into(), content: line.to_string() });
        let reply = backend.chat(&messages).await?;
        println!("{}", reply);
        messages.push(Message { role: "assistant".into(), content: reply });
    }

    Ok(())
}
