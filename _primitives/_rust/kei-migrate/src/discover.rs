//! Filesystem migration discovery.
//!
//! Convention: `migrations/<version>_<name>.sql` (up) and optional
//! `migrations/<version>_<name>.down.sql` (down). Version is a monotonic
//! integer, typically a UTC timestamp like `20260421120000`.

use anyhow::{bail, Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

/// One discovered migration (up-side). `down_path` is `Some` iff the sibling file exists.
#[derive(Debug, Clone)]
pub struct Migration {
    pub version: i64,
    pub name: String,
    pub up_path: PathBuf,
    pub down_path: Option<PathBuf>,
    pub up_sql: String,
    pub checksum: String,
}

/// Read every `<version>_<name>.sql` file (ignoring `.down.sql`), sort by version ASC.
pub fn scan(dir: &Path) -> Result<Vec<Migration>> {
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("read_dir {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("sql") {
            continue;
        }
        let fname = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        if fname.ends_with(".down.sql") {
            continue;
        }
        let m = parse_migration(&path)?;
        out.push(m);
    }
    out.sort_by_key(|m| m.version);
    check_unique(&out)?;
    Ok(out)
}

fn parse_migration(path: &Path) -> Result<Migration> {
    let stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .context("non-utf8 filename")?;
    let (ver_str, name) = stem
        .split_once('_')
        .with_context(|| format!("filename not <version>_<name>.sql: {}", stem))?;
    let version: i64 = ver_str
        .parse()
        .with_context(|| format!("version must be integer, got {}", ver_str))?;
    let up_sql = fs::read_to_string(path)
        .with_context(|| format!("read {}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(up_sql.as_bytes());
    let checksum = format!("{:x}", hasher.finalize());
    let down_path = path.with_file_name(format!("{}_{}.down.sql", version, name));
    let down = if down_path.exists() { Some(down_path) } else { None };
    Ok(Migration {
        version,
        name: name.to_string(),
        up_path: path.to_path_buf(),
        down_path: down,
        up_sql,
        checksum,
    })
}

fn check_unique(migs: &[Migration]) -> Result<()> {
    for w in migs.windows(2) {
        if w[0].version == w[1].version {
            bail!(
                "duplicate migration version {} ({} and {})",
                w[0].version,
                w[0].up_path.display(),
                w[1].up_path.display()
            );
        }
    }
    Ok(())
}
