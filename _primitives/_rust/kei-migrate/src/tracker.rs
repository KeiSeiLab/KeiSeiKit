//! `_kei_migrations` tracking table operations.
//!
//! Row shape: (version, name, checksum, applied_at). Checksum guards against
//! silent edits of an applied file — mismatch = hard fail, requires human ack.

use crate::db::Backend;
use anyhow::{bail, Result};
use sqlx::{AnyPool, Row};

/// Create tracker table if missing. Idempotent.
pub async fn ensure_table(pool: &AnyPool, backend: Backend) -> Result<()> {
    sqlx::query(backend.create_tracker_sql()).execute(pool).await?;
    Ok(())
}

/// Versions of all applied migrations, ASC.
pub async fn applied_versions(pool: &AnyPool) -> Result<Vec<i64>> {
    let rows = sqlx::query("SELECT version FROM _kei_migrations ORDER BY version ASC")
        .fetch_all(pool)
        .await?;
    let mut out = Vec::with_capacity(rows.len());
    for r in rows {
        out.push(r.try_get::<i64, _>(0)?);
    }
    Ok(out)
}

/// Checksum of a specific applied version, or `None` if not applied.
pub async fn applied_checksum(pool: &AnyPool, version: i64) -> Result<Option<String>> {
    let row = sqlx::query("SELECT checksum FROM _kei_migrations WHERE version = $1")
        .bind(version)
        .fetch_optional(pool)
        .await?;
    Ok(row.map(|r| r.try_get::<String, _>(0)).transpose()?)
}

/// Insert a tracker row after a successful up-migration.
pub async fn record_up(
    pool: &AnyPool,
    version: i64,
    name: &str,
    checksum: &str,
    applied_at: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO _kei_migrations (version, name, checksum, applied_at) VALUES ($1, $2, $3, $4)",
    )
    .bind(version)
    .bind(name)
    .bind(checksum)
    .bind(applied_at)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a tracker row after a successful down-migration.
pub async fn record_down(pool: &AnyPool, version: i64) -> Result<()> {
    sqlx::query("DELETE FROM _kei_migrations WHERE version = $1")
        .bind(version)
        .execute(pool)
        .await?;
    Ok(())
}

/// Abort if any applied migration's recorded checksum doesn't match the on-disk file.
pub async fn verify_checksums<'a, I>(pool: &AnyPool, on_disk: I) -> Result<()>
where
    I: IntoIterator<Item = (i64, &'a str, &'a str)>, // (version, name, checksum)
{
    for (version, name, disk_sum) in on_disk {
        if let Some(db_sum) = applied_checksum(pool, version).await? {
            if db_sum != disk_sum {
                bail!(
                    "checksum drift on applied migration {} ({}): db={}, disk={}. \
                     Refusing to proceed — someone edited an already-applied file.",
                    version,
                    name,
                    db_sum,
                    disk_sum
                );
            }
        }
    }
    Ok(())
}
