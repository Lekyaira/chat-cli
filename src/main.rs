use chat_cli::{cli, chat_backend, ollama_backend, transcript};

use clap::Parser;
use std::io::{self, Write};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use chat_backend::{ChatBackend, Message};
use ollama_backend::OllamaBackend;
use transcript::{append_message, default_transcript_path, load_transcript, update_last_symlink};
use ctrlc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = cli::Cli::parse();

    // Initialize the chat backend using the requested model
    let backend = OllamaBackend::new(cli.model.clone(), cli.color);
    let mut messages: Vec<Message> = Vec::new();
    let running = Arc::new(AtomicBool::new(true));
    {
        let r = running.clone();
        ctrlc::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })?;
    }
    let mut transcript_path = if let Some(p) = cli.new.clone() {
        Some(p)
    } else if let Some(p) = cli.load.clone() {
        Some(p)
    } else {
        default_transcript_path()
    };

    if cli.new.is_none() && cli.load.is_none() {
        if let Some(path) = transcript_path.clone() {
            if path.exists() {
                match load_transcript(&path) {
                    Ok(msgs) => {
                        println!("Loaded conversation from {:?}", path);
                        messages = msgs;
                    }
                    Err(e) => eprintln!("Failed to load transcript {:?}: {}", path, e),
                }
            }
        }
    }

    if let Some(path) = cli.new.as_ref() {
        println!("Starting new conversation at {:?}", path);
        messages.clear();
        transcript_path = Some(path.clone());
        if let Err(e) = update_last_symlink(path) {
            eprintln!("Failed to update last symlink: {}", e);
        }
    }

    if let Some(path) = cli.load.as_ref() {
        println!("Loading conversation from {:?}", path);
        transcript_path = Some(path.clone());
        match load_transcript(path) {
            Ok(msgs) => messages = msgs,
            Err(e) => eprintln!("Failed to load transcript {:?}: {}", path, e),
        }
    }

    println!("Using model: {}", cli.model);

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        if !running.load(Ordering::SeqCst) {
            break;
        }
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
        if !running.load(Ordering::SeqCst) {
            break;
        }
        let user_msg = Message { role: "user".into(), content: line.to_string() };
        messages.push(user_msg.clone());
        if let Some(path) = transcript_path.as_ref() {
            if let Err(e) = append_message(path, &user_msg) {
                eprintln!("Failed to save message: {}", e);
            }
        }
        let reply = match backend.chat(&messages).await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Backend error: {}", e);
                continue;
            }
        };
        let assistant_msg = Message { role: "assistant".into(), content: reply };
        messages.push(assistant_msg.clone());
        if let Some(path) = transcript_path.as_ref() {
            if let Err(e) = append_message(path, &assistant_msg) {
                eprintln!("Failed to save message: {}", e);
            }
        }
    }

    Ok(())
}
