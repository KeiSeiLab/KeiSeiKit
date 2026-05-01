//! Session + message operations.
//!
//! Multi-schema convergence (2026-04-23): BOTH sessions and messages
//! now flow through `kei_entity_store::verbs::*`. `start_session` uses
//! `create` against `SESSIONS_SCHEMA` (TextPk + TextArchiveEnum);
//! `archive_session` uses `archive`; `get_session` uses `get`;
//! `save_message` uses `create` against `MESSAGES_SCHEMA`.
//!
//! Only the per-message aggregate update on `chat_sessions`
//! (message_count / total_tokens / total_cost) stays bespoke — the
//! engine has no "update-on-related-insert" verb.

use crate::schema::{MESSAGES_SCHEMA, SESSIONS_SCHEMA};
use crate::store::Store;
use anyhow::{anyhow, Result};
use chrono::Utc;
use kei_entity_store::verbs::{archive as v_archive, create as v_create, get as v_get};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub project: String,
    pub title: String,
    pub model: String,
    pub status: String,
    pub message_count: i64,
    pub total_tokens: i64,
    pub total_cost: f64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: i64,
    pub session_id: String,
    pub role: String,
    pub content: String,
    pub tokens_in: i64,
    pub tokens_out: i64,
    pub cost: f64,
    pub created_at: i64,
}

pub fn start_session(store: &Store, project: &str, title: &str, model: &str) -> Result<String> {
    let id = uuid::Uuid::new_v4().to_string();
    v_create::run(
        store.conn(),
        &SESSIONS_SCHEMA,
        json!({ "id": id, "project": project, "title": title, "model": model }),
    )
    .map_err(|e| anyhow!("{e}"))?;
    Ok(id)
}

pub fn save_message(store: &Store, msg: &ChatMessage) -> Result<i64> {
    let now = Utc::now().timestamp();
    let payload = json!({
        "session_id": msg.session_id,
        "role": msg.role,
        "content": msg.content,
        "tokens_in": msg.tokens_in,
        "tokens_out": msg.tokens_out,
        "cost": msg.cost,
        "created_at": msg.created_at,
    });
    let v = v_create::run(store.conn(), &MESSAGES_SCHEMA, payload)
        .map_err(|e| anyhow!("{e}"))?;
    let id = v["id"]
        .as_i64()
        .ok_or_else(|| anyhow!("missing id in create response"))?;
    bump_session_totals(store, &msg.session_id, msg.tokens_in + msg.tokens_out, msg.cost, now)?;
    Ok(id)
}

/// Bespoke aggregate update — engine has no "increment-on-related-insert"
/// verb. Keeps the per-session counters in sync with what was just
/// inserted into chat_messages.
fn bump_session_totals(
    store: &Store,
    session_id: &str,
    tokens_delta: i64,
    cost_delta: f64,
    now: i64,
) -> Result<()> {
    store.conn().execute(
        "UPDATE chat_sessions
            SET message_count = message_count + 1,
                total_tokens  = total_tokens + ?1,
                total_cost    = total_cost + ?2,
                updated_at    = ?3
          WHERE id = ?4",
        params![tokens_delta, cost_delta, now, session_id],
    )?;
    Ok(())
}

pub fn archive_session(store: &Store, session_id: &str) -> Result<()> {
    v_archive::run(store.conn(), &SESSIONS_SCHEMA, json!({ "id": session_id }))
        .map_err(|e| anyhow!("{e}"))?;
    Ok(())
}

pub fn get_session(store: &Store, id: &str) -> Result<Option<ChatSession>> {
    match v_get::run(store.conn(), &SESSIONS_SCHEMA, json!({ "id": id })) {
        Ok(v) => Ok(Some(session_from_json(v)?)),
        Err(e) if e.exit_code() == 2 => Ok(None),
        Err(e) => Err(anyhow!("{e}")),
    }
}

fn session_from_json(v: Value) -> Result<ChatSession> {
    let obj = v
        .as_object()
        .ok_or_else(|| anyhow!("expected object in get response"))?;
    let s = |k: &str| obj.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let i = |k: &str| obj.get(k).and_then(|x| x.as_i64()).unwrap_or(0);
    let f = |k: &str| obj.get(k).and_then(|x| x.as_f64()).unwrap_or(0.0);
    Ok(ChatSession {
        id: s("id"),
        project: s("project"),
        title: s("title"),
        model: s("model"),
        status: s("status"),
        message_count: i("message_count"),
        total_tokens: i("total_tokens"),
        total_cost: f("total_cost"),
        created_at: i("created_at"),
        updated_at: i("updated_at"),
    })
}
