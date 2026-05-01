//! Integration tests for the v0.16 substrate-role field (phase 5).
//!
//! Confirms that when a manifest declares `substrate_role`, the assembler:
//!   1. Reads `_roles/<role>.toml` from the kit root
//!   2. Concatenates each capability's `_capabilities/<cat>/<slug>/text.md`
//!   3. Emits the fragments as a new `# AGENT SUBSTRATE` section between
//!      `# ROLE` and the first behavioural block, preserving the existing
//!      generation for manifests that do NOT declare the field.

mod common;

use common::{assemble_bin, read_generated};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Kit root (parent of `_assembler/`). Used by migrated manifests that
/// reference real `_roles/` + `_capabilities/` content.
fn kit_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

/// Mirror `_manifests/`, `_blocks/`, `_roles/`, `_capabilities/` from
/// the live kit into a temp dir so the test is hermetic.
fn seed_full_kit() -> (TempDir, PathBuf) {
    let tmp = TempDir::new().expect("mktempdir");
    let root = tmp.path().to_path_buf();
    let src = kit_root();
    for sub in ["_manifests", "_blocks", "_roles"] {
        mirror_flat(&src.join(sub), &root.join(sub));
    }
    mirror_caps(&src.join("_capabilities"), &root.join("_capabilities"));
    (tmp, root)
}

fn mirror_flat(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mkdir dst");
    for entry in fs::read_dir(from).expect("read src").flatten() {
        let p = entry.path();
        if p.is_file() {
            fs::copy(&p, to.join(p.file_name().unwrap())).expect("copy");
        }
    }
}

fn mirror_caps(from: &Path, to: &Path) {
    fs::create_dir_all(to).expect("mkdir caps root");
    for cat in fs::read_dir(from).expect("read caps").flatten() {
        let cat_path = cat.path();
        if !cat_path.is_dir() { continue; }
        let cat_dst = to.join(cat_path.file_name().unwrap());
        fs::create_dir_all(&cat_dst).expect("mkdir cat");
        for slug in fs::read_dir(&cat_path).expect("read cat").flatten() {
            let slug_path = slug.path();
            if !slug_path.is_dir() { continue; }
            let slug_dst = cat_dst.join(slug_path.file_name().unwrap());
            fs::create_dir_all(&slug_dst).expect("mkdir slug");
            for file in fs::read_dir(&slug_path).expect("read slug").flatten() {
                let fp = file.path();
                if fp.is_file() {
                    fs::copy(&fp, slug_dst.join(fp.file_name().unwrap())).expect("copy cap");
                }
            }
        }
    }
}

fn assemble(root: &Path, manifest: &str) -> (bool, String, String) {
    let path = root.join("_manifests").join(format!("{manifest}.toml"));
    let out = Command::new(assemble_bin())
        .env("AGENT_ROOT", root)
        .env("HOME", root)
        .arg(path)
        .output()
        .expect("spawn");
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).to_string(),
        String::from_utf8_lossy(&out.stderr).to_string(),
    )
}

#[test]
fn migrated_code_implementer_embeds_substrate_section() {
    let (_tmp, root) = seed_full_kit();
    let (ok, _stdout, stderr) = assemble(&root, "code-implementer");
    assert!(ok, "assemble failed: {stderr}");
    let md = read_generated(&root, "code-implementer");
    assert!(md.contains("# AGENT SUBSTRATE — role `edit-local`"),
        "substrate section header missing in generated md");
    assert!(md.contains("You MUST NOT invoke `git`"),
        "policy::no-git-ops text.md fragment missing");
    assert!(md.contains("under 200 lines of code"),
        "quality::constructor-pattern text.md fragment missing");
    // Existing block content still present.
    assert!(md.contains("# BASELINE"), "baseline block dropped during substrate injection");
    assert!(md.contains("# DOMAIN SCOPE"), "domain scope section dropped");
}

#[test]
fn migrated_read_only_agents_embed_read_only_substrate() {
    let (_tmp, root) = seed_full_kit();
    for name in ["critic", "architect", "security-auditor", "validator"] {
        let (ok, _stdout, stderr) = assemble(&root, name);
        assert!(ok, "assemble {name} failed: {stderr}");
        let md = read_generated(&root, name);
        assert!(md.contains("# AGENT SUBSTRATE — role `read-only`"),
            "{name}: substrate section header missing");
        assert!(md.contains("You MUST NOT use the `Edit` or `Write` tools"),
            "{name}: tools::deny-tools text.md fragment missing");
    }
}

#[test]
fn non_migrated_agent_has_no_substrate_section() {
    // v0.16 phase-5 wave 2 (2026-04-23): all 12 kit-shipped agents now
    // carry `substrate_role`, so we synthesize a non-migrated manifest
    // by stripping the field from a copy of `researcher.toml`
    // inside the temp kit. This keeps the gate-test invariant honest
    // without requiring a permanently-unmigrated shipping manifest.
    let (_tmp, root) = seed_full_kit();
    let manifest_path = root.join("_manifests").join("researcher.toml");
    let original = fs::read_to_string(&manifest_path).expect("read manifest");
    let stripped: String = original
        .lines()
        .filter(|line| !line.trim_start().starts_with("substrate_role"))
        .collect::<Vec<_>>()
        .join("\n");
    fs::write(&manifest_path, stripped).expect("write stripped manifest");

    let (ok, _stdout, stderr) = assemble(&root, "researcher");
    assert!(ok, "assemble failed: {stderr}");
    let md = read_generated(&root, "researcher");
    assert!(!md.contains("# AGENT SUBSTRATE"),
        "non-migrated agent must not emit substrate section");
}

#[test]
fn substrate_section_precedes_first_block() {
    // Invariant: substrate fragments are injected AFTER `# ROLE` and
    // BEFORE the first `_blocks/*.md` block (baseline).
    let (_tmp, root) = seed_full_kit();
    let (ok, _stdout, stderr) = assemble(&root, "code-implementer");
    assert!(ok, "assemble failed: {stderr}");
    let md = read_generated(&root, "code-implementer");
    let role_pos = md.find("# ROLE").expect("# ROLE missing");
    let substrate_pos = md.find("# AGENT SUBSTRATE").expect("# AGENT SUBSTRATE missing");
    let baseline_pos = md.find("# BASELINE").expect("# BASELINE missing");
    assert!(role_pos < substrate_pos, "substrate must come AFTER # ROLE");
    assert!(substrate_pos < baseline_pos, "substrate must come BEFORE first block");
}
