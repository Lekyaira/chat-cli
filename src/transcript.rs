use crate::chat_backend::Message;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

/// Return the default directory used for storing transcripts.
fn data_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("XDG_DATA_HOME") {
        Some(PathBuf::from(dir).join("chat_cli"))
    } else if let Ok(home) = std::env::var("HOME") {
        Some(PathBuf::from(home).join(".local/share/chat_cli"))
    } else {
        None
    }
}

/// Path to the `last.jsonl` transcript file used for autoloading the previous
/// conversation.
pub fn default_transcript_path() -> Option<PathBuf> {
    data_dir().map(|d| d.join("last.jsonl"))
}

/// Load a transcript from the given JSONL file. Each line should contain a
/// serialized `Message`.
pub fn load_transcript(path: &Path) -> Result<Vec<Message>, Box<dyn std::error::Error + Send + Sync>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut messages = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let msg: Message = serde_json::from_str(&line)?;
        messages.push(msg);
    }
    Ok(messages)
}

/// Append a single message to the transcript at `path` as JSONL.
pub fn append_message(path: &Path, message: &Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    let line = serde_json::to_string(message)?;
    writeln!(file, "{}", line)?;
    Ok(())
}

/// Update the `last.jsonl` symlink to point to the provided transcript path.
pub fn update_last_symlink(target: &Path) -> std::io::Result<()> {
    if let Some(dir) = data_dir() {
        std::fs::create_dir_all(&dir)?;
        let link = dir.join("last.jsonl");
        let _ = std::fs::remove_file(&link);
        #[cfg(unix)]
        std::os::unix::fs::symlink(target, &link)?;
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(target, &link)?;
    }
    Ok(())
}
