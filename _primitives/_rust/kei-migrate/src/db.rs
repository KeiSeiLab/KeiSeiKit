//! Database backend detection + pool construction.
//!
//! Uses `sqlx::Any` so one binary covers Postgres / SQLite / MySQL.
//! Detection is purely on URL scheme — no live probe needed.

use anyhow::{bail, Result};
use sqlx::any::{install_default_drivers, AnyPoolOptions};
use sqlx::AnyPool;

/// Backend inferred from the URL scheme. Determines dialect quirks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    Postgres,
    Sqlite,
    Mysql,
}

impl Backend {
    /// Backend-specific CREATE TABLE for `_kei_migrations`.
    pub fn create_tracker_sql(self) -> &'static str {
        match self {
            Backend::Postgres | Backend::Mysql => {
                "CREATE TABLE IF NOT EXISTS _kei_migrations (
                    version      BIGINT PRIMARY KEY,
                    name         VARCHAR(255) NOT NULL,
                    checksum     CHAR(64)     NOT NULL,
                    applied_at   VARCHAR(32)  NOT NULL
                )"
            }
            Backend::Sqlite => {
                "CREATE TABLE IF NOT EXISTS _kei_migrations (
                    version      INTEGER PRIMARY KEY,
                    name         TEXT NOT NULL,
                    checksum     TEXT NOT NULL,
                    applied_at   TEXT NOT NULL
                )"
            }
        }
    }
}

/// Parse a database URL into a [`Backend`]. Never touches the network.
pub fn detect_backend(url: &str) -> Result<Backend> {
    let lower = url.to_ascii_lowercase();
    if lower.starts_with("postgres://") || lower.starts_with("postgresql://") {
        Ok(Backend::Postgres)
    } else if lower.starts_with("sqlite:") {
        Ok(Backend::Sqlite)
    } else if lower.starts_with("mysql://") || lower.starts_with("mariadb://") {
        Ok(Backend::Mysql)
    } else {
        bail!(
            "unsupported or unrecognised DATABASE_URL scheme: {}. \
             Expected postgres://, sqlite:, or mysql://",
            url
        )
    }
}

/// Build a sqlx `AnyPool` for the given URL (max 4 conns — migration runner is not a server).
pub async fn connect(url: &str) -> Result<AnyPool> {
    install_default_drivers();
    let pool = AnyPoolOptions::new()
        .max_connections(4)
        .connect(url)
        .await?;
    Ok(pool)
}
