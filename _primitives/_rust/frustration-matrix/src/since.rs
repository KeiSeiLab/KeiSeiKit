//! `--since` parser + mtime filter.
//!
//! Accepts `30d`, `7d`, `1d`, `all`, or any `<N>d` (positive integer days).
//! Returns a `SystemTime` cut-off; files strictly older than the cut-off
//! are excluded from the scan.
//!
//! Rationale: we use filesystem mtime rather than in-document timestamps
//! because chatlogs have heterogeneous timestamp formats (ISO, human,
//! none). mtime is reliable, cheap, and matches user intent of "files I
//! edited in the last 30 days".

use anyhow::{anyhow, Context, Result};
use std::path::Path;
use std::time::{Duration, SystemTime};

/// Cut-off time from a `--since` string. `None` means "scan everything".
pub fn parse(s: &str) -> Result<Option<SystemTime>> {
    let s = s.trim().to_ascii_lowercase();
    if s == "all" {
        return Ok(None);
    }
    let Some(num) = s.strip_suffix('d') else {
        return Err(anyhow!("--since: expected '<N>d' or 'all', got {s:?}"));
    };
    let days: u64 = num
        .parse()
        .with_context(|| format!("--since: day count {num:?}"))?;
    let cutoff = SystemTime::now()
        .checked_sub(Duration::from_secs(days * 86_400))
        .ok_or_else(|| anyhow!("--since: {days}d underflows SystemTime"))?;
    Ok(Some(cutoff))
}

/// True iff `path`'s mtime is at or after `cutoff`. Missing mtime → true
/// (i.e. include by default; never silently drop a file on FS quirks).
pub fn passes(path: &Path, cutoff: Option<SystemTime>) -> bool {
    let Some(cut) = cutoff else {
        return true;
    };
    let Ok(meta) = std::fs::metadata(path) else {
        return true;
    };
    let Ok(mtime) = meta.modified() else {
        return true;
    };
    mtime >= cut
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_day_spec() {
        let t = parse("30d").unwrap().unwrap();
        let now = SystemTime::now();
        let delta = now.duration_since(t).unwrap().as_secs();
        assert!(delta >= 29 * 86_400 && delta <= 31 * 86_400);
    }

    #[test]
    fn parses_all() {
        assert!(parse("all").unwrap().is_none());
    }

    #[test]
    fn rejects_garbage() {
        assert!(parse("30").is_err());
        assert!(parse("yesterday").is_err());
    }
}
