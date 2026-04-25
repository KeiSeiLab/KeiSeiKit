//! Integration tests for kei-store.

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_kei-store"))
}

fn write_config(tmp: &TempDir, backend: &str, local: &PathBuf) -> PathBuf {
    let cfg_path = tmp.path().join("c.toml");
    let body = format!(
        "[active]\nbackend = \"{b}\"\nlocal_path = \"{p}\"\n\n[filesystem]\npath = \"{p}\"\n\n[s3]\ncache_path = \"{p}\"\n",
        b = backend,
        p = local.to_string_lossy()
    );
    fs::write(&cfg_path, body).unwrap();
    cfg_path
}

fn run(args: &[&str]) -> std::process::Output {
    std::process::Command::new(bin()).args(args).output().unwrap()
}

fn run_with_env(args: &[&str], env: &[(&str, &str)]) -> std::process::Output {
    let mut cmd = std::process::Command::new(bin());
    cmd.args(args);
    for (k, v) in env {
        cmd.env(k, v);
    }
    cmd.output().unwrap()
}

#[test]
fn init_writes_config() {
    let tmp = TempDir::new().unwrap();
    let cfg = tmp.path().join("store.toml");
    let out = run(&[
        "--config",
        cfg.to_str().unwrap(),
        "init",
        "filesystem",
    ]);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(cfg.exists());
    let text = fs::read_to_string(&cfg).unwrap();
    assert!(text.contains("backend = \"filesystem\""));
}

#[test]
fn filesystem_read_write_roundtrip() {
    let tmp = TempDir::new().unwrap();
    let local = tmp.path().join("repo");
    let cfg = write_config(&tmp, "filesystem", &local);
    let file = tmp.path().join("input.bin");
    fs::write(&file, b"hello world").unwrap();
    let w = run(&[
        "--config", cfg.to_str().unwrap(),
        "write", "a/b.txt", file.to_str().unwrap(),
    ]);
    assert!(w.status.success(), "{}", String::from_utf8_lossy(&w.stderr));
    let r = run(&[
        "--config", cfg.to_str().unwrap(),
        "read", "a/b.txt",
    ]);
    assert!(r.status.success());
    assert_eq!(r.stdout, b"hello world");
}

#[test]
fn filesystem_list_shows_files() {
    let tmp = TempDir::new().unwrap();
    let local = tmp.path().join("repo");
    let cfg = write_config(&tmp, "filesystem", &local);
    let file = tmp.path().join("x");
    fs::write(&file, b"x").unwrap();
    run(&["--config", cfg.to_str().unwrap(), "write", "dir/a", file.to_str().unwrap()]);
    run(&["--config", cfg.to_str().unwrap(), "write", "dir/b", file.to_str().unwrap()]);
    let out = run(&["--config", cfg.to_str().unwrap(), "list", "dir"]);
    assert!(out.status.success());
    let s = String::from_utf8(out.stdout).unwrap();
    assert!(s.contains("a"));
    assert!(s.contains("b"));
}

#[test]
fn filesystem_commit_returns_hash() {
    let tmp = TempDir::new().unwrap();
    let local = tmp.path().join("repo");
    let cfg = write_config(&tmp, "filesystem", &local);
    let file = tmp.path().join("x");
    fs::write(&file, b"x").unwrap();
    run(&["--config", cfg.to_str().unwrap(), "write", "a.txt", file.to_str().unwrap()]);
    let out = run(&["--config", cfg.to_str().unwrap(), "commit", "--message", "init"]);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let hash = String::from_utf8(out.stdout).unwrap();
    assert!(hash.trim().len() >= 7);
}

#[test]
fn filesystem_push_pull_are_noop() {
    let tmp = TempDir::new().unwrap();
    let local = tmp.path().join("repo");
    let cfg = write_config(&tmp, "filesystem", &local);
    let file = tmp.path().join("x");
    fs::write(&file, b"x").unwrap();
    run(&["--config", cfg.to_str().unwrap(), "write", "a.txt", file.to_str().unwrap()]);
    run(&["--config", cfg.to_str().unwrap(), "commit", "--message", "init"]);
    let p1 = run(&["--config", cfg.to_str().unwrap(), "push", "main"]);
    let p2 = run(&["--config", cfg.to_str().unwrap(), "pull", "main"]);
    assert!(p1.status.success());
    assert!(p2.status.success());
}

#[test]
fn s3_stub_commit_writes_manifest() {
    let tmp = TempDir::new().unwrap();
    let local = tmp.path().join("cache");
    let cfg = write_config(&tmp, "s3", &local);
    let file = tmp.path().join("x");
    fs::write(&file, b"x").unwrap();
    // v0.14.1: S3 stub requires explicit opt-in env var.
    run_with_env(
        &["--config", cfg.to_str().unwrap(), "write", "a.txt", file.to_str().unwrap()],
        &[("KEI_STORE_ALLOW_S3_STUB", "1")],
    );
    let out = run_with_env(
        &["--config", cfg.to_str().unwrap(), "commit", "--message", "first"],
        &[("KEI_STORE_ALLOW_S3_STUB", "1")],
    );
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let entries: Vec<_> = fs::read_dir(&local)
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().starts_with("manifest-"))
        .collect();
    assert_eq!(entries.len(), 1);
}

#[test]
fn s3_backend_requires_env_optin() {
    let tmp = TempDir::new().unwrap();
    let local = tmp.path().join("cache");
    let cfg = write_config(&tmp, "s3", &local);
    // Without KEI_STORE_ALLOW_S3_STUB, status must fail with a clear message.
    let mut cmd = std::process::Command::new(bin());
    cmd.args(["--config", cfg.to_str().unwrap(), "status"]);
    cmd.env_remove("KEI_STORE_ALLOW_S3_STUB");
    let out = cmd.output().unwrap();
    assert!(!out.status.success());
    let msg = String::from_utf8_lossy(&out.stderr);
    assert!(msg.contains("KEI_STORE_ALLOW_S3_STUB"), "expected stub-gate message, got: {msg}");
}

#[test]
fn status_reports_backend() {
    let tmp = TempDir::new().unwrap();
    let local = tmp.path().join("repo");
    let cfg = write_config(&tmp, "filesystem", &local);
    let out = run(&["--config", cfg.to_str().unwrap(), "status"]);
    assert!(out.status.success());
    let s = String::from_utf8(out.stdout).unwrap();
    assert!(s.contains("filesystem"));
}

#[test]
fn unknown_backend_errors() {
    let tmp = TempDir::new().unwrap();
    let local = tmp.path().join("repo");
    let cfg_path = tmp.path().join("c.toml");
    let body = format!(
        "[active]\nbackend = \"xyz\"\nlocal_path = \"{p}\"\n",
        p = local.to_string_lossy()
    );
    fs::write(&cfg_path, body).unwrap();
    let out = run(&["--config", cfg_path.to_str().unwrap(), "status"]);
    assert!(!out.status.success());
    let e = String::from_utf8_lossy(&out.stderr);
    assert!(e.contains("unknown backend"), "{}", e);
}
