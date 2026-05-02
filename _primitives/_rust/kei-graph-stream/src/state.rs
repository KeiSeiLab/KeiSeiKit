use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// Minimal info kept per alive agent (spawned, not yet done).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentInfo {
    pub id: String,
    pub subagent_type: String,
    pub model: String,
    pub ts: String,
}

/// Thread-safe map of currently alive agents.
pub struct AliveState {
    inner: Mutex<HashMap<String, AgentInfo>>,
}

impl AliveState {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }

    /// Insert or update an agent from a spawn event.
    pub fn insert(&self, event: &serde_json::Value) {
        let Some(id) = event["id"].as_str() else { return };
        let info = AgentInfo {
            id: id.to_string(),
            subagent_type: event["subagent_type"]
                .as_str()
                .unwrap_or("unknown")
                .to_string(),
            model: event["model"].as_str().unwrap_or("unknown").to_string(),
            ts: event["ts"].as_str().unwrap_or("").to_string(),
        };
        self.inner.lock().unwrap().insert(id.to_string(), info);
    }

    /// Remove an agent on done event.
    pub fn remove(&self, event: &serde_json::Value) {
        let Some(id) = event["id"].as_str() else { return };
        self.inner.lock().unwrap().remove(id);
    }

    /// Snapshot sorted newest-first (ISO8601 lexicographic on ts).
    pub fn snapshot(&self) -> Vec<AgentInfo> {
        let mut agents: Vec<AgentInfo> =
            self.inner.lock().unwrap().values().cloned().collect();
        agents.sort_by(|a, b| b.ts.cmp(&a.ts));
        agents
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn insert_and_snapshot() {
        let s = AliveState::new();
        s.insert(&json!({"id":"a1","subagent_type":"researcher","model":"sonnet","ts":"2026-05-02T13:00:00Z"}));
        s.insert(&json!({"id":"a2","subagent_type":"coder","model":"opus","ts":"2026-05-02T13:01:00Z"}));
        let snap = s.snapshot();
        assert_eq!(snap.len(), 2);
        assert_eq!(snap[0].id, "a2"); // newest first
    }

    #[test]
    fn remove_clears_agent() {
        let s = AliveState::new();
        s.insert(&json!({"id":"a1","subagent_type":"x","model":"y","ts":"2026-05-02T00:00:00Z"}));
        s.remove(&json!({"id":"a1"}));
        assert!(s.snapshot().is_empty());
    }

    #[test]
    fn snapshot_empty_initial() {
        let s = AliveState::new();
        assert!(s.snapshot().is_empty());
    }
}
