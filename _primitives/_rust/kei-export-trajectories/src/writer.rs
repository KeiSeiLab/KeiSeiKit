//! Atomic JSONL writer.
//!
//! Constructor Pattern: one cube = path → tempfile → fsync → rename. The
//! caller hands us an iterator of `Trajectory`; we serialize one per line
//! using `serde_json::to_string` (no pretty-printing — JSONL is
//! line-delimited and trainers expect compact output).
//!
//! Atomicity matters because exports often run during a Phase B / Phase D
//! sleep cycle: a partially written `trajectory_samples.jsonl` would
//! poison the next training run. We write to `<path>.tmp` in the same
//! directory (so rename is atomic on POSIX) and only swap on full success.

use crate::sharegpt::Trajectory;
use anyhow::{Context, Result};
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Serialize `trajectories` to JSONL at `path`, writing through a sibling
/// `.tmp` file and renaming on success. Each line is a complete JSON
/// object terminated by exactly one `\n`.
pub fn write_jsonl(path: &Path, trajectories: &[Trajectory]) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("mkdir {}", parent.display()))?;
    }
    let tmp = tmp_path(path);
    write_all_to(&tmp, trajectories)?;
    std::fs::rename(&tmp, path)
        .with_context(|| format!("rename {} -> {}", tmp.display(), path.display()))?;
    Ok(())
}

/// Compute the sibling `.tmp` path. Putting it in the same directory keeps
/// the rename POSIX-atomic — across-FS renames silently degrade to
/// copy+unlink, which we explicitly do NOT want for partial-write safety.
fn tmp_path(target: &Path) -> std::path::PathBuf {
    let mut s = target.as_os_str().to_owned();
    s.push(".tmp");
    s.into()
}

/// Serialize every trajectory to the temp file, flush, drop. Each line is
/// one `serde_json::to_string` call — line-flushing implicit via BufWriter
/// drop at end of scope.
fn write_all_to(tmp: &Path, trajectories: &[Trajectory]) -> Result<()> {
    let f = create_truncated(tmp)?;
    let mut w = BufWriter::new(f);
    for t in trajectories {
        let line = serde_json::to_string(t).context("serialize trajectory")?;
        w.write_all(line.as_bytes())
            .with_context(|| format!("write line to {}", tmp.display()))?;
        w.write_all(b"\n")
            .with_context(|| format!("write newline to {}", tmp.display()))?;
    }
    w.flush().with_context(|| format!("flush {}", tmp.display()))?;
    Ok(())
}

/// Open the temp file for write, truncating any previous failed run's
/// leftovers. We deliberately do NOT use `tempfile::NamedTempFile` — it
/// places files in `$TMPDIR` which often crosses filesystems on macOS,
/// breaking the rename-atomicity guarantee.
fn create_truncated(path: &Path) -> Result<File> {
    OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
        .with_context(|| format!("open {} for write", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sharegpt::{From, ShareGptMessage, ToolStats};
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    fn sample() -> Trajectory {
        Trajectory {
            prompt_index: 0,
            conversations: vec![ShareGptMessage {
                from: From::Human,
                value: "hello".into(),
            }],
            completed: true,
            tool_stats: BTreeMap::from([(
                "Read".to_string(),
                ToolStats { count: 1, success: 1, failure: 0 },
            )]),
            tool_error_counts: BTreeMap::from([("Read".to_string(), 0)]),
            metadata: serde_json::Map::new(),
        }
    }

    #[test]
    fn jsonl_roundtrip_one_line_per_record() {
        let d = tempdir().unwrap();
        let p = d.path().join("out.jsonl");
        let batch = vec![sample(), sample()];
        write_jsonl(&p, &batch).unwrap();
        let txt = std::fs::read_to_string(&p).unwrap();
        let lines: Vec<&str> = txt.lines().collect();
        assert_eq!(lines.len(), 2);
        for line in lines {
            let _: Trajectory = serde_json::from_str(line).expect("valid jsonl line");
        }
    }
}
