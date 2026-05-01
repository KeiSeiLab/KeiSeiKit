//! Shared helpers for rule_blocks integration tests.
//! Separate from `common/mod.rs` because these helpers depend on rusqlite
//! and are only needed by rule_blocks tests.

use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub fn seed_schema(conn: &Connection) {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS blocks (
            id            INTEGER PRIMARY KEY AUTOINCREMENT,
            dna           TEXT NOT NULL UNIQUE,
            block_type    TEXT NOT NULL,
            name          TEXT NOT NULL,
            path          TEXT NOT NULL,
            caps          TEXT NOT NULL,
            scope_sha     TEXT NOT NULL,
            body_sha      TEXT NOT NULL,
            nonce         TEXT NOT NULL,
            created       INTEGER NOT NULL,
            modified      INTEGER NOT NULL,
            superseded_by TEXT
        );",
    )
    .expect("create schema");
}

pub fn insert_rule(conn: &Connection, name: &str, path: &str) {
    conn.execute(
        "INSERT INTO blocks \
         (dna, block_type, name, path, caps, scope_sha, body_sha, nonce, created, modified)
         VALUES (?1, 'rule', ?2, ?3, 'md', 'aa', 'bb', 'cc', 0, 0)",
        rusqlite::params![
            format!("rule::md::aaaa::bbbb-cccc-{name}"),
            name,
            path,
        ],
    )
    .expect("insert rule");
}

/// Create a temp directory with the assembler fixture structure + write a
/// manifest TOML with the given `rule_blocks` field.
/// Returns (TempDir guard, kit root path, registry DB path).
pub fn setup_kit(
    rule_blocks: &[&str],
    frag_files: &[(&str, &str)],
) -> (TempDir, PathBuf, PathBuf) {
    let tmp = TempDir::new().expect("mktempdir");
    let root = tmp.path().to_path_buf();

    let fx = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures");
    let kit = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf();

    copy_dir(&fx.join("_manifests"), &root.join("_manifests"));
    copy_dir(&fx.join("_blocks"), &root.join("_blocks"));
    copy_dir(&kit.join("_roles"), &root.join("_roles"));
    copy_caps(&kit.join("_capabilities"), &root.join("_capabilities"));

    let db_path = root.join("registry.sqlite");
    let conn = Connection::open(&db_path).expect("open db");
    seed_schema(&conn);

    let frags_dir = root.join("_rule_frags");
    fs::create_dir_all(&frags_dir).expect("mkdir frags");
    for (name, body) in frag_files {
        let file = frags_dir.join(format!("{}.md", name.replace("::", "--")));
        fs::write(&file, body).expect("write frag");
        insert_rule(&conn, name, file.to_str().unwrap());
    }
    drop(conn);

    let rule_blocks_toml = if rule_blocks.is_empty() {
        String::new()
    } else {
        let list = rule_blocks
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>()
            .join(", ");
        format!("rule_blocks = [{}]\n", list)
    };

    let manifest = format!(
        "name = \"test-rule-blocks\"\n\
         description = \"test manifest for rule_blocks integration tests\"\n\
         tools = [\"Read\"]\n\
         model = \"opus\"\n\
         substrate_role = \"read-only\"\n\
         role = \"Test role text.\"\n\
         blocks = [\"baseline\", \"evidence-grading\", \"memory-protocol\"]\n\
         domain_in = [\"test domain\"]\n\
         forbidden_domain = [\"forbidden action\"]\n\
         {rule_blocks_toml}\n\
         [[handoff]]\n\
         target = \"architect\"\n\
         trigger = \"test handoff\"\n"
    );
    fs::write(
        root.join("_manifests").join("test-rule-blocks.toml"),
        manifest,
    )
    .expect("write manifest");

    (tmp, root, db_path)
}

pub fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mkdir dst");
    if !from.is_dir() {
        return;
    }
    for entry in fs::read_dir(from).expect("read src").flatten() {
        let p = entry.path();
        if p.is_file() {
            fs::copy(&p, to.join(p.file_name().unwrap())).expect("copy file");
        }
    }
}

pub fn copy_caps(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mkdir caps root");
    for cat in fs::read_dir(from).expect("read caps").flatten() {
        let cat_path = cat.path();
        if !cat_path.is_dir() {
            continue;
        }
        let cat_dst = to.join(cat_path.file_name().unwrap());
        fs::create_dir_all(&cat_dst).expect("mkdir cat");
        for slug in fs::read_dir(&cat_path).expect("read cat").flatten() {
            let slug_path = slug.path();
            if !slug_path.is_dir() {
                continue;
            }
            let slug_dst = cat_dst.join(slug_path.file_name().unwrap());
            fs::create_dir_all(&slug_dst).expect("mkdir slug");
            for file in fs::read_dir(&slug_path).expect("read slug").flatten() {
                let fp = file.path();
                if fp.is_file() {
                    fs::copy(&fp, slug_dst.join(fp.file_name().unwrap()))
                        .expect("copy cap file");
                }
            }
        }
    }
}
