//! `keisei detach` implementation.
//!
//! Constructor Pattern: single responsibility — read the marker,
//! iterate recorded attachments (each carrying its own `brain_name`,
//! `scope`, `client_type`), call `adapter.detach(brain_name, scope)` on
//! each, delete the marker file after all adapters succeed. Per-adapter
//! failures are collected and reported but do NOT abort the other
//! detaches — partial detach is better than stuck state.
//!
//! v0.22: marker is now v4 (per-attachment `brain_path` + `brain_name`);
//! detach iterates each `Attachment` directly rather than reading a
//! single top-level `brain_name`. Multi-brain markers detach ALL
//! attachments by default.

use crate::adapter;
use crate::config::{self, AttachRecord};
use crate::display::sanitize_display;
use crate::error::Result;

pub fn run() -> Result<()> {
    let Some(rec) = config::read()? else {
        println!("no brain attached; nothing to detach");
        return Ok(());
    };

    let (succeeded, failed) = detach_all(&rec);
    print_summary(&rec, &succeeded, &failed);

    if failed.is_empty() {
        config::delete()?;
    } else {
        eprintln!(
            "keisei: {} adapter(s) failed to detach cleanly — marker retained",
            failed.len()
        );
    }
    Ok(())
}

struct DetachOutcome {
    client: String,
    brain: String,
}

/// For each attachment in the marker, run `adapter.detach(brain_name, scope)`.
/// Returns `(succeeded, failed_pairs)`.
fn detach_all(rec: &AttachRecord) -> (Vec<DetachOutcome>, Vec<(String, String)>) {
    let mut ok = Vec::new();
    let mut err = Vec::new();
    for a in &rec.attachments {
        match adapter::by_name(&a.client_type) {
            Some(adapter) => match adapter.detach(&a.brain_name, a.scope) {
                Ok(()) => ok.push(DetachOutcome {
                    client: a.client_type.clone(),
                    brain: a.brain_name.clone(),
                }),
                Err(e) => err.push((a.client_type.clone(), e.to_string())),
            },
            None => err.push((
                a.client_type.clone(),
                "unknown adapter (not registered)".to_string(),
            )),
        }
    }
    (ok, err)
}

fn print_summary(rec: &AttachRecord, ok: &[DetachOutcome], err: &[(String, String)]) {
    if !ok.is_empty() {
        println!("detached:");
        for d in ok {
            println!(
                "  - {} ({})",
                sanitize_display(&d.client),
                sanitize_display(&d.brain)
            );
        }
    }
    for (client, reason) in err {
        eprintln!(
            "  ! {}: {}",
            sanitize_display(client),
            sanitize_display(reason)
        );
    }
    // Print each distinct brain_path referenced in the marker — one
    // line each so multi-brain markers show every detachment.
    let mut seen: Vec<&str> = Vec::new();
    for a in &rec.attachments {
        if !seen.contains(&a.brain_path.as_str()) {
            println!("brain was: {}", sanitize_display(&a.brain_path));
            seen.push(a.brain_path.as_str());
        }
    }
}
