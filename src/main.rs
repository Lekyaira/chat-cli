mod cli;
mod chat_backend;
mod ollama_backend;

use clap::Parser;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

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
        println!("You said: {}", line);
    }

    Ok(())
}
