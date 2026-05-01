// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! `PingStore` trait + auto-selector (Constructor Pattern dispatcher).

use crate::model::{Heartbeat, PingFilter};
use crate::redis_store::RedisPingStore;
use crate::sqlite_store::SqlitePingStore;
use anyhow::Result;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    Sqlite,
    Redis,
}

impl BackendKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            BackendKind::Sqlite => "sqlite",
            BackendKind::Redis => "redis",
        }
    }
}

#[async_trait::async_trait]
pub trait PingStore: Send + Sync {
    fn kind(&self) -> BackendKind;
    async fn send(&self, h: &Heartbeat) -> Result<()>;
    async fn list(&self, f: &PingFilter) -> Result<Vec<Heartbeat>>;
    async fn clear(&self, agent_id: &str) -> Result<()>;
}

/// Auto-detect — try Redis first (1s timeout), fallback to SQLite.
/// Cached choice persists in `~/.claude/config/ping-backend` so we
/// don't ping Redis on every CLI call.
pub async fn auto_select() -> Result<Box<dyn PingStore>> {
    let cache = config_path();
    if let Ok(s) = std::fs::read_to_string(&cache) {
        match s.trim() {
            "redis" => {
                if let Ok(rs) = RedisPingStore::connect(default_redis_url()).await {
                    return Ok(Box::new(rs));
                }
            }
            "sqlite" => {
                let sq = SqlitePingStore::open(default_sqlite_path())?;
                return Ok(Box::new(sq));
            }
            _ => {}
        }
    }

    let kind = if redis_alive() {
        BackendKind::Redis
    } else {
        BackendKind::Sqlite
    };
    let _ = std::fs::create_dir_all(cache.parent().unwrap());
    let _ = std::fs::write(&cache, kind.as_str());

    match kind {
        BackendKind::Redis => Ok(Box::new(RedisPingStore::connect(default_redis_url()).await?)),
        BackendKind::Sqlite => Ok(Box::new(SqlitePingStore::open(default_sqlite_path())?)),
    }
}

fn redis_alive() -> bool {
    // 1s budget: spawn redis-cli ping with a child-kill-after-timeout.
    let mut child = match Command::new("redis-cli")
        .arg("-t")
        .arg("1")
        .arg("ping")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return false,
    };
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    return false;
                }
                let mut buf = String::new();
                use std::io::Read;
                if let Some(mut out) = child.stdout.take() {
                    let _ = out.read_to_string(&mut buf);
                }
                return buf.trim() == "PONG";
            }
            Ok(None) => {
                if std::time::Instant::now() > deadline {
                    let _ = child.kill();
                    return false;
                }
                std::thread::sleep(Duration::from_millis(50));
            }
            Err(_) => return false,
        }
    }
}

fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(home).join(".claude/config/ping-backend")
}

fn default_redis_url() -> String {
    std::env::var("KEI_PING_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".into())
}

fn default_sqlite_path() -> PathBuf {
    if let Ok(p) = std::env::var("KEI_PING_SQLITE_PATH") {
        return PathBuf::from(p);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(home).join(".claude/agents/ping.sqlite")
}
