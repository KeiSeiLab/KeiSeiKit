//! `kei-migrate down [n]` — revert the last N applied migrations.
//!
//! Requires a sibling `<version>_<name>.down.sql` for each target. Missing
//! down-file = hard error — we don't guess reversals.

use crate::discover::Migration;
use crate::tracker;
use anyhow::{bail, Context, Result};
use sqlx::AnyPool;
use std::collections::HashMap;

/// Revert the last `n` applied migrations in reverse order.
pub async fn run(pool: &AnyPool, migrations: &[Migration], n: u32) -> Result<u32> {
    let mut applied: Vec<i64> = tracker::applied_versions(pool).await?;
    applied.sort_unstable();
    applied.reverse(); // newest first
    let by_version: HashMap<i64, &Migration> =
        migrations.iter().map(|m| (m.version, m)).collect();
    let mut reverted = 0u32;
    for v in applied.into_iter().take(n as usize) {
        let m = by_version.get(&v).with_context(|| {
            format!("applied version {} has no matching file on disk", v)
        })?;
        revert_one(pool, m).await?;
        reverted += 1;
        println!("[down] {} {} — reverted", m.version, m.name);
    }
    Ok(reverted)
}

async fn revert_one(pool: &AnyPool, m: &Migration) -> Result<()> {
    let down_path = m.down_path.as_ref().ok_or_else(|| {
        anyhow::anyhow!(
            "no down-sql for migration {} ({}) — create {}_{}.down.sql",
            m.version,
            m.name,
            m.version,
            m.name
        )
    })?;
    let sql = std::fs::read_to_string(down_path)
        .with_context(|| format!("read {}", down_path.display()))?;
    if sql.contains("-- IRREVERSIBLE") {
        bail!(
            "migration {} ({}) is marked IRREVERSIBLE — refusing to run down-sql",
            m.version,
            m.name
        );
    }
    let mut tx = pool.begin().await?;
    sqlx::raw_sql(&sql)
        .execute(&mut *tx)
        .await
        .with_context(|| format!("revert migration {} ({})", m.version, m.name))?;
    tx.commit().await?;
    tracker::record_down(pool, m.version).await?;
    Ok(())
}
