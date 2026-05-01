//! `POST /api/v1/cortex/tool/apply` — apply edit/write proposed by the
//! agentic loop. UI captures `tool_use_start{name:"edit"}`, shows DiffPane,
//! and POSTs here on Apply. TRUSTED op — bearer-auth only; see INTEGRATION.md.
//! Wave 44b F-CRIT-4: atomic write moved to `tool_apply_atomic.rs` (`O_NOFOLLOW`
//! openat + post-rename canonical re-check).

use crate::error::AppError;
use crate::state::AppState;
use super::tool_apply_atomic::atomic_write_nofollow;
use crate::tool::edit::count_occurrences;
use crate::tool::read::validate_path as validate_abs_path;
use crate::tool::write::deny_system_dirs;
use axum::extract::State;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Hard cap on file bytes — matches `tool::read::MAX_BYTES`.
const MAX_BYTES: u64 = 10 * 1024 * 1024;
const DEFAULT_TOOL: &str = "edit";

/// Request body. `old_text`/`new_text` are aliases for `old_string`/`new_string`
/// to match the current cortex-ui wire shape.
#[derive(Debug, Deserialize)]
pub struct ApplyRequest {
    #[serde(default)] pub tool: Option<String>,
    pub path: String,
    #[serde(default)] pub old_string: Option<String>,
    #[serde(default)] pub new_string: Option<String>,
    #[serde(default)] pub old_text: Option<String>,
    #[serde(default)] pub new_text: Option<String>,
    #[serde(default)] pub content: Option<String>,
    #[serde(default)] pub replace_all: bool,
    #[serde(default)] pub force: bool,
}

/// Response body — UI consumes `applied` and `diff_summary.lines_changed`.
#[derive(Debug, Serialize)]
pub struct ApplyResponse {
    pub applied: bool,
    pub tool: String,
    pub path: String,
    pub diff_summary: DiffSummary,
}

#[derive(Debug, Serialize)]
pub struct DiffSummary {
    pub lines_changed: usize,
}

/// Resolved sandbox view: the candidate path the caller asked for plus the
/// canonical project root. Both are needed downstream for the post-write
/// re-canonicalisation check.
struct Resolved {
    path: PathBuf,
    root_canon: PathBuf,
}

/// Handler entry point — wired in `routes.rs` under the bearer group.
pub async fn apply(
    State(state): State<AppState>,
    Json(req): Json<ApplyRequest>,
) -> Result<Json<ApplyResponse>, AppError> {
    let tool = req.tool.clone().unwrap_or_else(|| DEFAULT_TOOL.to_string());
    let resolved = resolve_under_root(&state, &req.path)?;
    match tool.as_str() {
        "edit" => apply_edit(&resolved, &req).await,
        "write" => apply_write(&resolved, &req).await,
        other => Err(AppError::BadRequest(format!("unknown tool: {other}"))),
    }
}

/// Run sandbox helpers + confirm path canonicalises inside `project_root`.
/// For not-yet-existing paths we walk up to the deepest existing ancestor
/// and canonicalise THAT — handles macOS `/var → /private/var` symlinks for
/// new write targets without false-403'ing on legitimate child creation.
fn resolve_under_root(state: &AppState, path: &str) -> Result<Resolved, AppError> {
    validate_abs_path(path).map_err(|e| AppError::BadRequest(format!("path: {e}")))?;
    deny_system_dirs(path).map_err(|_| AppError::Forbidden)?;
    let root_canon = state.config().project_root.canonicalize()
        .map_err(|_| AppError::Internal("project_root canonicalize".into()))?;
    let candidate = PathBuf::from(path);
    let anchor = deepest_existing_canonical(&candidate);
    if !anchor.starts_with(&root_canon) {
        return Err(AppError::Forbidden);
    }
    Ok(Resolved { path: candidate, root_canon })
}

/// Walk `p` upward until an ancestor exists; return its canonical form.
/// Falls back to `p` itself if even `/` is missing (impossible on a sane fs).
fn deepest_existing_canonical(p: &Path) -> PathBuf {
    let mut cur = p;
    loop {
        if let Ok(canon) = cur.canonicalize() {
            return canon;
        }
        match cur.parent() {
            Some(parent) if parent != cur => cur = parent,
            _ => return p.to_path_buf(),
        }
    }
}

