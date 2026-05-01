//! `keisei mount <brain-path>` — attach to every detected client.
//!
//! Constructor Pattern: single responsibility — orchestrate the fan-out
//! (load brain → enumerate adapters → for each detecting adapter, pick
//! per-adapter scope via `auto_scope()` → attach → collect successes /
//! failures → write v4 marker with one attachment per success → print
//! summary).
//!
//! v0.22: mount resolves scope per-adapter via `auto_scope()` rather than
//! forcing `Scope::User` — a user running `keisei mount brain` inside
//! `team-repo/` with `.cursor/` present will get project-scope Cursor +
//! user-scope Claude Code in a single command. The v4 marker stores each
//! attachment's resolved scope so `detach` can reverse the fan-out exactly.

use crate::adapter;
use crate::brain::Brain;
use crate::config::{self, AttachRecord, Attachment};
use crate::display::sanitize_display;
use crate::error::{Error, Result};
use crate::scope::Scope;
use std::path::Path;

pub fn run(brain_path: &Path) -> Result<()> {
    let brain = Brain::load(brain_path)?;
    let (succeeded, failed) = mount_all(&brain);
    if succeeded.is_empty() {
        print_all_failed(&failed);
        return Err(Error::NoClientDetected);
    }
    let rec = build_record(&brain, &succeeded);
    let marker = config::write(&rec)?;
    print_summary(&brain, &succeeded, &failed, &marker);
    Ok(())
}

struct Success {
    client_type: String,
    config_path: String,
    scope: Scope,
}

/// Returns `(succeeded, failed)` where:
///   - succeeded: adapters that detected AND attached OK at their auto-scope
///   - failed:    adapters that detected BUT attach() errored
/// Adapters that didn't detect aren't reported either way.
fn mount_all(brain: &Brain) -> (Vec<Success>, Vec<(String, String)>) {
    let mut ok = Vec::new();
    let mut err = Vec::new();
    for a in adapter::all() {
        if !a.detect() {
            continue;
        }
        let scope = a.auto_scope();
        match a.attach(brain, scope) {
            Ok(()) => ok.push(Success {
                client_type: a.name().to_string(),
                config_path: a.config_path(scope).to_string_lossy().into_owned(),
                scope,
            }),
            Err(e) => err.push((a.name().to_string(), e.to_string())),
        }
    }
    (ok, err)
}

fn build_record(brain: &Brain, succeeded: &[Success]) -> AttachRecord {
    let now = config::now_utc_string();
    let attachments = succeeded
        .iter()
        .map(|s| Attachment {
            brain_path: brain.root.to_string_lossy().into_owned(),
            brain_name: brain.name().to_string(),
            client_type: s.client_type.clone(),
            config_path: s.config_path.clone(),
            scope: s.scope,
            attached_at: now.clone(),
        })
        .collect();
    AttachRecord::new(attachments)
}

fn print_all_failed(failed: &[(String, String)]) {
    eprintln!("keisei: no MCP-capable client detected on this host");
    for (client, reason) in failed {
        eprintln!(
            "  ! {}: {}",
            sanitize_display(client),
            sanitize_display(reason)
        );
    }
    eprintln!("install Claude Code, Cursor, Continue, or Zed, then retry.");
}

fn print_summary(
    brain: &Brain,
    ok: &[Success],
    err: &[(String, String)],
    marker: &std::path::Path,
) {
    println!("mounted brain '{}' to:", sanitize_display(brain.name()));
    for s in ok {
        println!(
            "  [OK] {} ({}): {}",
            sanitize_display(&s.client_type),
            s.scope,
            sanitize_display(&s.config_path)
        );
    }
    for (client, reason) in err {
        eprintln!(
            "  [FAIL] {}: {}",
            sanitize_display(client),
            sanitize_display(reason)
        );
    }
    println!("marker: {}", marker.display());
    println!("run `keisei status` to inspect, `keisei detach` to remove.");
}
