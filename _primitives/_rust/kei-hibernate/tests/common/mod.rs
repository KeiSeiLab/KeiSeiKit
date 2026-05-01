//! Shared fixtures + bundle-crafting helpers for kei-hibernate tests.
//! Constructor Pattern keeps helper code in one cube, tests split by topic.

use kei_hibernate::manifest::{HibernateManifest, MANIFEST_FILENAME, MANIFEST_VERSION};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

pub fn build_kit(dir: &Path) {
    write_file(dir, "skills/alpha/skill.sh", b"alpha");
    write_file(dir, "hooks/pre.sh", b"hook-body");
    write_file(dir, "_capabilities/cap.toml", b"[cap]");
    write_file(dir, "_roles/role.toml", b"role");
    write_file(dir, "_blocks/blk.md", b"# block");
    write_file(dir, "_agents/bot.md", b"bot");
    write_file(dir, ".claude/agents/ledger.sqlite", b"SQLITE-DUMMY-A");
    write_file(dir, ".claude/memory/kei-memory.sqlite", b"SQLITE-DUMMY-B");
    write_file(dir, ".claude/agents/skip.txt", b"should-be-excluded");
    write_file(dir, "README.md", b"root readme excluded");
}

pub fn write_file(root: &Path, rel: &str, bytes: &[u8]) {
    let p = root.join(rel);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    let mut f = fs::File::create(&p).unwrap();
    f.write_all(bytes).unwrap();
}

pub fn bundle_path(tmp: &TempDir) -> PathBuf {
    tmp.path().join("bundle.tar.zst")
}

pub fn craft_bundle_with_manifest(out: &Path, manifest: &HibernateManifest) {
    let f = fs::File::create(out).unwrap();
    let enc = zstd::Encoder::new(f, 3).unwrap().auto_finish();
    let mut b = tar::Builder::new(enc);
    let toml_s = manifest.to_toml().unwrap();
    append_blob(&mut b, MANIFEST_FILENAME, toml_s.as_bytes());
    b.finish().unwrap();
}

pub fn craft_bundle_with_extra(
    out: &Path,
    manifest: &HibernateManifest,
    name: &str,
    bytes: &[u8],
) {
    let f = fs::File::create(out).unwrap();
    let enc = zstd::Encoder::new(f, 3).unwrap().auto_finish();
    let mut b = tar::Builder::new(enc);
    append_blob(&mut b, name, bytes);
    let toml_s = manifest.to_toml().unwrap();
    append_blob(&mut b, MANIFEST_FILENAME, toml_s.as_bytes());
    b.finish().unwrap();
}

pub fn append_blob<W: Write>(b: &mut tar::Builder<W>, name: &str, bytes: &[u8]) {
    let mut h = tar::Header::new_gnu();
    h.set_size(bytes.len() as u64);
    h.set_mode(0o644);
    h.set_cksum();
    b.append_data(&mut h, name, bytes).unwrap();
}

/// Signal-only consumer so `rustc` does not flag `MANIFEST_VERSION`
/// as unused when imported by test crates that don't directly cite it.
#[allow(dead_code)]
pub const MANIFEST_VERSION_RE: &str = MANIFEST_VERSION;
