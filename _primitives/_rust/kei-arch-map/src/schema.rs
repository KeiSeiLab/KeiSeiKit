use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
    pub meta: Meta,
    #[serde(default, rename = "module")]
    pub modules: Vec<Module>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub schema_version: u32,
    #[serde(default)]
    pub repo_root: Option<String>,
    #[serde(default)]
    pub github_blob_base: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    pub path: PathBuf,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default, rename = "claim")]
    pub claims: Vec<Claim>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claim {
    pub id: String,
    pub description: String,
    pub evidence: Evidence,
}

/// Allowlisted evidence kinds. NO raw shell.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Evidence {
    /// File exists at `path` (relative to repo root).
    FileExists { path: PathBuf },
    /// Regex matches in file. Pattern compiled with size_limit cap.
    RegexMatch { file: PathBuf, pattern: String },
    /// Count lines matching regex in file equals `expected`.
    /// Replaces the `grep -c` ExitCode pattern.
    GrepCount {
        file: PathBuf,
        pattern: String,
        expected: u64,
    },
    /// File size in bytes is in `range` (inclusive). Use `[N, N]` for exact.
    FileSize { path: PathBuf, range: [u64; 2] },
    /// Dotted JSON path equals `expected` string. No wildcards.
    JsonField {
        file: PathBuf,
        path: String,
        expected: String,
    },
    /// `cargo check --workspace --offline --message-format=json` produces
    /// zero compiler-error diagnostics, run from `manifest_dir`.
    CargoCheckClean { manifest_dir: PathBuf },
    /// `cargo check --workspace --offline --message-format=json` produces zero
    /// compiler-error messages. SAFETY: caller must whitelist `manifest_dir`;
    /// this kind runs build.rs of the workspace. Use only on internally-
    /// controlled workspaces. External / untrusted manifests MUST use
    /// `CargoCheckClean` (manifest-resolve only via cargo metadata).
    CargoCheckSafe {
        manifest_dir: PathBuf,
        /// Allowlist of paths (relative to repo root) authorised to run
        /// build.rs. Required: refuse to run if `manifest_dir` does not
        /// match. Typical: `["_primitives/_rust"]`.
        #[serde(default)]
        allowed_paths: Vec<PathBuf>,
    },
    /// HTTP GET URL returns one of `expected` codes. SSRF-hardened.
    HttpStatus {
        url: String,
        #[serde(default = "default_2xx")]
        expected: Vec<u16>,
    },
}

fn default_2xx() -> Vec<u16> {
    vec![200]
}

pub fn load(path: &std::path::Path) -> anyhow::Result<Plan> {
    let s = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&s)?)
}

/// Short label for evidence kind (for tables in rendered docs).
pub fn evidence_kind(ev: &Evidence) -> &'static str {
    match ev {
        Evidence::FileExists { .. } => "file_exists",
        Evidence::RegexMatch { .. } => "regex_match",
        Evidence::GrepCount { .. } => "grep_count",
        Evidence::FileSize { .. } => "file_size",
        Evidence::JsonField { .. } => "json_field",
        Evidence::CargoCheckClean { .. } => "cargo_check_clean",
        Evidence::CargoCheckSafe { .. } => "cargo_check_safe",
        Evidence::HttpStatus { .. } => "http_status",
    }
}

/// Short representation of evidence for table cells.
pub fn evidence_repr(ev: &Evidence) -> String {
    match ev {
        Evidence::FileExists { path } => format!("file_exists: {}", path.display()),
        Evidence::RegexMatch { file, pattern } => repr_regex(file, pattern),
        Evidence::GrepCount { file, pattern, expected } => repr_grep(file, pattern, *expected),
        Evidence::FileSize { path, range } => repr_size(path, range),
        Evidence::JsonField { file, path, expected } => repr_json(file, path, expected),
        Evidence::CargoCheckClean { manifest_dir } => {
            format!("cargo_check_clean({})", manifest_dir.display())
        }
        Evidence::CargoCheckSafe { manifest_dir, allowed_paths } => {
            format!(
                "cargo_check_safe({}, allowed={})",
                manifest_dir.display(),
                allowed_paths.len()
            )
        }
        Evidence::HttpStatus { url, expected } => {
            format!("GET {} -> {:?}", truncate(url, 60), expected)
        }
    }
}

fn repr_regex(file: &std::path::Path, pattern: &str) -> String {
    format!("regex `{}` in {}", truncate(pattern, 40), file.display())
}

fn repr_grep(file: &std::path::Path, pattern: &str, expected: u64) -> String {
    format!(
        "grep_count `{}` in {} == {}",
        truncate(pattern, 30),
        file.display(),
        expected
    )
}

fn repr_size(path: &std::path::Path, range: &[u64; 2]) -> String {
    format!("size({}) in [{}..={}]", path.display(), range[0], range[1])
}

fn repr_json(file: &std::path::Path, path: &str, expected: &str) -> String {
    format!(
        "json `{}`.{} == `{}`",
        file.display(),
        path,
        truncate(expected, 30)
    )
}

/// UTF-8-safe truncate by character count, appending "…" if truncated.
pub(crate) fn truncate(s: &str, n: usize) -> String {
    let mut chars = s.chars();
    let head: String = chars.by_ref().take(n).collect();
    if chars.next().is_some() {
        format!("{}…", head)
    } else {
        head
    }
}
