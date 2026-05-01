// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//
//! Redis-backed PingStore. TTL keys (auto-expire 90s).
//! Schema: kei-ping:agent:<agent_id> → JSON Heartbeat (EX 90).

use crate::model::{now_epoch, Heartbeat, PingFilter};
use crate::store::{BackendKind, PingStore};
use anyhow::Result;
use redis::{aio::MultiplexedConnection, AsyncCommands, Client};
use tokio::sync::Mutex;

const KEY_PREFIX: &str = "kei-ping:agent:";

pub struct RedisPingStore {
    conn: Mutex<MultiplexedConnection>,
}

impl RedisPingStore {
    pub async fn connect(url: String) -> Result<Self> {
        let client = Client::open(url)?;
        let conn = client.get_multiplexed_async_connection().await?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn key(agent_id: &str) -> String {
        format!("{KEY_PREFIX}{agent_id}")
    }
}

#[async_trait::async_trait]
impl PingStore for RedisPingStore {
    fn kind(&self) -> BackendKind {
        BackendKind::Redis
    }

    async fn send(&self, h: &Heartbeat) -> Result<()> {
        let payload = serde_json::to_string(h)?;
        let mut c = self.conn.lock().await;
        let _: () = c.set_ex(Self::key(&h.agent_id), payload, 120).await?;
        Ok(())
    }

    async fn list(&self, f: &PingFilter) -> Result<Vec<Heartbeat>> {
        let pattern = format!("{KEY_PREFIX}*");
        let mut out = Vec::new();
        let now = now_epoch();
        // SCAN — cooperative iteration, doesn't block other writers.
        let mut c = self.conn.lock().await;
        let mut iter = c.scan_match::<_, String>(pattern).await?;
        let mut keys: Vec<String> = Vec::new();
        while let Some(k) = iter.next_item().await {
            keys.push(k);
        }
        drop(iter);
        for k in keys {
            let raw: Option<String> = c.get(&k).await?;
            if let Some(s) = raw {
                if let Ok(h) = serde_json::from_str::<Heartbeat>(&s) {
                    if f.alive(&h, now) {
                        out.push(h);
                    }
                }
            }
        }
        out.sort_by(|a, b| b.last_seen_epoch.cmp(&a.last_seen_epoch));
        Ok(out)
    }

    async fn clear(&self, agent_id: &str) -> Result<()> {
        let mut c = self.conn.lock().await;
        let _: i64 = c.del(Self::key(agent_id)).await?;
        Ok(())
    }
}
