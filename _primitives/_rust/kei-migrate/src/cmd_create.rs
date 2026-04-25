//! `kei-migrate create <name>` — scaffold a new timestamped migration pair.

use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::{Path, PathBuf};

const UP_TEMPLATE: &str = "-- up migration\n-- Write forward-direction SQL below.\n\n";
const DOWN_TEMPLATE: &str =
    "-- down migration\n-- Write reverse SQL below, or add `-- IRREVERSIBLE` to block reversion.\n\n";

/// Create `<dir>/<utc-timestamp>_<sanitized-name>.sql` + `.down.sql`. Returns paths written.
pub fn run(dir: &Path, name: &str) -> Result<(PathBuf, PathBuf)> {
    validate_name(name)?;
    fs::create_dir_all(dir).with_context(|| format!("mkdir -p {}", dir.display()))?;
    let ts = Utc::now().format("%Y%m%d%H%M%S").to_string();
    let sanitized = sanitize(name);
    let up = dir.join(format!("{}_{}.sql", ts, sanitized));
    let down = dir.join(format!("{}_{}.down.sql", ts, sanitized));
    if up.exists() || down.exists() {
        bail!("collision: {} or {} already exists", up.display(), down.display());
    }
    fs::write(&up, UP_TEMPLATE)?;
    fs::write(&down, DOWN_TEMPLATE)?;
    println!("[create] {}", up.display());
    println!("[create] {}", down.display());
    Ok((up, down))
}

fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("migration name must not be empty");
    }
    if name.len() > 80 {
        bail!("migration name too long ({} chars, max 80)", name.len());
    }
    Ok(())
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_lowercase() } else { '_' })
        .collect()
}
