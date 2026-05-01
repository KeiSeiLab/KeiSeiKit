//! Single-folder verdict.
//!
//! Verdict thresholds (from PLAN.md Wave 1 verdict):
//!   * `.git/` present → ALREADY-REPO regardless of score
//!   * score ≥ 8       → PROJECT
//!   * score 5..=7     → AMBIGUOUS
//!   * score < 5       → NOT-A-PROJECT

use std::path::Path;

use serde::Serialize;

use crate::scoring::{MarkerKind, MARKERS};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum Verdict {
    Project,
    Ambiguous,
    NotAProject,
    AlreadyRepo,
}

#[derive(Debug, Clone, Serialize)]
pub struct MarkerHit {
    pub file: String,
    pub weight: i32,
    pub kind: MarkerKind,
}

#[derive(Debug, Clone, Serialize)]
pub struct Classification {
    pub path: String,
    pub verdict: Verdict,
    pub score: i32,
    pub primary_lang: String,
    pub markers: Vec<MarkerHit>,
}

/// Apply marker scoring to a flat list of filenames + verdict.
/// Shared by `classify` (local FS) and `classify_remote` (rclone names).
fn verdict_from_names<I: IntoIterator<Item = String>>(
    path_display: String,
    names: I,
) -> Classification {
    let mut hits: Vec<MarkerHit> = Vec::new();
    let mut primary_lang: Option<&'static str> = None;
    let mut already_repo = false;
    let mut score: i32 = 0;

    for name in names {
        let Some(marker) = crate::scoring::marker_for(name.as_str()) else { continue };
        if marker.kind == MarkerKind::AlreadyRepo {
            already_repo = true;
            hits.push(MarkerHit {
                file: marker.file.to_string(),
                weight: marker.weight,
                kind: marker.kind,
            });
            continue;
        }
        score += marker.weight;
        if primary_lang.is_none() {
            if let Some(lang) = marker.primary_lang {
                primary_lang = Some(lang);
            }
        }
        hits.push(MarkerHit {
            file: marker.file.to_string(),
            weight: marker.weight,
            kind: marker.kind,
        });
    }

    let verdict = if already_repo {
        Verdict::AlreadyRepo
    } else if score >= 8 {
        Verdict::Project
    } else if score >= 5 {
        Verdict::Ambiguous
    } else {
        Verdict::NotAProject
    };

    Classification {
        path: path_display,
        verdict,
        score,
        primary_lang: primary_lang.unwrap_or("unknown").to_string(),
        markers: hits,
    }
}

/// Remote classify via `rclone lsf <remote-path> --max-depth 1`. Lists ALL
/// filenames in the folder (no recursion, no download), checks against the
/// marker table. The HEAD file under `.git/` is also detected since rclone
/// returns `.git/` as a name when present in Drive.
pub fn classify_remote(remote_path: &str) -> anyhow::Result<Classification> {
    use std::process::Command;
    let output = Command::new("rclone")
        .args(["lsf", remote_path, "--max-depth", "1"])
        .output()
        .map_err(|e| anyhow::anyhow!("invoke rclone lsf {remote_path}: {e}"))?;
    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "rclone lsf {remote_path} failed ({}): {}",
            output.status,
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    // Strip trailing slash from dir entries (rclone lsf prints "src/", ".git/").
    let names: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|l| l.trim_end_matches('/').to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(verdict_from_names(remote_path.to_string(), names))
}

pub fn classify(path: &Path) -> Classification {
    // Walk the marker table, collect names of files present on disk,
    // then delegate to the shared scoring core. Single SSoT for the
    // verdict ladder + scoring rules (see verdict_from_names above).
    let names: Vec<String> = MARKERS
        .iter()
        .filter(|m| path.join(m.file).exists())
        .map(|m| m.file.to_string())
        .collect();
    verdict_from_names(path.display().to_string(), names)
}
