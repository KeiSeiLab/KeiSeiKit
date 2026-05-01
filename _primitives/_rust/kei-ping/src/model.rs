// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Heartbeat record + query filter. One file, no dependencies on backends.

use serde::{Deserialize, Serialize};

/// One agent's "I'm alive, doing X" record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Heartbeat {
    pub agent_id: String,            // unique per worktree / session
    pub session_id: Option<String>,  // CLAUDE_SESSION_ID if known
    pub phase: String,               // free-form: "wave-7-auth-providers", "merge-ceremony", etc.
    pub dna: Option<String>,         // active DNA serial (RULE 0.12)
    pub branch: Option<String>,      // git branch the agent is on
    pub cwd: Option<String>,         // working directory
    pub last_seen_epoch: u64,        // seconds since UNIX epoch
    pub note: Option<String>,        // optional human-readable status
}

#[derive(Debug, Clone, Default)]
pub struct PingFilter {
    /// Only return heartbeats newer than this many seconds (TTL filter).
    /// Default 90s.
    pub max_age_s: Option<u64>,
    /// Only return heartbeats matching this phase prefix.
    pub phase_prefix: Option<String>,
    /// Only return heartbeats with branch matching exactly.
    pub branch: Option<String>,
}

impl PingFilter {
    pub fn alive(&self, h: &Heartbeat, now: u64) -> bool {
        let max = self.max_age_s.unwrap_or(90);
        if now.saturating_sub(h.last_seen_epoch) > max {
            return false;
        }
        if let Some(p) = &self.phase_prefix {
            if !h.phase.starts_with(p.as_str()) {
                return false;
            }
        }
        if let Some(b) = &self.branch {
            if h.branch.as_deref() != Some(b.as_str()) {
                return false;
            }
        }
        true
    }
}

pub fn now_epoch() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
