//! ID generators for the OpenAI surface.
//!
//! OpenAI uses prefixed-uuid ids for objects (`chatcmpl-...`, `resp_...`,
//! `run_...`). We mirror that convention so frontends that pattern-match
//! on the prefix continue to work.

use uuid::Uuid;

/// Short hex slug for embedding in object ids — first 24 hex chars of a
/// fresh v4 UUID (96 bits of entropy, plenty for a per-process namespace).
pub fn short_id() -> String {
    let u = Uuid::new_v4().simple().to_string();
    u.chars().take(24).collect()
}

/// `chatcmpl-<24hex>` — POST /v1/chat/completions completion id.
pub fn chat_completion_id() -> String {
    format!("chatcmpl-{}", short_id())
}

/// `call_<24hex>` — assistant.tool_calls[*].id.
pub fn tool_call_id() -> String {
    format!("call_{}", short_id())
}

/// `run_<24hex>` — POST /v1/runs response id.
pub fn run_id() -> String {
    format!("run_{}", short_id())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_have_expected_prefixes() {
        assert!(chat_completion_id().starts_with("chatcmpl-"));
        assert!(tool_call_id().starts_with("call_"));
        assert!(run_id().starts_with("run_"));
    }

    #[test]
    fn short_id_is_24_chars() {
        assert_eq!(short_id().len(), 24);
    }

    #[test]
    fn ids_are_unique_in_a_burst() {
        let a = run_id();
        let b = run_id();
        assert_ne!(a, b);
    }
}
