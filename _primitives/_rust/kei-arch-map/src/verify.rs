use crate::schema::{self, Claim, Evidence};
use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

/// Resolve repo root: parent of PLAN.toml's parent dir.
/// E.g. `arch/PLAN.toml` → repo root is `.` (parent of `arch`).
/// Canonicalize to absolute path so `Command::current_dir` always has a real cwd.
pub fn repo_root(plan_path: &Path) -> PathBuf {
    let abs = std::fs::canonicalize(plan_path).unwrap_or_else(|_| plan_path.to_path_buf());
    abs.parent()
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Run all claims in `plan_path`. Returns Err if any FAIL (process exits 1).
pub fn run(plan_path: &Path) -> Result<()> {
    let plan = schema::load(plan_path)?;
    let root = repo_root(plan_path);
    let mut total = 0usize;
    let mut pass = 0usize;
    let mut fail = 0usize;
    for module in &plan.modules {
        for claim in &module.claims {
            total += 1;
            let (ok, reason) = check_claim(claim, &root);
            if ok {
                pass += 1;
                println!("[PASS] {}::{}", module.id, claim.id);
            } else {
                fail += 1;
                eprintln!("[FAIL] {}::{} — {}", module.id, claim.id, reason);
            }
        }
    }
    println!("Total: {} claims, {} PASS, {} FAIL", total, pass, fail);
    if fail > 0 {
        Err(anyhow!("verification failed: {} claims", fail))
    } else {
        Ok(())
    }
}

/// Check a single claim. Returns (passed, reason_if_failed).
pub fn check_claim(claim: &Claim, repo_root: &Path) -> (bool, String) {
    match &claim.evidence {
        Evidence::FileExists { path } => check_file_exists(path, repo_root),
        Evidence::RegexMatch { file, pattern } => check_regex(file, pattern, repo_root),
        Evidence::ExitCode { cmd, expected } => check_exit_code(cmd, *expected, repo_root),
        Evidence::CountEq { cmd_a, cmd_b } => check_count_eq(cmd_a, cmd_b, repo_root),
        Evidence::HttpStatus { url, expected } => check_http(url, expected),
    }
}

fn check_file_exists(path: &Path, root: &Path) -> (bool, String) {
    let resolved = if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    };
    if resolved.exists() {
        (true, String::new())
    } else {
        (false, format!("path not found: {}", resolved.display()))
    }
}

fn check_regex(file: &Path, pattern: &str, root: &Path) -> (bool, String) {
    let resolved = if file.is_absolute() {
        file.to_path_buf()
    } else {
        root.join(file)
    };
    let contents = match std::fs::read_to_string(&resolved) {
        Ok(s) => s,
        Err(e) => return (false, format!("read {} failed: {}", resolved.display(), e)),
    };
    let re = match regex::Regex::new(pattern) {
        Ok(r) => r,
        Err(e) => return (false, format!("invalid regex `{}`: {}", pattern, e)),
    };
    if re.is_match(&contents) {
        (true, String::new())
    } else {
        (false, format!("regex `{}` did not match in {}", pattern, resolved.display()))
    }
}

fn run_sh(cmd: &str, root: &Path) -> Result<std::process::Output> {
    let out = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .current_dir(root)
        .output()?;
    Ok(out)
}

fn check_exit_code(cmd: &str, expected: i32, root: &Path) -> (bool, String) {
    let out = match run_sh(cmd, root) {
        Ok(o) => o,
        Err(e) => return (false, format!("spawn failed: {}", e)),
    };
    let code = out.status.code().unwrap_or(-1);
    if code == expected {
        (true, String::new())
    } else {
        let stderr = String::from_utf8_lossy(&out.stderr);
        let trimmed: String = stderr.chars().take(200).collect();
        (
            false,
            format!("exit {} (want {}); stderr: {}", code, expected, trimmed),
        )
    }
}

fn parse_int(stdout: &[u8]) -> Result<i64> {
    let s = String::from_utf8_lossy(stdout);
    let trimmed = s.trim();
    trimmed
        .parse::<i64>()
        .map_err(|e| anyhow!("parse `{}` as i64: {}", trimmed, e))
}

fn check_count_eq(cmd_a: &str, cmd_b: &str, root: &Path) -> (bool, String) {
    let out_a = match run_sh(cmd_a, root) {
        Ok(o) => o,
        Err(e) => return (false, format!("cmd_a spawn failed: {}", e)),
    };
    let out_b = match run_sh(cmd_b, root) {
        Ok(o) => o,
        Err(e) => return (false, format!("cmd_b spawn failed: {}", e)),
    };
    let a = match parse_int(&out_a.stdout) {
        Ok(n) => n,
        Err(e) => return (false, format!("cmd_a output: {}", e)),
    };
    let b = match parse_int(&out_b.stdout) {
        Ok(n) => n,
        Err(e) => return (false, format!("cmd_b output: {}", e)),
    };
    if a == b {
        (true, String::new())
    } else {
        (false, format!("count_eq: {} != {}", a, b))
    }
}

fn check_http(url: &str, expected: &[u16]) -> (bool, String) {
    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(e) => return (false, format!("http client build failed: {}", e)),
    };
    let resp = match client.get(url).send() {
        Ok(r) => r,
        Err(e) => return (false, format!("GET {} failed: {}", url, e)),
    };
    let status = resp.status().as_u16();
    if expected.contains(&status) {
        (true, String::new())
    } else {
        (false, format!("status {} not in {:?}", status, expected))
    }
}
