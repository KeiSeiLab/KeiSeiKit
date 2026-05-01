// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! [`SqliteBackend`] — `MemoryBackend` impl over [`SqliteStore`].
//!
//! All SQL ops are synchronous (rusqlite) and wrapped in
//! `tokio::task::spawn_blocking` so the async runtime is never stalled.
//! The store itself is shared via `Arc`; cloning a backend is cheap.

use crate::error::Error;
use crate::schema::{decode_tags, encode_tags};
use crate::store::SqliteStore;
use async_trait::async_trait;
use kei_runtime_core::dna::{Dna, HasDna};
use kei_runtime_core::error::Result as CoreResult;
use kei_runtime_core::traits::memory::{MemoryBackend, MemoryItem, MemoryQuery};
use rusqlite::params_from_iter;
use std::sync::Arc;

/// SQLite-backed [`MemoryBackend`]. Holds its own DNA + an `Arc<SqliteStore>`.
pub struct SqliteBackend {
    dna: Dna,
    parent: Option<Dna>,
    store: Arc<SqliteStore>,
}

impl SqliteBackend {
    /// Construct from an already-built store + DNA. Parent DNA optional.
    pub fn new(dna: Dna, parent: Option<Dna>, store: Arc<SqliteStore>) -> Self {
        Self { dna, parent, store }
    }

    /// Borrow the underlying store (e.g. for sibling backends to share it).
    pub fn inner_store(&self) -> &Arc<SqliteStore> {
        &self.store
    }
}

impl HasDna for SqliteBackend {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait]
impl MemoryBackend for SqliteBackend {
    fn backend_name(&self) -> &'static str {
        "sqlite"
    }

    async fn store(&self, item: &MemoryItem) -> CoreResult<()> {
        let store = self.store.clone();
        let item = item.clone();
        let inner = tokio::task::spawn_blocking(move || -> crate::Result<()> {
            let tags_csv = encode_tags(&item.tags);
            let parent_str = item.parent_dna.as_ref().map(|d| d.as_str().to_string());
            let conn = store.lock();
            conn.execute(
                "INSERT OR REPLACE INTO memory_items
                 (dna, parent_dna, kind, key, value, tags_csv, created_at_ms)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![
                    item.dna.as_str(),
                    parent_str,
                    item.kind,
                    item.key,
                    item.value,
                    tags_csv,
                    item.created_at_ms,
                ],
            )?;
            Ok(())
        })
        .await
        .map_err(|e| Error::Join(e.to_string()))?;
        inner.map_err(Into::into)
    }

    async fn query(&self, q: &MemoryQuery) -> CoreResult<Vec<MemoryItem>> {
        let store = self.store.clone();
        let q = q.clone();
        let inner = tokio::task::spawn_blocking(move || -> crate::Result<Vec<MemoryItem>> {
            let conn = store.lock();
            let (sql, params) = build_query_sql(&q);
            let mut stmt = conn.prepare(&sql)?;
            let mapped = stmt.query_map(params_from_iter(params.iter()), row_to_item)?;
            let mut out = Vec::new();
            for r in mapped {
                out.push(r?);
            }
            Ok(out)
        })
        .await
        .map_err(|e| Error::Join(e.to_string()))?;
        inner.map_err(Into::into)
    }

    async fn compact(&self, since_ms: i64) -> CoreResult<usize> {
        let store = self.store.clone();
        let inner = tokio::task::spawn_blocking(move || -> crate::Result<usize> {
            let conn = store.lock();
            let n = conn.execute(
                "DELETE FROM memory_items WHERE created_at_ms < ?1",
                rusqlite::params![since_ms],
            )?;
            Ok(n)
        })
        .await
        .map_err(|e| Error::Join(e.to_string()))?;
        inner.map_err(Into::into)
    }

    async fn mirror_to_remote(&self, _dest_url: &str) -> CoreResult<()> {
        Err(Error::Provider(
            "sqlite backend does not implement remote mirroring; use kei-sleep-sync.sh".into(),
        )
        .into())
    }
}

/// Compose dynamic SELECT with parameter list. Order DESC by created_at_ms.
fn build_query_sql(q: &MemoryQuery) -> (String, Vec<rusqlite::types::Value>) {
    use rusqlite::types::Value;
    let mut sql = String::from(
        "SELECT dna, parent_dna, kind, key, value, tags_csv, created_at_ms
         FROM memory_items WHERE 1=1",
    );
    let mut params: Vec<Value> = Vec::new();
    append_filters(&mut sql, &mut params, q);
    sql.push_str(" ORDER BY created_at_ms DESC");
    if let Some(lim) = q.limit {
        sql.push_str(" LIMIT ?");
        params.push(Value::Integer(lim as i64));
    }
    (sql, params)
}

/// Append WHERE-clause filters in stable order. Splits to keep
/// `build_query_sql` under the Constructor-Pattern 30-LOC ceiling.
fn append_filters(sql: &mut String, params: &mut Vec<rusqlite::types::Value>, q: &MemoryQuery) {
    use rusqlite::types::Value;
    if let Some(kind) = &q.kind {
        sql.push_str(" AND kind = ?");
        params.push(Value::Text(kind.clone()));
    }
    if let Some(prefix) = &q.key_prefix {
        sql.push_str(" AND key LIKE ? ESCAPE '\\'");
        let escaped = prefix.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_");
        params.push(Value::Text(format!("{escaped}%")));
    }
    if let Some(since) = q.since_ms {
        sql.push_str(" AND created_at_ms >= ?");
        params.push(Value::Integer(since));
    }
    for tag in &q.tag_any {
        sql.push_str(" AND tags_csv LIKE ?");
        params.push(Value::Text(format!("%,{tag},%")));
    }
}

/// Map one row → `MemoryItem`.
fn row_to_item(row: &rusqlite::Row<'_>) -> rusqlite::Result<MemoryItem> {
    let dna_s: String = row.get(0)?;
    let parent_s: Option<String> = row.get(1)?;
    let kind: String = row.get(2)?;
    let key: String = row.get(3)?;
    let value: String = row.get(4)?;
    let tags_csv: String = row.get(5)?;
    let created_at_ms: i64 = row.get(6)?;
    let dna = Dna::parse(dna_s).map_err(|e| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e))
    })?;
    let parent_dna = match parent_s {
        Some(s) => Some(Dna::parse(s).map_err(|e| {
            rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e))
        })?),
        None => None,
    };
    Ok(MemoryItem {
        dna,
        parent_dna,
        kind,
        key,
        value,
        tags: decode_tags(&tags_csv),
        created_at_ms,
    })
}
