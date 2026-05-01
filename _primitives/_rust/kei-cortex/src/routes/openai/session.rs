//! Per-process session continuity store for the OpenAI surface.
//!
//! Two shapes share this module:
//!   * `X-Kei-Session-Id` (chat-completions) — opt-in continuity for an
//!     otherwise stateless endpoint. Caller passes the same id on each
//!     turn to retain message history.
//!   * `previous_response_id` (responses) — required-by-spec stateful
//!     continuity for the Responses API.
//!
//! Both keys point at the same `SessionRecord` shape because the agent
//! loop doesn't care which surface populated the history.
//!
//! Storage is in-memory only (`DashMap`). Persistence-across-restart
//! would mean wiring `kei-ledger`; deferred to Phase 1.2 per
//! `HERMES-MIGRATION-PLAN.md`.

use super::types::{ChatMessage, ResponseObject, Usage};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// One session = one ordered list of messages + last-response snapshot.
#[derive(Debug, Clone)]
pub struct SessionRecord {
    pub messages: Vec<ChatMessage>,
    pub last_response: Option<ResponseObject>,
}

impl SessionRecord {
    pub fn empty() -> Self {
        Self { messages: Vec::new(), last_response: None }
    }
}

/// Process-wide session map. Cheap to clone (Arc inside).
#[derive(Clone, Default)]
pub struct SessionStore {
    inner: Arc<DashMap<String, SessionRecord>>,
}

/// Process-singleton accessor. Each call returns a clone of the same
/// underlying `Arc<DashMap>` so handlers in different routers see one
/// consistent store. Lazily initialised on first call.
pub fn global() -> SessionStore {
    use once_cell::sync::Lazy;
    static STORE: Lazy<SessionStore> = Lazy::new(SessionStore::new);
    STORE.clone()
}

impl SessionStore {
    pub fn new() -> Self { Self::default() }

    /// Look up a session by id; returns `None` if absent.
    pub fn get(&self, id: &str) -> Option<SessionRecord> {
        self.inner.get(id).map(|r| r.value().clone())
    }

    /// Replace (or insert) a session.
    pub fn put(&self, id: impl Into<String>, rec: SessionRecord) {
        self.inner.insert(id.into(), rec);
    }

    /// Remove a session, returning the prior value if any.
    pub fn delete(&self, id: &str) -> Option<SessionRecord> {
        self.inner.remove(id).map(|(_, v)| v)
    }

    /// Append messages to an existing session, creating it if needed.
    /// Used by chat-completions when `X-Kei-Session-Id` is supplied.
    pub fn append(&self, id: &str, new_msgs: Vec<ChatMessage>) {
        let mut entry = self.inner.entry(id.to_string()).or_insert_with(SessionRecord::empty);
        entry.messages.extend(new_msgs);
    }

    /// Number of stored sessions (test helper).
    #[cfg(test)]
    pub fn len(&self) -> usize { self.inner.len() }
}

/// Build a fresh `ResponseObject` skeleton. Caller fills `output` and
/// `usage` once the agent finishes.
pub fn new_response_skeleton(model: String, prev: Option<String>) -> ResponseObject {
    ResponseObject {
        id: format!("resp_{}", super::ids::short_id()),
        object: "response",
        created_at: unix_secs(),
        model,
        status: "completed".into(),
        output: Vec::new(),
        previous_response_id: prev,
        usage: Usage::default(),
    }
}

fn unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(role: &str, content: &str) -> ChatMessage {
        ChatMessage {
            role: role.into(),
            content: Some(content.into()),
            name: None, tool_call_id: None, tool_calls: None,
        }
    }

    #[test]
    fn put_get_delete_roundtrip() {
        let s = SessionStore::new();
        let mut rec = SessionRecord::empty();
        rec.messages.push(msg("user", "hi"));
        s.put("sess_1", rec);
        assert_eq!(s.get("sess_1").unwrap().messages.len(), 1);
        s.delete("sess_1");
        assert!(s.get("sess_1").is_none());
    }

    #[test]
    fn append_creates_then_extends() {
        let s = SessionStore::new();
        s.append("sess_a", vec![msg("user", "1")]);
        s.append("sess_a", vec![msg("assistant", "2")]);
        assert_eq!(s.get("sess_a").unwrap().messages.len(), 2);
        assert_eq!(s.len(), 1);
    }
}
