//! `GET /api/v1/cortex/fs/list?path=<rel-or-abs>` — directory listing.
//!
//! Path is treated as relative-to-`project_root` when not absolute. Absolute
//! paths must already be inside `project_root` (canonicalised). Symlinks are
//! NOT followed; hidden noise (`node_modules`, `.git`, `target`, etc.) is
//! filtered out. Sorted dirs-first, alpha within each group.

use crate::error::AppError;
use crate::state::AppState;
use axum::extract::{Query, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

/// Cap on entries to keep the response bounded.
const MAX_ENTRIES: usize = 500;

/// Directories whose contents we never enumerate (noise filter).
const HIDE_DIRS: &[&str] = &[
    "node_modules", ".git", "target", "dist", "_archive",
    ".svelte-kit", ".cache", "_forks", ".turbo", ".next",
];

/// Optional query — `?path=<rel>` defaults to `""` (project root).
#[derive(Debug, Default, Deserialize)]
pub struct ListQuery {
    #[serde(default)]
    pub path: Option<String>,
}

/// One entry in the listing response.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct FsEntry {
    pub name: String,
    pub kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<u64>,
}

/// Response body the UI parses.
#[derive(Debug, Serialize)]
pub struct ListResponse {
    pub entries: Vec<FsEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Handler entry point.
pub async fn list(
    State(state): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<ListResponse>, AppError> {
    let root = state.config().project_root.clone();
    let rel = q.path.unwrap_or_default();
    let target = resolve_target(&root, &rel)?;
    let entries = read_dir_entries(&target)?;
    Ok(Json(build_response(entries)))
}

/// Resolve `rel` to an absolute path inside `project_root`. Rejects parent
/// references and absolute paths that escape the root.
fn resolve_target(root: &Path, rel: &str) -> Result<PathBuf, AppError> {
    if rel.contains("..") {
        return Err(AppError::BadRequest("path may not contain '..'".into()));
    }
    let candidate = if rel.is_empty() {
        root.to_path_buf()
    } else if Path::new(rel).is_absolute() {
        PathBuf::from(rel)
    } else {
        root.join(rel)
    };
    let canon = candidate
        .canonicalize()
        .map_err(|e| AppError::NotFound(format!("path not found: {e}")))?;
    let root_canon = root
        .canonicalize()
        .map_err(|_| AppError::Internal("project_root canonicalize".into()))?;
    if !canon.starts_with(&root_canon) {
        return Err(AppError::BadRequest("path escapes project_root".into()));
    }
    if !canon.is_dir() {
        return Err(AppError::BadRequest("path is not a directory".into()));
    }
    Ok(canon)
}

/// Read a directory and produce `FsEntry`s, filtering noise + hiding
/// dotfiles + capping at `MAX_ENTRIES`.
fn read_dir_entries(dir: &Path) -> Result<Vec<FsEntry>, AppError> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if should_hide(&name) {
            continue;
        }
        if let Some(e) = entry_to_fs_entry(&entry, &name) {
            out.push(e);
        }
        if out.len() >= MAX_ENTRIES {
            break;
        }
    }
    Ok(out)
}

/// Convert one `DirEntry` to an `FsEntry`. Skips symlinks (not followed).
fn entry_to_fs_entry(entry: &std::fs::DirEntry, name: &str) -> Option<FsEntry> {
    let meta = entry.metadata().ok()?;
    if meta.file_type().is_symlink() {
        return None;
    }
    let kind = if meta.is_dir() { "dir" } else { "file" };
    let size = if kind == "file" { Some(meta.len()) } else { None };
    let mtime = meta
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_secs());
    Some(FsEntry {
        name: name.to_string(),
        kind,
        size,
        mtime,
    })
}

/// True when this name should be omitted from the listing (noise filter).
///
/// MISS-3: matches the entry name against `HIDE_DIRS` either exactly OR as
/// a prefix followed by a non-alphanumeric separator (`-`, `_`, `.`). This
/// hides `node_modules`, `node_modules.bak`, and `node_modules-archive`,
/// while still showing legitimate names like `nodejs` or `target_audience`
/// that merely SHARE A PREFIX. Plain `name.starts_with(d)` is too greedy;
/// `name == d` is too narrow. The separator-anchor strikes the balance.
fn should_hide(name: &str) -> bool {
    if name.starts_with('.') {
        return true;
    }
    HIDE_DIRS.iter().any(|d| name_matches_hide_token(name, d))
}

/// Match `name` against a HIDE_DIRS token. Exact match wins; otherwise the
/// name must start with `<token><sep>` where `<sep>` is one of `-_.`.
/// Anything else (`nodejs` vs `node`, `targets` vs `target`) is allowed
/// through.
fn name_matches_hide_token(name: &str, token: &str) -> bool {
    if name == token {
        return true;
    }
    if !name.starts_with(token) {
        return false;
    }
    match name.as_bytes().get(token.len()) {
        Some(b'-') | Some(b'_') | Some(b'.') => true,
        _ => false,
    }
}

/// Sort dirs-first / alpha-within-group; cap-note when truncated.
fn build_response(mut entries: Vec<FsEntry>) -> ListResponse {
    let truncated = entries.len() >= MAX_ENTRIES;
    entries.sort_by(|a, b| match (a.kind, b.kind) {
        ("dir", "file") => std::cmp::Ordering::Less,
        ("file", "dir") => std::cmp::Ordering::Greater,
        _ => a.name.cmp(&b.name),
    });
    let note = if truncated {
        Some(format!("listing capped at {MAX_ENTRIES} entries"))
    } else {
        None
    };
    ListResponse { entries, note }
}

#[cfg(test)]
#[path = "fs_list_test.rs"]
mod tests;