/// Apply an `edit` — read file, enforce uniqueness, atomic-write back.
async fn apply_edit(r: &Resolved, req: &ApplyRequest) -> Result<Json<ApplyResponse>, AppError> {
    let path = r.path.as_path();
    let old_s = req.old_string.clone().or_else(|| req.old_text.clone())
        .ok_or_else(|| AppError::BadRequest("missing old_string/old_text".into()))?;
    let new_s = req.new_string.clone().or_else(|| req.new_text.clone())
        .ok_or_else(|| AppError::BadRequest("missing new_string/new_text".into()))?;
    if old_s.is_empty() {
        return Err(AppError::BadRequest("old_string is empty".into()));
    }
    let meta = tokio::fs::metadata(path).await
        .map_err(|_| AppError::NotFound(format!("path not found: {}", path.display())))?;
    if meta.len() > MAX_BYTES {
        return Err(AppError::PayloadTooLarge(format!("{} bytes > {MAX_BYTES}", meta.len())));
    }
    let original = tokio::fs::read_to_string(path).await
        .map_err(|e| AppError::BadRequest(format!("not utf-8 or read fail: {e}")))?;
    let count = count_occurrences(&original, &old_s);
    let replaced = perform_replacement(&original, &old_s, &new_s, count, req.replace_all)?;
    atomic_write_nofollow(path, replaced.as_bytes(), &r.root_canon).await?;
    let lines_changed = lines_changed(&original, &replaced);
    Ok(Json(ApplyResponse {
        applied: true,
        tool: "edit".into(),
        path: path.display().to_string(),
        diff_summary: DiffSummary { lines_changed },
    }))
}

/// Apply a `write` — refuses overwrite without `force=true`.
async fn apply_write(r: &Resolved, req: &ApplyRequest) -> Result<Json<ApplyResponse>, AppError> {
    let path = r.path.as_path();
    let content = req.content.clone()
        .ok_or_else(|| AppError::BadRequest("missing content".into()))?;
    if content.len() as u64 > MAX_BYTES {
        return Err(AppError::PayloadTooLarge(format!("{} bytes > {MAX_BYTES}", content.len())));
    }
    if path.exists() && !req.force {
        return Err(AppError::Conflict(
            "path exists; use edit for in-place change or pass force=true".into()
        ));
    }
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }
    atomic_write_nofollow(path, content.as_bytes(), &r.root_canon).await?;
    let lines_changed = content.lines().count();
    Ok(Json(ApplyResponse {
        applied: true,
        tool: "write".into(),
        path: path.display().to_string(),
        diff_summary: DiffSummary { lines_changed },
    }))
}

/// Enforce uniqueness/match rules and produce the post-replacement string.
fn perform_replacement(
    original: &str, old_s: &str, new_s: &str, count: usize, replace_all: bool,
) -> Result<String, AppError> {
    if count == 0 {
        return Err(AppError::Conflict("old_string not found in file".into()));
    }
    if !replace_all && count > 1 {
        return Err(AppError::Conflict(format!(
            "old_string matched {count} times; pass replace_all=true or add more context"
        )));
    }
    if replace_all {
        Ok(original.replace(old_s, new_s))
    } else {
        Ok(original.replacen(old_s, new_s, 1))
    }
}

/// Count of lines that differ between `before` and `after` (length-tolerant).
fn lines_changed(before: &str, after: &str) -> usize {
    let b: Vec<&str> = before.lines().collect();
    let a: Vec<&str> = after.lines().collect();
    (0..b.len().max(a.len()))
        .filter(|i| b.get(*i).copied().unwrap_or("") != a.get(*i).copied().unwrap_or(""))
        .count()
}

#[cfg(test)]
#[path = "tool_apply_test.rs"]
mod tests;

#[cfg(test)]
#[path = "tool_apply_write_test.rs"]
mod write_tests;

#[cfg(test)]
#[path = "tool_apply_symlink_test.rs"]
mod symlink_tests;
