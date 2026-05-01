//! `kei-migrate status` — list applied + pending migrations.

use crate::discover::Migration;
use crate::tracker;
use anyhow::Result;
use sqlx::AnyPool;
use std::collections::HashSet;

/// Print a human-readable table. Returns (applied_count, pending_count).
pub async fn run(pool: &AnyPool, migrations: &[Migration]) -> Result<(u32, u32)> {
    let applied: HashSet<i64> = tracker::applied_versions(pool).await?.into_iter().collect();
    let mut a = 0u32;
    let mut p = 0u32;
    println!("{:>14} {:<8} name", "version", "status");
    println!("{:>14} {:<8} ----", "-------", "------");
    for m in migrations {
        let status = if applied.contains(&m.version) {
            a += 1;
            "APPLIED"
        } else {
            p += 1;
            "PENDING"
        };
        println!("{:>14} {:<8} {}", m.version, status, m.name);
    }
    println!();
    println!("{} applied, {} pending", a, p);
    Ok((a, p))
}
