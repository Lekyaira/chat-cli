mod cli;
mod chat_backend;
mod ollama_backend;

use clap::Parser;

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

    Ok(())
}
