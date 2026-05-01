//! `keisei attach <brain-path> [--scope=<user|project|auto>]` implementation.
//!
//! Constructor Pattern: single responsibility — orchestrate the 7-step
//! attach ritual (canonicalize → load manifest → validate schema →
//! detect client → resolve Auto scope → adapter.attach → merge into SSoT
//! marker → print summary). No I/O here beyond what the `brain`,
//! `adapter`, and `config` modules already own.
//!
//! v0.22:
//! * `Scope::Auto` (CLI default) is resolved into a concrete `User` /
//!   `Project` by the adapter's `auto_scope()` before the attach runs —
//!   the marker never stores `Auto`.
//! * The marker merges v4-style: if a v4 marker already exists, the new
//!   attachment is appended (or replaced if `(client_type, scope)`
//!   already matches); otherwise a fresh marker is written.

use crate::adapter::{detect_active, ClientAdapter};
use crate::brain::Brain;
use crate::config::{self, AttachRecord, Attachment};
use crate::display::sanitize_display;
use crate::error::{Error, Result};
use crate::scope::Scope;
use std::path::Path;

pub fn run(brain_path: &Path, scope: Scope) -> Result<()> {
    let brain = Brain::load(brain_path)?;
    let adapter = detect_active()?;
    let resolved = resolve_scope(adapter.as_ref(), scope);
    ensure_scope_supported(adapter.as_ref(), resolved)?;
    adapter.attach(&brain, resolved)?;
    let attachment = build_attachment(&brain, adapter.as_ref(), resolved);
    let rec = merge_into_marker(attachment)?;
    let marker = config::write(&rec)?;
    print_summary(&brain, adapter.as_ref(), resolved, &marker);
    Ok(())
}

/// If the user passed `Scope::Auto`, ask the adapter to pick based on
/// its CWD heuristic. Otherwise return the scope unchanged.
fn resolve_scope(adapter: &dyn ClientAdapter, scope: Scope) -> Scope {
    if matches!(scope, Scope::Auto) {
        adapter.auto_scope()
    } else {
        scope
    }
}

fn ensure_scope_supported(adapter: &dyn ClientAdapter, scope: Scope) -> Result<()> {
    if adapter.supports_scope(scope) {
        return Ok(());
    }
    Err(Error::ScopeUnsupported {
        client: adapter.name().to_string(),
        scope: scope.to_string(),
        supported: adapter
            .supported_scopes()
            .iter()
            .map(|s| s.to_string())
            .collect(),
    })
}

fn build_attachment(brain: &Brain, adapter: &dyn ClientAdapter, scope: Scope) -> Attachment {
    Attachment {
        brain_path: brain.root.to_string_lossy().into_owned(),
        brain_name: brain.name().to_string(),
        client_type: adapter.name().to_string(),
        config_path: adapter.config_path(scope).to_string_lossy().into_owned(),
        scope,
        attached_at: config::now_utc_string(),
    }
}

/// Merge a new attachment into the existing marker, or start fresh.
/// Replaces any prior attachment with the same `(client_type, scope)` —
/// re-attaching to the same client+scope updates the entry in place.
fn merge_into_marker(new_attachment: Attachment) -> Result<AttachRecord> {
    let mut existing = config::read()?.map(|r| r.attachments).unwrap_or_default();
    existing.retain(|a| {
        !(a.client_type == new_attachment.client_type && a.scope == new_attachment.scope)
    });
    existing.push(new_attachment);
    Ok(AttachRecord::new(existing))
}

fn print_summary(
    brain: &Brain,
    adapter: &dyn ClientAdapter,
    scope: Scope,
    marker: &std::path::Path,
) {
    let brain_name = sanitize_display(brain.name());
    let brain_path = sanitize_display(&brain.root.to_string_lossy());
    println!(
        "attached brain '{}' to {} ({} scope)",
        brain_name,
        adapter.name(),
        scope
    );
    println!("  brain path: {}", brain_path);
    match brain.mcp_server_path() {
        Ok(p) => {
            let mcp_path = sanitize_display(&p.to_string_lossy());
            println!("  mcp server: {}", mcp_path);
        }
        Err(e) => println!(
            "  mcp server: [unresolved — {}]",
            sanitize_display(&e.to_string())
        ),
    }
    println!("  client cfg: {}", adapter.config_path(scope).display());
    println!("  marker:     {}", marker.display());
    let hint = adapter.post_attach_hint(brain, scope);
    println!("{}", sanitize_display(&hint));
}
