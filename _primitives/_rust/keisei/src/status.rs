//! `keisei status` implementation.
//!
//! Constructor Pattern: single responsibility — read the
//! `attached.toml` SSoT (v1..v4), verify each brain + its mcp binary
//! still exists, print a human-readable summary with per-client health.
//!
//! v0.22: marker is v4 (per-attachment brain fields) so status groups the
//! output by brain: one header per unique `brain_path`, then the list of
//! `(client, scope, config_path)` attached to it, then a health check.

use crate::brain::Brain;
use crate::config::{self, AttachRecord, Attachment};
use crate::display::sanitize_display;
use crate::error::Result;
use std::path::PathBuf;

pub fn run() -> Result<()> {
    match config::read()? {
        None => {
            println!("no brain attached");
            println!("run: keisei attach <brain-path>  or  keisei mount <brain-path>");
            Ok(())
        }
        Some(rec) => {
            if rec.attachments.is_empty() {
                println!("marker present but has no attachments (migrated v1 marker?)");
                return Ok(());
            }
            print_grouped_by_brain(&rec);
            Ok(())
        }
    }
}

fn print_grouped_by_brain(rec: &AttachRecord) {
    for brain_path in unique_brain_paths(rec) {
        let group: Vec<&Attachment> = rec
            .attachments
            .iter()
            .filter(|a| a.brain_path == brain_path)
            .collect();
        let Some(head) = group.first() else { continue };
        println!("brain:       {}", sanitize_display(&head.brain_name));
        println!("brain path:  {}", sanitize_display(&head.brain_path));
        println!("attached at: {}", sanitize_display(&head.attached_at));
        let names: Vec<String> = group
            .iter()
            .map(|a| format!("{} ({})", a.client_type, a.scope))
            .collect();
        println!("clients:     {}", sanitize_display(&names.join(", ")));
        for a in &group {
            let cfg = if a.config_path.is_empty() {
                "(unknown — v1 marker)".to_string()
            } else {
                sanitize_display(&a.config_path)
            };
            println!(
                "  - {} ({}): {}",
                sanitize_display(&a.client_type),
                a.scope,
                cfg
            );
        }
        print_health(&brain_path);
        println!();
    }
}

fn unique_brain_paths(rec: &AttachRecord) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    for a in &rec.attachments {
        if !seen.contains(&a.brain_path) {
            seen.push(a.brain_path.clone());
        }
    }
    seen
}

fn print_health(brain_path: &str) {
    let brain_root = PathBuf::from(brain_path);
    let brain_ok = brain_root.is_dir();
    let mcp_ok = mcp_binary_ok(&brain_root);
    if brain_ok && mcp_ok {
        println!("health:      [OK] brain dir exists, mcp binary exists");
    } else {
        println!(
            "health:      [WARN] brain_dir={}, mcp_binary={}",
            health_mark(brain_ok),
            health_mark(mcp_ok)
        );
    }
}

fn mcp_binary_ok(brain_root: &std::path::Path) -> bool {
    match Brain::load(brain_root) {
        Ok(b) => matches!(b.mcp_server_path(), Ok(p) if p.is_file()),
        Err(_) => false,
    }
}

fn health_mark(ok: bool) -> &'static str {
    if ok {
        "present"
    } else {
        "MISSING"
    }
}
