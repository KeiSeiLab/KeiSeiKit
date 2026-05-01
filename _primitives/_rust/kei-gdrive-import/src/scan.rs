//! Walk-tree scanner.
//!
//! Two backends:
//!   * local FS (`std::fs::read_dir`, no `walkdir` dep)
//!   * remote rclone (shell out to `rclone lsjson <remote> --dirs-only`)
//!
//! Depth: one level under root. The wizard recurses by re-invoking
//! `scan-tree` on subfolders the user marks AMBIGUOUS — keeps the
//! primitive flat and predictable.

use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{anyhow, Context, Result};
use serde::Deserialize;

use crate::classify::{classify, Classification};

pub fn scan_tree(root: &Path) -> Result<Vec<Classification>> {
    let mut out: Vec<Classification> = Vec::new();
    let entries = std::fs::read_dir(root)
        .with_context(|| format!("read_dir {}", root.display()))?;
    for entry in entries {
        let entry = entry?;
        let p = entry.path();
        if !p.is_dir() {
            continue;
        }
        out.push(classify(&p));
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}

#[derive(Debug, Deserialize)]
struct RcloneEntry {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "IsDir")]
    is_dir: bool,
}

pub fn scan_remote(remote_root: &str) -> Result<Vec<Classification>> {
    let output = Command::new("rclone")
        .args(["lsjson", remote_root, "--dirs-only"])
        .output()
        .with_context(|| format!("invoke rclone lsjson {remote_root}"))?;
    if !output.status.success() {
        return Err(anyhow!(
            "rclone lsjson failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let entries: Vec<RcloneEntry> =
        serde_json::from_slice(&output.stdout).context("parse rclone lsjson output")?;

    let mut out: Vec<Classification> = Vec::new();
    for e in entries {
        if !e.is_dir {
            continue;
        }
        // For remote folders we can't classify without download — emit
        // a stub Classification keyed on the remote path. The wizard
        // is responsible for `rclone copy`-ing the candidate to a
        // staging dir and then re-running `classify` locally.
        let pseudo = PathBuf::from(format!("{}/{}", remote_root.trim_end_matches('/'), e.name));
        out.push(Classification {
            path: pseudo.display().to_string(),
            verdict: crate::classify::Verdict::Ambiguous,
            score: 0,
            primary_lang: "unknown".to_string(),
            markers: Vec::new(),
        });
    }
    out.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(out)
}
