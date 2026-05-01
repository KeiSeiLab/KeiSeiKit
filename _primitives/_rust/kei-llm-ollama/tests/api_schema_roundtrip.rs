//! Wire-schema round-trip tests against fixtures matching Ollama's documented
//! JSON envelopes (`<https://github.com/ollama/ollama/blob/main/docs/api.md>`).

use kei_llm_ollama::{ChatResp, GenerateResp, Message, ModelEntry, TagsResp};

#[test]
fn tags_resp_roundtrip() {
    let raw = r#"{
        "models": [
            {
                "name": "qwen3:4b",
                "model": "qwen3:4b",
                "modified_at": "2025-10-06T18:37:52Z",
                "size": 4733124608,
                "digest": "deadbeef",
                "details": {
                    "parent_model": "",
                    "format": "gguf",
                    "family": "qwen3",
                    "families": ["qwen3"],
                    "parameter_size": "4.0B",
                    "quantization_level": "Q4_K_M"
                }
            }
        ]
    }"#;
    let parsed: TagsResp = serde_json::from_str(raw).expect("decode tags");
    assert_eq!(parsed.models.len(), 1);
    assert_eq!(parsed.models[0].name, "qwen3:4b");
    let again = serde_json::to_string(&parsed).expect("encode tags");
    let reparsed: TagsResp = serde_json::from_str(&again).expect("re-decode tags");
    assert_eq!(parsed, reparsed);
}

#[test]
fn generate_resp_roundtrip() {
    let raw = r#"{
        "model": "qwen3:4b",
        "created_at": "2025-04-01T00:00:00Z",
        "response": "Hello, world!",
        "done": true,
        "eval_count": 7,
        "eval_duration": 12345
    }"#;
    let parsed: GenerateResp = serde_json::from_str(raw).expect("decode generate");
    assert!(parsed.done);
    assert_eq!(parsed.response, "Hello, world!");
    assert_eq!(parsed.eval_count, Some(7));
    let again = serde_json::to_string(&parsed).expect("encode generate");
    let reparsed: GenerateResp = serde_json::from_str(&again).expect("re-decode generate");
    assert_eq!(parsed, reparsed);
}

#[test]
fn chat_resp_roundtrip() {
    let raw = r#"{
        "model": "qwen3:4b",
        "created_at": "2025-04-01T00:00:00Z",
        "message": {"role": "assistant", "content": "Hi!"},
        "done": true,
        "eval_count": 3
    }"#;
    let parsed: ChatResp = serde_json::from_str(raw).expect("decode chat");
    assert_eq!(parsed.message.role, "assistant");
    assert_eq!(parsed.message.content, "Hi!");
    let again = serde_json::to_string(&parsed).expect("encode chat");
    let reparsed: ChatResp = serde_json::from_str(&again).expect("re-decode chat");
    assert_eq!(parsed, reparsed);
}

#[test]
fn message_round_trip_all_three_roles() {
    for role in ["system", "user", "assistant"] {
        let m = Message {
            role: role.into(),
            content: "x".into(),
        };
        let raw = serde_json::to_string(&m).unwrap();
        let back: Message = serde_json::from_str(&raw).unwrap();
        assert_eq!(m, back);
    }
}

#[test]
fn model_entry_minimal_decode() {
    // Real-world: the daemon may emit an empty `details` payload.
    let raw = r#"{"name": "tiny:latest"}"#;
    let parsed: ModelEntry = serde_json::from_str(raw).expect("decode minimal model");
    assert_eq!(parsed.name, "tiny:latest");
    assert_eq!(parsed.size, 0);
    assert!(parsed.digest.is_empty());
}
