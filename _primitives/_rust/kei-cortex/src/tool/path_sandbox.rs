//! Path-sandbox primitives — chroot enforcement + basename deny-lists.
//!
//! Composition: pure functions, no I/O beyond `canonicalize`. Used by
//! `read.rs`, `write.rs`, `edit.rs` to (a) keep all access inside the
//! configured `project_root`, and (b) reject sensitive basenames even
//! when they happen to live inside the project tree (a symlinked
//! `.env`, a co-located `id_rsa`, etc.).
//!
//! Defense-in-depth layers:
//!   1. `enforce_project_root(path, root)` — canonicalised prefix check
//!   2. `is_blocked_basename(path)` — file-type matches (env / keys / pem)
//!   3. `is_blocked_home_rc(path)` — shell rc / dotfile ownership at $HOME
//!
//! Constructor Pattern: zero state, ≤200 LOC, lives next to the tools
//! that import it (sibling cube, no dyn dispatch).

use super::types::ToolError;
use std::path::{Path, PathBuf};

/// Canonicalised prefix-check: requested path must resolve INSIDE
/// `project_root`. Both sides are canonicalised before comparison so
/// symlinked escapes (`/tmp/proj/link → /etc`) are caught.
pub fn enforce_project_root(path: &str, project_root: &Path) -> Result<PathBuf, ToolError> {
    let req = PathBuf::from(path);
    let req_canon = canon_or_lexical(&req);
    let root_canon = project_root.canonicalize().map_err(|e| {
        ToolError::Internal(format!("project_root canonicalize: {e}"))
    })?;
    if !req_canon.starts_with(&root_canon) {
        return Err(ToolError::OutsideRoot(format!(
            "{} not inside {}",
            req_canon.display(),
            root_canon.display()
        )));
    }
    Ok(req_canon)
}

/// `canonicalize()` fails when the file does not exist yet (write tool).
/// Fall back to lexical resolution of the parent dir, then append the
/// final basename. Still catches `..` because the parent must exist and
/// be canonicalisable.
fn canon_or_lexical(req: &Path) -> PathBuf {
    if let Ok(c) = req.canonicalize() {
        return c;
    }
    let parent = req.parent().unwrap_or_else(|| Path::new("/"));
    let basename = req.file_name();
    if let Ok(p_canon) = parent.canonicalize() {
        match basename {
            Some(b) => p_canon.join(b),
            None => p_canon,
        }
    } else {
        req.to_path_buf()
    }
}

/// Reject sensitive basenames regardless of containing dir. Catches
/// `.env`, private keys, credential files, sqlite-stored secrets, etc.
pub fn is_blocked_basename(path: &str) -> bool {
    let p = Path::new(path);
    let basename = match p.file_name().and_then(|n| n.to_str()) {
        Some(b) => b.to_ascii_lowercase(),
        None => return false,
    };
    // Exact-match blocks. `.gitconfig` is intentionally NOT here —
    // project-local gitconfig stubs may be legitimate; the $HOME root
    // copy is blocked by `is_blocked_home_rc` instead.
    const EXACT: &[&str] = &[
        ".env", ".envrc", ".netrc",
        "credentials", "credentials.json",
    ];
    if EXACT.iter().any(|s| basename == *s) {
        return true;
    }
    // Suffix / prefix patterns
    let suffix_blocked = basename.ends_with(".pem")
        || basename.ends_with(".key")
        || basename.ends_with(".gpg")
        || basename.ends_with(".env");
    let prefix_blocked = basename.starts_with("id_rsa")
        || basename.starts_with("id_ed25519")
        || basename.starts_with("id_ecdsa");
    if suffix_blocked || prefix_blocked {
        return true;
    }
    // Substring containing "secret" inside basename catches *secret*.json
    if basename.contains("secret") && !basename.contains("secretly") {
        return true;
    }
    false
}

/// Block writes / reads of dotfile shell-rc and ssh / aws credential
/// files at $HOME root, even via project-root symlink. We compare
/// against a fixed list of full path suffixes: `~/.zshrc`, etc.
pub fn is_blocked_home_rc(path: &str) -> bool {
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return false,
    };
    let p = Path::new(path);
    // Resolve symlinks and `..` lexically; canonicalize when present.
    let canon = p.canonicalize().unwrap_or_else(|_| p.to_path_buf());
    let canon_str = canon.to_string_lossy().to_string();
    for rel in HOME_BLOCKED_RELATIVE {
        let full = format!("{home}/{rel}");
        if canon_str == full {
            return true;
        }
    }
    false
}

/// Relative paths under `$HOME` blocked even by basename match. Does
/// NOT include `.env` because `is_blocked_basename` covers that.
const HOME_BLOCKED_RELATIVE: &[&str] = &[
    ".zshrc", ".bashrc", ".bash_profile", ".profile", ".tmux.conf",
    ".vimrc", ".gitconfig", ".fish/config.fish", ".config/fish/config.fish",
    ".ssh/config", ".ssh/authorized_keys", ".ssh/id_rsa", ".ssh/id_ed25519",
    ".aws/credentials", ".aws/config", ".netrc",
];

/// Compose the three checks; returns the first denial it finds.
pub fn check_all(path: &str, project_root: &Path) -> Result<PathBuf, ToolError> {
    if is_blocked_basename(path) {
        return Err(ToolError::PathDenied(format!(
            "blocked basename: {path}"
        )));
    }
    if is_blocked_home_rc(path) {
        return Err(ToolError::PathDenied(format!(
            "blocked dotfile at $HOME: {path}"
        )));
    }
    enforce_project_root(path, project_root)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn basename_blocks_dotenv() {
        assert!(is_blocked_basename("/anywhere/.env"));
        assert!(is_blocked_basename("/x/y/z.env"));
    }

    #[test]
    fn basename_blocks_id_rsa_variants() {
        assert!(is_blocked_basename("/a/id_rsa"));
        assert!(is_blocked_basename("/a/id_rsa.pub"));
        assert!(is_blocked_basename("/a/id_ed25519"));
    }

    #[test]
    fn basename_blocks_pem_and_key() {
        assert!(is_blocked_basename("/a/server.pem"));
        assert!(is_blocked_basename("/a/private.key"));
    }

    #[test]
    fn basename_allows_normal() {
        assert!(!is_blocked_basename("/a/main.rs"));
        assert!(!is_blocked_basename("/a/README.md"));
    }

    #[test]
    fn enforce_root_rejects_outside() {
        let dir = tempdir().unwrap();
        let outside = "/tmp";
        let res = enforce_project_root(outside, dir.path());
        // /tmp may or may not start_with the temp dir; assert error.
        assert!(matches!(res, Err(ToolError::OutsideRoot(_))));
    }

    #[test]
    fn enforce_root_accepts_inside() {
        let dir = tempdir().unwrap();
        let inside = dir.path().join("sub").join("file.rs");
        std::fs::create_dir_all(inside.parent().unwrap()).unwrap();
        std::fs::write(&inside, b"x").unwrap();
        let res = enforce_project_root(inside.to_str().unwrap(), dir.path());
        assert!(res.is_ok());
    }

    #[test]
    fn check_all_rejects_dotenv_inside_root() {
        let dir = tempdir().unwrap();
        let env_path = dir.path().join(".env");
        std::fs::write(&env_path, b"x").unwrap();
        let res = check_all(env_path.to_str().unwrap(), dir.path());
        assert!(matches!(res, Err(ToolError::PathDenied(_))));
    }
}
