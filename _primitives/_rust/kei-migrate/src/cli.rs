//! CLI surface — clap argument parsing for `kei-migrate`.

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    name = "kei-migrate",
    about = "Universal SQL migration runner (Postgres / SQLite / MySQL)",
    version
)]
pub struct Cli {
    /// Database URL. Overrides $DATABASE_URL.
    /// Formats:
    ///   postgres://user:pass@host:port/db
    ///   sqlite:///absolute/path.db  or  sqlite::memory:
    ///   mysql://user:pass@host:port/db
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,

    /// Migrations directory (default: ./migrations)
    #[arg(long, default_value = "migrations")]
    pub dir: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Apply all pending migrations.
    Up,
    /// Revert the last N migrations (requires <ts>_<name>.down.sql).
    Down {
        #[arg(default_value_t = 1)]
        n: u32,
    },
    /// List applied vs pending migrations.
    Status,
    /// Create a new timestamped migration scaffold: <ts>_<name>.sql (+ .down.sql).
    Create {
        /// Short migration name, e.g. "add_users_email_index".
        name: String,
    },
}
