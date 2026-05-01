//! `kei-capability fork <source> --as <new-name>` — copy+rewrite a capability.
//!
//! Reads `_capabilities/<src-cat>/<src-slug>/{capability.toml, text.md}`,
//! validates the new `<cat>::<slug>` name, creates the target directory,
//! writes a rewritten `capability.toml` (new name + `[lineage]` block),
//! and copies `text.md` byte-identical.
//!
//! Constructor Pattern: one cube = fork copy+rewrite. No subcommand dispatch.

use anyhow::{anyhow, bail, Context, Result};
use kei_agent_runtime::role::validate_name;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use toml::{map::Map, Value};

/// Summary returned to the CLI / tests after a successful fork.
#[derive(Debug)]
pub struct ForkSummary {
    pub source: String,
    pub target: String,
    pub diff_count: usize,
    pub target_dir: PathBuf,
}

/// Run the fork operation against a kit root.
///
/// `now_iso` is an injectable clock (ISO-8601 UTC string). Pass
/// `current_iso_utc` for production; tests pass a fixed value.
pub fn run_fork(
    source: &str,
    new_name: &str,
    kit_root: &Path,
    now_iso: &str,
) -> Result<ForkSummary> {
    let (src_dir, target_dir) = resolve_paths(source, new_name, kit_root)?;
    ensure_source_exists(&src_dir)?;
    ensure_target_free(&target_dir)?;
    let src_toml_raw = std::fs::read_to_string(src_dir.join("capability.toml"))
        .with_context(|| format!("read {}", src_dir.join("capability.toml").display()))?;
    let creator = std::env::var("KEI_CREATOR_ID").unwrap_or_else(|_| "unknown".into());
    let (rewritten, diff_count) =
        rewrite_toml(&src_toml_raw, source, new_name, &creator, now_iso)?;
    write_fork(&src_dir, &target_dir, &rewritten)?;
    Ok(ForkSummary {
        source: source.to_string(),
        target: new_name.to_string(),
        diff_count,
        target_dir,
    })
}

fn resolve_paths(source: &str, new_name: &str, kit_root: &Path) -> Result<(PathBuf, PathBuf)> {
    let (src_cat, src_slug) = split_cap_name(source)?;
    let (new_cat, new_slug) = split_cap_name(new_name)?;
    let caps_root = kit_root.join("_capabilities");
    Ok((
        caps_root.join(src_cat).join(src_slug),
        caps_root.join(new_cat).join(new_slug),
    ))
}

fn ensure_source_exists(src_dir: &Path) -> Result<()> {
    if !src_dir.is_dir() {
        bail!("source capability dir not found: {}", src_dir.display());
    }
    Ok(())
}

fn ensure_target_free(target_dir: &Path) -> Result<()> {
    if target_dir.exists() {
        bail!(
            "target capability dir already exists — refusing to clobber: {}",
            target_dir.display()
        );
    }
    Ok(())
}

fn write_fork(src_dir: &Path, target_dir: &Path, toml_body: &str) -> Result<()> {
    std::fs::create_dir_all(target_dir)
        .with_context(|| format!("mkdir {}", target_dir.display()))?;
    std::fs::write(target_dir.join("capability.toml"), toml_body)
        .with_context(|| format!("write {}", target_dir.join("capability.toml").display()))?;
    std::fs::copy(src_dir.join("text.md"), target_dir.join("text.md"))
        .with_context(|| format!("copy text.md from {}", src_dir.display()))?;
    Ok(())
}

/// Split `<cat>::<slug>` and validate both halves through the shared regex.
fn split_cap_name(name: &str) -> Result<(&str, &str)> {
    let (cat, slug) = name
        .split_once("::")
        .filter(|(c, s)| !c.is_empty() && !s.is_empty())
        .ok_or_else(|| anyhow!("malformed capability name '{name}' — expected <cat>::<slug>"))?;
    validate_name("capability-category", cat)?;
    validate_name("capability-slug", slug)?;
    Ok((cat, slug))
}

/// Parse source capability.toml, rewrite `[capability].name`, insert a
/// `[lineage]` table with `fork_from` / `parents` / `creator` / `created`.
/// Returns the serialized string and the number of field writes performed.
fn rewrite_toml(
    src_raw: &str,
    source: &str,
    new_name: &str,
    creator: &str,
    now_iso: &str,
) -> Result<(String, usize)> {
    let mut root: Value = toml::from_str(src_raw).context("parse source capability.toml")?;
    let tbl = root
        .as_table_mut()
        .ok_or_else(|| anyhow!("source capability.toml root is not a table"))?;
    let mut writes = 0usize;
    rewrite_capability_name(tbl, new_name)?;
    writes += 1;
    insert_lineage(tbl, source, creator, now_iso);
    writes += 3; // fork_from, parents, creator+created counted as 3 additions
    let out = toml::to_string_pretty(&root).context("serialize rewritten capability.toml")?;
    Ok((out, writes))
}

fn rewrite_capability_name(root: &mut Map<String, Value>, new_name: &str) -> Result<()> {
    let cap_tbl = root
        .get_mut("capability")
        .and_then(|v| v.as_table_mut())
        .ok_or_else(|| anyhow!("source capability.toml missing [capability] table"))?;
    cap_tbl.insert("name".into(), Value::String(new_name.into()));
    let (cat, _) = new_name.split_once("::").unwrap_or((new_name, ""));
    cap_tbl.insert("category".into(), Value::String(cat.into()));
    Ok(())
}

fn insert_lineage(root: &mut Map<String, Value>, source: &str, creator: &str, now_iso: &str) {
    let mut lineage: Map<String, Value> = Map::new();
    lineage.insert("fork_from".into(), Value::String(source.into()));
    lineage.insert(
        "parents".into(),
        Value::Array(vec![Value::String(source.into())]),
    );
    lineage.insert("creator".into(), Value::String(creator.into()));
    lineage.insert("created".into(), Value::String(now_iso.into()));
    root.insert("lineage".into(), Value::Table(lineage));
}

/// Current UTC time as `YYYY-MM-DDTHH:MM:SSZ`. No chrono dep — minimal
/// proleptic-Gregorian converter over Unix epoch seconds.
pub fn current_iso_utc() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    epoch_to_iso(secs as i64)
}

fn epoch_to_iso(secs: i64) -> String {
    let (days, sod) = (secs.div_euclid(86_400), secs.rem_euclid(86_400));
    let (h, rem) = (sod / 3600, sod % 3600);
    let (m, s) = (rem / 60, rem % 60);
    let (y, mo, d) = days_to_ymd(days);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

fn days_to_ymd(days_since_epoch: i64) -> (i64, u32, u32) {
    // Howard Hinnant's algorithm — days_since_epoch → civil date (y, m, d).
    let z = days_since_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_to_iso_spot_check() {
        // Unix epoch itself.
        assert_eq!(epoch_to_iso(0), "1970-01-01T00:00:00Z");
        // 2000-01-01T00:00:00Z = 946684800.
        assert_eq!(epoch_to_iso(946_684_800), "2000-01-01T00:00:00Z");
        // 2026-04-23T00:00:00Z.
        assert_eq!(epoch_to_iso(1_777_334_400 - 5 * 86_400), "2026-04-23T00:00:00Z");
    }

    #[test]
    fn split_cap_name_rejects_unqualified() {
        assert!(split_cap_name("no-colons").is_err());
        assert!(split_cap_name("a::b").is_ok());
        assert!(split_cap_name("BAD::slug").is_err());
        assert!(split_cap_name("cat::").is_err());
    }
}
