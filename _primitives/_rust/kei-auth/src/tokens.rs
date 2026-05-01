//! Token issue / verify / revoke.
//!
//! Token layout (URL-safe, no padding):
//!   `<b64(payload_json)>.<b64(hmac)>`
//! Payload contains {tid, user_id, project, scope, expires_at}.
//! The db keeps sha256(token) to support revocation and lookup.

use crate::hmac::{sign, verify as verify_mac};
use crate::scopes::Scope;
use anyhow::{anyhow, Result};
use base64::Engine;
use chrono::Utc;
use rand::RngCore;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Payload {
    tid: String,
    user_id: String,
    project: String,
    scope: String,
    expires_at: i64,
}

#[derive(Debug)]
pub struct VerifyOutcome {
    pub user_id: String,
    pub project: String,
    pub scope: Scope,
}

/// Issue a new token. The returned string is the ONLY copy — DB stores only its sha256.
pub fn issue(
    conn: &Connection,
    user_id: &str,
    project: &str,
    scope: Scope,
    ttl_secs: i64,
    key: &[u8],
) -> Result<String> {
    let now = Utc::now().timestamp();
    let expires_at = now + ttl_secs;
    let payload = new_payload(user_id, project, scope, expires_at);
    let token = encode_token(&payload, key)?;
    persist_token(conn, &token, user_id, project, scope, expires_at, now)?;
    Ok(token)
}

fn new_payload(user_id: &str, project: &str, scope: Scope, expires_at: i64) -> Payload {
    let mut raw = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut raw);
    let tid = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(raw);
    Payload {
        tid,
        user_id: user_id.into(),
        project: project.into(),
        scope: scope.to_string(),
        expires_at,
    }
}

fn encode_token(payload: &Payload, key: &[u8]) -> Result<String> {
    let body = serde_json::to_vec(payload)?;
    let body_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&body);
    let sig = sign(key, body_b64.as_bytes());
    Ok(format!("{}.{}", body_b64, sig))
}

fn persist_token(conn: &Connection, token: &str, user_id: &str, project: &str,
                 scope: Scope, expires_at: i64, now: i64) -> Result<()> {
    let hash = sha256_hex(token.as_bytes());
    conn.execute(
        "INSERT INTO auth_tokens (token_hash, user_id, project, scope, expires_at, created_at)
         VALUES (?1,?2,?3,?4,?5,?6)",
        params![hash, user_id, project, scope.as_str(), expires_at, now],
    )?;
    Ok(())
}

/// Verify a token: signature valid, not revoked, not expired, returns identity + scope.
pub fn verify(conn: &Connection, token: &str, key: &[u8]) -> Result<VerifyOutcome> {
    let (body_b64, sig) = token
        .split_once('.')
        .ok_or_else(|| anyhow!("malformed token"))?;
    verify_mac(key, body_b64.as_bytes(), sig)?;
    let body = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(body_b64)
        .map_err(|e| anyhow!("bad b64 payload: {e}"))?;
    let p: Payload = serde_json::from_slice(&body)?;
    if p.expires_at < Utc::now().timestamp() {
        return Err(anyhow!("token expired"));
    }
    let hash = sha256_hex(token.as_bytes());
    let row: Option<i64> = conn.query_row(
        "SELECT revoked_at FROM auth_tokens WHERE token_hash=?1",
        params![hash], |r| r.get(0)).ok();
    match row {
        None => Err(anyhow!("token unknown to server")),
        Some(rev) if rev > 0 => Err(anyhow!("token revoked")),
        _ => Ok(VerifyOutcome {
            user_id: p.user_id,
            project: p.project,
            scope: Scope::from_str(&p.scope).map_err(|e| anyhow!(e))?,
        }),
    }
}

/// Mark a token as revoked. Returns number of rows affected (0 = unknown).
pub fn revoke(conn: &Connection, token: &str) -> Result<usize> {
    let hash = sha256_hex(token.as_bytes());
    let now = Utc::now().timestamp();
    let n = conn.execute(
        "UPDATE auth_tokens SET revoked_at=?1 WHERE token_hash=?2 AND revoked_at=0",
        params![now, hash],
    )?;
    Ok(n)
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
