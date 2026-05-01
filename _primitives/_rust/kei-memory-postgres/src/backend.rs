// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! `MemoryBackend` impl over `PgStore`. One backend = one DNA. Many
//! backends can share the same `Arc<PgStore>`.

use crate::error::{Error as PgErr, Result as PgResult};
use crate::query_builder::build_select;
use crate::store::PgStore;
use kei_runtime_core::traits::memory::{MemoryBackend, MemoryItem, MemoryQuery};
use kei_runtime_core::{Dna, DnaBuilder, HasDna};
use std::sync::Arc;
use tokio_postgres::Row;

pub struct PostgresBackend {
    dna: Dna,
    parent: Option<Dna>,
    store: Arc<PgStore>,
}

impl PostgresBackend {
    /// Build with a fresh DNA. `body` defaults to `b"pg-v16"` to
    /// fingerprint the schema generation; bump it when [`crate::SCHEMA_SQL`]
    /// changes.
    pub fn new(store: Arc<PgStore>, parent: Option<Dna>) -> PgResult<Self> {
        let dna = DnaBuilder::new("primitive")
            .caps(["PR", "AP", "PG"])
            .scope("keiseikit.dev/primitives/kei-memory-postgres")
            .body(b"pg-v16")
            .build()
            .map_err(|e| PgErr::Provider(format!("dna: {e}")))?;
        Ok(Self { dna, parent, store })
    }
}

impl HasDna for PostgresBackend {
    fn dna(&self) -> &Dna {
        &self.dna
    }
    fn parent_dna(&self) -> Option<&Dna> {
        self.parent.as_ref()
    }
}

#[async_trait::async_trait]
impl MemoryBackend for PostgresBackend {
    fn backend_name(&self) -> &'static str {
        "postgres"
    }

    async fn store(&self, item: &MemoryItem) -> kei_runtime_core::Result<()> {
        // value is JSON-encoded text on the trait surface; deserialize
        // once so PostgreSQL stores it as JSONB (not as a quoted string).
        let value_json: serde_json::Value =
            serde_json::from_str(&item.value).map_err(PgErr::Serde)?;
        let parent_str: Option<String> =
            item.parent_dna.as_ref().map(|d| d.as_str().to_string());

        let sql = "INSERT INTO memory_items \
            (dna, parent_dna, kind, key, value, tags, created_at_ms) \
            VALUES ($1, $2, $3, $4, $5, $6, $7) \
            ON CONFLICT (dna) DO UPDATE SET \
                parent_dna = EXCLUDED.parent_dna, \
                kind       = EXCLUDED.kind, \
                key        = EXCLUDED.key, \
                value      = EXCLUDED.value, \
                tags       = EXCLUDED.tags, \
                created_at_ms = EXCLUDED.created_at_ms";

        self.store
            .client()
            .execute(
                sql,
                &[
                    &item.dna.as_str(),
                    &parent_str,
                    &item.kind,
                    &item.key,
                    &value_json,
                    &item.tags,
                    &item.created_at_ms,
                ],
            )
            .await
            .map_err(PgErr::Postgres)?;
        Ok(())
    }

    async fn query(
        &self,
        q: &MemoryQuery,
    ) -> kei_runtime_core::Result<Vec<MemoryItem>> {
        let built = build_select(q);
        // tokio-postgres expects &[&(dyn ToSql + Sync)]; project the
        // owning Vec<Box<...>> through that view.
        let params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = built
            .params
            .iter()
            .map(|b| b.as_ref() as &(dyn tokio_postgres::types::ToSql + Sync))
            .collect();
        let rows = self
            .store
            .client()
            .query(&built.sql, &params)
            .await
            .map_err(PgErr::Postgres)?;
        let mut items = Vec::with_capacity(rows.len());
        for row in rows {
            items.push(row_to_item(&row)?);
        }
        Ok(items)
    }

    async fn compact(&self, since_ms: i64) -> kei_runtime_core::Result<usize> {
        let sql = "DELETE FROM memory_items \
            WHERE created_at_ms < $1 RETURNING dna";
        let rows = self
            .store
            .client()
            .query(sql, &[&since_ms])
            .await
            .map_err(PgErr::Postgres)?;
        Ok(rows.len())
    }

    async fn mirror_to_remote(
        &self,
        _dest_url: &str,
    ) -> kei_runtime_core::Result<()> {
        Err(PgErr::Provider(
            "mirror_to_remote: postgres backend has no native mirror; \
             use kei-sleep-sync.sh for git-push semantics (RULE 0.15)"
                .into(),
        )
        .into())
    }
}

fn row_to_item(row: &Row) -> PgResult<MemoryItem> {
    let dna_s: String = row.try_get("dna").map_err(PgErr::Postgres)?;
    let parent_s: Option<String> =
        row.try_get("parent_dna").map_err(PgErr::Postgres)?;
    let kind: String = row.try_get("kind").map_err(PgErr::Postgres)?;
    let key: String = row.try_get("key").map_err(PgErr::Postgres)?;
    let value_json: serde_json::Value =
        row.try_get("value").map_err(PgErr::Postgres)?;
    let tags: Vec<String> = row.try_get("tags").map_err(PgErr::Postgres)?;
    let created_at_ms: i64 =
        row.try_get("created_at_ms").map_err(PgErr::Postgres)?;

    let dna = Dna::parse(dna_s)
        .map_err(|e| PgErr::Provider(format!("dna parse: {e}")))?;
    let parent_dna = match parent_s {
        Some(s) => Some(
            Dna::parse(s)
                .map_err(|e| PgErr::Provider(format!("parent dna: {e}")))?,
        ),
        None => None,
    };
    let value = serde_json::to_string(&value_json).map_err(PgErr::Serde)?;
    Ok(MemoryItem {
        dna,
        parent_dna,
        kind,
        key,
        value,
        tags,
        created_at_ms,
    })
}
