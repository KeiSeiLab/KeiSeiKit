// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Sled-backed storage layer. The async surface lives in `backend.rs`;
//! this module is sync (sled is sync) and exposes blocking helpers
//! that the backend wraps in `tokio::task::spawn_blocking`.
//!
//! Key encoding (lex-sortable for `scan_prefix` + chronological order):
//!
//! ```text
//! <kind>\x00<ts_be_8>\x00<key>
//! ```
//!
//! - `kind` bytes form the prefix (1 NUL terminator → no kind-name bleed).
//! - `ts_be_8` is `i64::to_be_bytes` so newer items sort *after* older.
//! - `key` is appended last so identical-timestamp items can coexist.
//!
//! Values are JSON-serialized `MemoryItem`.

use crate::error::{Error, Result};
use kei_runtime_core::traits::memory::MemoryItem;
use std::path::Path;

const SEP: u8 = 0x00;

/// Owned handle around a `sled::Db`. Cheap to clone (sled::Db is Arc).
#[derive(Clone)]
pub struct SledStore {
    db: sled::Db,
}

impl SledStore {
    /// Open or create a sled DB at `path`. Directory is created if absent.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let db = sled::open(path.as_ref())?;
        Ok(Self { db })
    }

    /// Underlying `sled::Db` for advanced ops (flush, size, etc).
    pub fn raw(&self) -> &sled::Db {
        &self.db
    }

    /// Insert (or overwrite) one `MemoryItem`.
    pub fn put_item(&self, item: &MemoryItem) -> Result<()> {
        let key = encode_key(&item.kind, item.created_at_ms, &item.key);
        let val = serde_json::to_vec(item)?;
        self.db.insert(key, val)?;
        Ok(())
    }

    /// Scan items, optionally restricted to `kind`. Returns DESC by ts.
    pub fn scan(&self, kind: Option<&str>) -> Result<Vec<MemoryItem>> {
        let mut out = Vec::new();
        let iter: Box<dyn Iterator<Item = _>> = match kind {
            Some(k) => {
                let mut prefix = k.as_bytes().to_vec();
                prefix.push(SEP);
                Box::new(self.db.scan_prefix(prefix))
            }
            None => Box::new(self.db.iter()),
        };
        for kv in iter {
            let (_k, v) = kv?;
            let item: MemoryItem = serde_json::from_slice(&v)?;
            out.push(item);
        }
        // sled iter is ascending by key; ts is in the key (BE) so this is
        // chronological ASC. Reverse for DESC by created_at_ms.
        out.sort_by(|a, b| b.created_at_ms.cmp(&a.created_at_ms));
        Ok(out)
    }

    /// Count items in `kind` strictly older than `since_ms`.
    /// v0.1 is a no-op delete: it surfaces the count for callers that
    /// want to drive their own retention policy.
    pub fn count_older_than(&self, kind: Option<&str>, since_ms: i64) -> Result<usize> {
        let items = self.scan(kind)?;
        Ok(items.iter().filter(|it| it.created_at_ms < since_ms).count())
    }

    /// Force a flush to disk. Mostly for tests.
    pub fn flush(&self) -> Result<()> {
        self.db.flush().map_err(Error::from)?;
        Ok(())
    }
}

/// Encode `<kind>\x00<ts_be>\x00<key>` for prefix-scan + ordering.
pub fn encode_key(kind: &str, ts_ms: i64, key: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(kind.len() + 1 + 8 + 1 + key.len());
    out.extend_from_slice(kind.as_bytes());
    out.push(SEP);
    out.extend_from_slice(&ts_ms.to_be_bytes());
    out.push(SEP);
    out.extend_from_slice(key.as_bytes());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_orders_by_ts_within_kind() {
        let a = encode_key("trace", 100, "x");
        let b = encode_key("trace", 200, "x");
        assert!(a < b, "older ts must sort before newer ts");
    }

    #[test]
    fn key_separates_kinds() {
        let a = encode_key("trace", 100, "x");
        let b = encode_key("traced", 100, "x");
        // 'trace\x00...' vs 'traced\x00...' → the NUL after 'trace' makes
        // them unambiguously distinct prefixes.
        assert_ne!(a, b);
    }
}
