//! `kei-migrate up` — apply all pending migrations in version-ASC order.

use crate::discover::Migration;
use crate::tracker;
use anyhow::{Context, Result};
use chrono::Utc;
use sqlx::AnyPool;
use std::collections::HashSet;

/// Apply every migration whose version is not in the applied set.
/// Each migration runs in its own transaction; failure aborts and leaves
/// prior applied migrations committed.
pub async fn run(pool: &AnyPool, migrations: &[Migration]) -> Result<u32> {
    let applied: HashSet<i64> = tracker::applied_versions(pool).await?.into_iter().collect();
    let on_disk: Vec<(i64, &str, &str)> = migrations
        .iter()
        .map(|m| (m.version, m.name.as_str(), m.checksum.as_str()))
        .collect();
    tracker::verify_checksums(pool, on_disk).await?;
    let mut count = 0u32;
    for m in migrations {
        if applied.contains(&m.version) {
            continue;
        }
        apply_one(pool, m).await?;
        count += 1;
        println!("[up] {} {} — applied", m.version, m.name);
    }
    Ok(count)
}

async fn apply_one(pool: &AnyPool, m: &Migration) -> Result<()> {
    let mut tx = pool.begin().await?;
    sqlx::raw_sql(&m.up_sql)
        .execute(&mut *tx)
        .await
        .with_context(|| format!("apply migration {} ({})", m.version, m.name))?;
    tx.commit().await?;
    let now = Utc::now().to_rfc3339();
    tracker::record_up(pool, m.version, &m.name, &m.checksum, &now).await?;
    Ok(())
}
