// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 <author org>
//!
//! Connection wrapper. `tokio_postgres::connect` returns `(Client,
//! Connection)`; the Connection future must be polled by an executor
//! task, otherwise the Client deadlocks. We spawn it on the current
//! tokio runtime as part of [`PgStore::connect`].

use crate::error::Result;
use crate::schema::apply_schema;
use tokio_postgres::{Client, NoTls};

/// Owns the live `tokio_postgres::Client`. Cheap to wrap in `Arc` and
/// share across many [`crate::PostgresBackend`] instances.
pub struct PgStore {
    client: Client,
}

impl PgStore {
    /// Connect to PostgreSQL using a libpq-style connection string and
    /// spawn the driver task on the current tokio runtime.
    ///
    /// Errors propagate from `tokio_postgres::connect`. Connection-task
    /// errors are logged to `stderr` (the Client surfaces them on the
    /// next operation as well).
    pub async fn connect(conn_string: &str) -> Result<Self> {
        let (client, connection) =
            tokio_postgres::connect(conn_string, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("kei-memory-postgres: connection error: {e}");
            }
        });
        Ok(Self { client })
    }

    /// Bootstrap the schema. Idempotent.
    pub async fn init(&self) -> Result<()> {
        apply_schema(&self.client).await
    }

    /// Borrow the underlying client. Used by `PostgresBackend`; not
    /// exposed for direct SQL by external callers (use the trait
    /// surface instead).
    pub(crate) fn client(&self) -> &Client {
        &self.client
    }
}

/// Lightweight validation: a libpq URI must start with `postgres://` or
/// `postgresql://`, otherwise the driver rejects it. We don't fully
/// parse — just sniff for the obvious mistake before a network call.
pub fn looks_like_pg_url(s: &str) -> bool {
    s.starts_with("postgres://")
        || s.starts_with("postgresql://")
        || s.contains("host=")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_sniff_accepts_uri_form() {
        assert!(looks_like_pg_url("postgres://u:p@h/db"));
        assert!(looks_like_pg_url("postgresql://u@h/db"));
    }

    #[test]
    fn url_sniff_accepts_kv_form() {
        assert!(looks_like_pg_url("host=localhost user=kei dbname=kei"));
    }

    #[test]
    fn url_sniff_rejects_obvious_garbage() {
        assert!(!looks_like_pg_url("sqlite:///tmp/x.db"));
        assert!(!looks_like_pg_url(""));
    }
}
