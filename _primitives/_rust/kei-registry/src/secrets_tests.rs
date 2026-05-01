//! Tests for secrets.rs — orphan-detection, env-parse, word-boundary, JSON roundtrip.

use std::io::Write;
use std::path::Path;
use tempfile::TempDir;

use super::*;

fn write_file(dir: &Path, name: &str, content: &str) {
    let p = dir.join(name);
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent).unwrap();
    }
    let mut f = std::fs::File::create(&p).unwrap();
    f.write_all(content.as_bytes()).unwrap();
}

fn make_env(dir: &Path, name: &str, content: &str) -> std::path::PathBuf {
    let p = dir.join(name);
    let mut f = std::fs::File::create(&p).unwrap();
    f.write_all(content.as_bytes()).unwrap();
    p
}

#[test]
fn test_parse_env_file_filters_correctly() {
    let tmp = TempDir::new().unwrap();
    let env_path = make_env(
        tmp.path(),
        ".env",
        "# comment\n\nANTHROPIC_API_KEY=sk-ant-xxx\nlower_key=ignored\nNO_EQUALS\nQUOTED_KEY=\"value\"\n",
    );
    let keys = parse_env_file(&env_path).unwrap();
    assert!(keys.contains(&"ANTHROPIC_API_KEY".to_string()));
    assert!(keys.contains(&"QUOTED_KEY".to_string()));
    assert!(!keys.contains(&"lower_key".to_string()));
    assert!(!keys.contains(&"NO_EQUALS".to_string()));
}

#[test]
fn test_scan_counts_usages_correctly() {
    let src_tmp = TempDir::new().unwrap();
    write_file(src_tmp.path(), "main.rs", "let x = std::env::var(\"MY_KEY\").unwrap();");
    write_file(src_tmp.path(), "config.toml", "key = \"$MY_KEY\"");
    write_file(src_tmp.path(), "other.rs", "// no secret here");

    let env_tmp = TempDir::new().unwrap();
    let env_path = make_env(env_tmp.path(), ".env", "MY_KEY=secret\nORPHAN_KEY=unused\n");

    let report = compute_secrets_report(&[env_path], src_tmp.path()).unwrap();
    let my_key = report.keys.iter().find(|k| k.name == "MY_KEY").unwrap();
    let orphan_key = report.keys.iter().find(|k| k.name == "ORPHAN_KEY").unwrap();

    assert_eq!(my_key.usage_count, 2);
    assert!(!my_key.orphan);
    assert_eq!(orphan_key.usage_count, 0);
    assert!(orphan_key.orphan);
}

#[test]
fn test_word_boundary_no_false_positive() {
    let src_tmp = TempDir::new().unwrap();
    // MY_KEY_EXTRA must NOT match MY_KEY due to word boundary.
    write_file(src_tmp.path(), "a.rs", "let _ = std::env::var(\"MY_KEY_EXTRA\");");

    let env_tmp = TempDir::new().unwrap();
    let env_path = make_env(env_tmp.path(), ".env", "MY_KEY=val\n");

    let report = compute_secrets_report(&[env_path], src_tmp.path()).unwrap();
    let row = report.keys.iter().find(|k| k.name == "MY_KEY").unwrap();
    assert_eq!(
        row.usage_count, 0,
        "word boundary regression: MY_KEY matched inside MY_KEY_EXTRA"
    );
}

#[test]
fn test_json_roundtrip() {
    let report = SecretsReport {
        keys: vec![KeyRow {
            name: "TEST_KEY".into(),
            source_env_file: "/tmp/.env".into(),
            usage_count: 3,
            usage_files: vec!["src/main.rs".into()],
            orphan: false,
        }],
        scanned_files: 10,
        env_files: vec!["/tmp/.env".into()],
    };
    let json = serde_json::to_string(&report).unwrap();
    let decoded: SecretsReport = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.keys[0].name, "TEST_KEY");
    assert_eq!(decoded.scanned_files, 10);
    assert!(!decoded.keys[0].orphan);
}

#[test]
fn test_render_ascii_shows_orphan_marker() {
    let report = SecretsReport {
        keys: vec![
            KeyRow {
                name: "ACTIVE_KEY".into(),
                source_env_file: "~/.env".into(),
                usage_count: 5,
                usage_files: vec!["src/a.rs".into()],
                orphan: false,
            },
            KeyRow {
                name: "LEGACY_TOKEN".into(),
                source_env_file: "~/.env".into(),
                usage_count: 0,
                usage_files: vec![],
                orphan: true,
            },
        ],
        scanned_files: 20,
        env_files: vec!["~/.env".into()],
    };
    let ascii = render_ascii(&report);
    assert!(ascii.contains("*ORPHAN*"));
    assert!(ascii.contains("LEGACY_TOKEN"));
    assert!(ascii.contains("ACTIVE_KEY"));
    assert!(ascii.contains("Total: 2 keys"));
    assert!(ascii.contains("1 orphan"));
}
