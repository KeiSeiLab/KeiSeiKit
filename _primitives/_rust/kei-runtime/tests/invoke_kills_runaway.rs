//! Integration test — runaway atom that floods stdout MUST be killed
//! after the 16 MiB cap, not buffered to OOM.
//!
//! Wave 44d resource-cap: replaces the post-hoc `cap_bytes` truncation
//! with streamed reads in `invoke_io.rs`. This test pins the new
//! behaviour: a fake atom binary that emits 100 MiB of zeros must
//! exit non-zero (killed by parent) rather than complete normally
//! with 100 MiB buffered.
//!
//! Strategy:
//!   1. Build a tiny shell-script "atom" that ignores stdin and writes
//!      100 MiB of zeros to stdout. We can't use `dd` directly because
//!      the runtime's allowlist enforces `kei-*` crate names — so we
//!      stage a script named `kei-flood` in a temp bin dir.
//!   2. Stage atom YAML naming `kei-flood::pour`.
//!   3. Invoke via the runtime CLI; expect non-zero exit + a stderr
//!      message naming the cap.

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_kei-runtime");

fn write_atom_md(root: &Path, crate_name: &str, verb: &str) {
    let atoms = root.join(crate_name).join("atoms");
    let schemas = atoms.join("schemas");
    fs::create_dir_all(&schemas).unwrap();
    let in_schema = r#"{"$schema":"http://json-schema.org/draft-07/schema#","type":"object"}"#;
    let out_schema = r#"{"$schema":"http://json-schema.org/draft-07/schema#","type":"object"}"#;
    fs::write(schemas.join(format!("{verb}-input.json")), in_schema).unwrap();
    fs::write(schemas.join(format!("{verb}-output.json")), out_schema).unwrap();
    let md = format!(
        "---\natom: {crate_name}::{verb}\nkind: command\nversion: \"0.1.0\"\n\
         input:\n  schema: schemas/{verb}-input.json\n\
         output:\n  schema: schemas/{verb}-output.json\n\
         side_effects: []\nidempotent: true\nstability: stable\n---\n"
    );
    fs::write(atoms.join(format!("{verb}.md")), md).unwrap();
}

/// Stage a `kei-flood` shell-script in `bin_dir`. When invoked with
/// `run-atom pour` it writes a continuous stream of zeros to stdout
/// (well past the 16 MiB cap) using a bash builtin loop — no
/// dependency on external `dd` because PATH may be locked down.
/// The parent runtime should kill it well before it finishes.
fn stage_flood_binary(bin_dir: &Path) -> std::path::PathBuf {
    fs::create_dir_all(bin_dir).unwrap();
    let script = bin_dir.join("kei-flood");
    // Pipe /dev/zero straight to stdout via /bin/cat, which is
    // present at /bin/cat on macOS and Linux. The script will run
    // unbounded; the parent runtime is expected to kill it after
    // the 16 MiB cap. SIGPIPE on cat (when the parent stops reading
    // and the pipe closes) is benign — we just want enough volume
    // to provably exceed the cap.
    let body = "#!/bin/sh\nexec /bin/cat /dev/zero\n";
    fs::write(&script, body).unwrap();
    let mut perms = fs::metadata(&script).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&script, perms).unwrap();
    script
}

#[test]
fn invoke_kills_runaway_atom() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("root");
    let bin = tmp.path().join("bin");
    write_atom_md(&root, "kei-flood", "pour");
    let _script = stage_flood_binary(&bin);
    let out = Command::new(BIN)
        .env("KEI_RUNTIME_BIN_DIR", &bin)
        .env("PATH", &bin)
        .arg("invoke")
        .arg("kei-flood::pour")
        .arg("--input")
        .arg("{}")
        .arg("--root")
        .arg(&root)
        .output()
        .expect("spawn kei-runtime");
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    let stdout_len = out.stdout.len();
    // The runtime printed at most a small JSON envelope OR nothing
    // (process killed). 16 MiB cap + a sliver of envelope JSON → well
    // under 18 MiB. If the cap had failed, stdout would be ~100 MiB.
    assert!(
        stdout_len < 18 * 1024 * 1024,
        "expected stdout < 18 MiB; got {stdout_len} (cap not enforced!)"
    );
    // We expect a non-zero exit because the child was killed.
    assert_ne!(
        out.status.code(),
        Some(0),
        "expected non-zero exit on runaway; stderr: {stderr}"
    );
    // Stderr should mention the cap so the operator can diagnose.
    assert!(
        stderr.contains("cap") || stderr.contains("subprocess"),
        "expected 'cap' / 'subprocess' in stderr: {stderr}"
    );
}
