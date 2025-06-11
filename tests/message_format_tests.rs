use chat_cli::chat_backend::Message;

#[test]
fn message_serde_roundtrip() {
    let msg = Message { role: "user".into(), content: "hi".into() };
    let json = serde_json::to_string(&msg).unwrap();
    assert_eq!(json, "{\"role\":\"user\",\"content\":\"hi\"}");
    let back: Message = serde_json::from_str(&json).unwrap();
    assert_eq!(back, msg);
}
