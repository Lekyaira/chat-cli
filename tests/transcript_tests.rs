use chat_cli::{transcript::{append_message, load_transcript}, chat_backend::Message};
use tempfile::tempdir;

#[test]
fn transcript_roundtrip() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let dir = tempdir()?;
    let path = dir.path().join("log.jsonl");
    let msg1 = Message { role: "user".into(), content: "hi".into() };
    let msg2 = Message { role: "assistant".into(), content: "hello".into() };
    append_message(&path, &msg1)?;
    append_message(&path, &msg2)?;
    let msgs = load_transcript(&path)?;
    assert_eq!(msgs, vec![msg1, msg2]);
    Ok(())
}
