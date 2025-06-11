use clap::Parser;
use std::path::PathBuf;

/// Command line arguments for the chat CLI.
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Start a new conversation log at <FILE> and clear history
    #[arg(long, value_name = "FILE")]
    pub new: Option<PathBuf>,

    /// Load an existing conversation log from <FILE>
    #[arg(long, value_name = "FILE")]
    pub load: Option<PathBuf>,

    /// Override the default model name (default: mistral)
    #[arg(long, default_value = "mistral", value_name = "MODEL")]
    pub model: String,
}
