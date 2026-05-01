// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Thin wrapper over `redis::Client` plus the deterministic key-schema
//! used by [`crate::backend::RedisBackend`]. Holds no trait impls so the
//! schema helpers can be unit-tested without a live Redis.
//!
//! Schema (deterministic; documented in spec/MEMORY-BACKENDS.md §Redis):
//!
//! ```text
//! <prefix>:item:<kind>:<created_at_ms>:<key>   → JSON-encoded MemoryItem
//! <prefix>:tag:<tag>                            → SET of item-ids
//! ```
//!
//! `item-id` is the encoded item key string above (the full path); this
//! lets a tag-driven query resolve straight to the JSON GET without an
//! extra index hop.

use crate::error::{Error, Result};

/// Redis client + scope prefix. Connections are short-lived: every call
/// to [`RedisStore::conn`] hands out a fresh `MultiplexedConnection`.
pub struct RedisStore {
    client: redis::Client,
    prefix: String,
}

impl RedisStore {
    /// Connect by URL (`redis://host:port`, `rediss://...`, etc).
    /// Prefix scopes every key emitted by this store; pick one per
    /// tenant / per environment.
    pub fn from_url(url: &str, prefix: impl Into<String>) -> Result<Self> {
        let client = redis::Client::open(url).map_err(Error::from)?;
        let prefix = prefix.into();
        if prefix.is_empty() {
            return Err(Error::Config("prefix must be non-empty".into()));
        }
        Ok(Self { client, prefix })
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    /// Hand out a fresh multiplexed async connection per call. The
    /// `redis` crate's `MultiplexedConnection` is cheap to clone and
    /// safe across tokio tasks; we deliberately do not pool here — that
    /// is a deployment concern surfaced by the operator.
    pub async fn conn(&self) -> Result<redis::aio::MultiplexedConnection> {
        let c = self.client.get_multiplexed_async_connection().await?;
        Ok(c)
    }

    pub fn item_key(&self, kind: &str, created_at_ms: i64, key: &str) -> String {
        encode_item_key(&self.prefix, kind, created_at_ms, key)
    }

    pub fn tag_key(&self, tag: &str) -> String {
        encode_tag_key(&self.prefix, tag)
    }

    /// SCAN match-pattern filtered by optional kind. `*` is used as a
    /// glob for unconstrained components.
    pub fn item_match(&self, kind: Option<&str>) -> String {
        let k = kind.unwrap_or("*");
        format!("{}:item:{}:*", self.prefix, k)
    }
}

/// Compose `<prefix>:item:<kind>:<ts>:<key>`.
pub fn encode_item_key(prefix: &str, kind: &str, ts_ms: i64, key: &str) -> String {
    format!("{prefix}:item:{kind}:{ts_ms}:{key}")
}

/// Compose `<prefix>:tag:<tag>`.
pub fn encode_tag_key(prefix: &str, tag: &str) -> String {
    format!("{prefix}:tag:{tag}")
}

/// Parsed view of an `item` key. None on malformed input.
#[derive(Debug, PartialEq, Eq)]
pub struct ParsedItemKey<'a> {
    pub prefix: &'a str,
    pub kind: &'a str,
    pub ts_ms: i64,
    pub key: &'a str,
}

/// Inverse of [`encode_item_key`]. Returns `None` if the input does not
/// match `<prefix>:item:<kind>:<ts>:<key>` with a parseable timestamp.
pub fn decode_item_key(s: &str) -> Option<ParsedItemKey<'_>> {
    // splitn(5, ':') — prefix, "item", kind, ts, key (key may itself
    // contain ':' so the trailing field is left unsplit).
    let mut it = s.splitn(5, ':');
    let prefix = it.next()?;
    let tag = it.next()?;
    if tag != "item" {
        return None;
    }
    let kind = it.next()?;
    let ts_str = it.next()?;
    let key = it.next()?;
    let ts_ms: i64 = ts_str.parse().ok()?;
    Some(ParsedItemKey { prefix, kind, ts_ms, key })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn item_key_roundtrip() {
        let k = encode_item_key("kei", "trace", 1714000000000, "session-42");
        assert_eq!(k, "kei:item:trace:1714000000000:session-42");
        let p = decode_item_key(&k).expect("parse");
        assert_eq!(p.prefix, "kei");
        assert_eq!(p.kind, "trace");
        assert_eq!(p.ts_ms, 1714000000000);
        assert_eq!(p.key, "session-42");
    }

    #[test]
    fn item_key_preserves_colons_in_user_key() {
        // User-supplied `key` may contain ':' (e.g. URL-style ids); the
        // 5-way split must keep it intact.
        let k = encode_item_key("kei", "concept", 100, "proj:foo:bar");
        let p = decode_item_key(&k).expect("parse");
        assert_eq!(p.key, "proj:foo:bar");
        assert_eq!(p.ts_ms, 100);
    }

    #[test]
    fn decode_rejects_malformed() {
        assert!(decode_item_key("kei:item:trace").is_none());
        assert!(decode_item_key("kei:NOTitem:trace:1:k").is_none());
        assert!(decode_item_key("kei:item:trace:NOT_AN_INT:k").is_none());
    }

    #[test]
    fn tag_key_format() {
        assert_eq!(encode_tag_key("kei", "sleep"), "kei:tag:sleep");
    }

    #[test]
    fn item_match_wildcards() {
        let s = RedisStore::from_url("redis://127.0.0.1:65535", "kei").unwrap();
        assert_eq!(s.item_match(None), "kei:item:*:*");
        assert_eq!(s.item_match(Some("trace")), "kei:item:trace:*");
    }

    #[test]
    fn empty_prefix_rejected() {
        let r = RedisStore::from_url("redis://127.0.0.1:6379", "");
        assert!(r.is_err());
    }
}
