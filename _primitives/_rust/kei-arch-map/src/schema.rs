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

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Evidence {
    /// File exists at `path` (resolved relative to plan parent dir).
    FileExists { path: PathBuf },
    /// `pattern` (regex) matches in file `file`.
    RegexMatch { file: PathBuf, pattern: String },
    /// `cmd` exits with `expected` (default 0). Run via `sh -c`.
    /// SECURITY: cmd is fully trusted (PLAN.toml is in-repo, version-controlled).
    ExitCode {
        cmd: String,
        #[serde(default)]
        expected: i32,
    },
    /// Two integer-producing commands' stdouts must be equal.
    /// Both run via `sh -c`. Stdouts trimmed and parsed as i64.
    CountEq { cmd_a: String, cmd_b: String },
    /// HTTP GET `url` returns status in `expected` (default [200]).
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
        Evidence::ExitCode { .. } => "exit_code",
        Evidence::CountEq { .. } => "count_eq",
        Evidence::HttpStatus { .. } => "http_status",
    }
}

/// Short representation of evidence for table cells.
pub fn evidence_repr(ev: &Evidence) -> String {
    match ev {
        Evidence::FileExists { path } => format!("file_exists: {}", path.display()),
        Evidence::RegexMatch { file, pattern } => {
            format!("regex `{}` in {}", truncate(pattern, 40), file.display())
        }
        Evidence::ExitCode { cmd, expected } => {
            format!("exit {} from `{}`", expected, truncate(cmd, 60))
        }
        Evidence::CountEq { cmd_a, cmd_b } => {
            format!("`{}` == `{}`", truncate(cmd_a, 30), truncate(cmd_b, 30))
        }
        Evidence::HttpStatus { url, expected } => {
            format!("GET {} -> {:?}", url, expected)
        }
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.len() <= n {
        s.to_string()
    } else {
        format!("{}…", &s[..n])
    }
}
