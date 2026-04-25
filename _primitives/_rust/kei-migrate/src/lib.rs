//! kei-migrate — universal SQL migration runner.
//!
//! Single binary, three backends (Postgres / SQLite / MySQL) autodetected
//! from `DATABASE_URL`. Sequential `.sql` files in `migrations/`, tracked in
//! `_kei_migrations` with SHA-256 checksums.
//!
//! Library surface exists so integration tests can drive the primitive
//! without `process::Command` gymnastics.

pub mod cli;
pub mod cmd_create;
pub mod cmd_down;
pub mod cmd_status;
pub mod cmd_up;
pub mod db;
pub mod discover;
pub mod tracker;

use anyhow::Result;
use std::path::Path;

/// End-to-end `up` entry: connect, ensure tracker, scan dir, apply pending.
/// Returns number of migrations applied.
pub async fn do_up(database_url: &str, dir: &Path) -> Result<u32> {
    let backend = db::detect_backend(database_url)?;
    let pool = db::connect(database_url).await?;
    tracker::ensure_table(&pool, backend).await?;
    let migs = discover::scan(dir)?;
    let n = cmd_up::run(&pool, &migs).await?;
    pool.close().await;
    Ok(n)
}

/// End-to-end `down` entry: revert last N applied.
pub async fn do_down(database_url: &str, dir: &Path, n: u32) -> Result<u32> {
    let backend = db::detect_backend(database_url)?;
    let pool = db::connect(database_url).await?;
    tracker::ensure_table(&pool, backend).await?;
    let migs = discover::scan(dir)?;
    let reverted = cmd_down::run(&pool, &migs, n).await?;
    pool.close().await;
    Ok(reverted)
}

/// End-to-end `status` entry: returns (applied, pending) counts.
pub async fn do_status(database_url: &str, dir: &Path) -> Result<(u32, u32)> {
    let backend = db::detect_backend(database_url)?;
    let pool = db::connect(database_url).await?;
    tracker::ensure_table(&pool, backend).await?;
    let migs = discover::scan(dir)?;
    let r = cmd_status::run(&pool, &migs).await?;
    pool.close().await;
    Ok(r)
}
