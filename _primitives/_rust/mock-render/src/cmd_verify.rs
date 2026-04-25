//! `mock-render verify --project <dir> --section <src>`
//! `mock-render status --project <dir>`
//!
//! Two closely-related subcommands extracted from `main.rs` in v0.14.1.
//! They share state-loading + hash-comparison logic.

use crate::cli_args::{flag, require_project_section};
use crate::hash;
use crate::state::SiteState;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

pub fn run_verify(args: &[String]) -> ExitCode {
    let (project, section) = match require_project_section(args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("verify: {e}");
            return ExitCode::from(1);
        }
    };

    let st = match SiteState::load(&project) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("verify: {e}");
            return ExitCode::from(2);
        }
    };

    let key = SiteState::key_for(&section);
    let Some(entry) = st.sections.get(&key) else {
        eprintln!("verify: section '{key}' not in site-state.json (not locked yet)");
        return ExitCode::SUCCESS;
    };
    if !entry.locked {
        return ExitCode::SUCCESS;
    }

    let Ok(hash_now) = hash::hash_file(&section) else {
        eprintln!("verify: cannot hash {}", section.display());
        return ExitCode::from(2);
    };

    if hash_now != entry.sha256 {
        eprintln!(
            "WYSIWYD VIOLATION: {key} drifted\n  locked : {}\n  current: {}\nThe screenshot user approved no longer matches the source.\nRerun render + user-approval before deploy.",
            &entry.sha256[..12],
            &hash_now[..12]
        );
        return ExitCode::from(2);
    }
    println!("ok {key} ({})", &hash_now[..12]);
    ExitCode::SUCCESS
}

pub fn run_status(args: &[String]) -> ExitCode {
    let project = flag(args, "--project")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let st = match SiteState::load(&project) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("status: {e}");
            return ExitCode::from(2);
        }
    };

    if st.sections.is_empty() {
        println!("(no sections tracked)");
        return ExitCode::SUCCESS;
    }

    for (name, sec) in &st.sections {
        let lock = if sec.locked { "LOCKED" } else { "open" };
        let drift = match hash::hash_file(Path::new(&sec.path)) {
            Ok(h) if h == sec.sha256 => "clean",
            Ok(_) => "DRIFT",
            Err(_) => "missing",
        };
        println!(
            "{:<20} {:>6}  {:<7}  {} ({})",
            name,
            lock,
            drift,
            sec.path,
            &sec.sha256[..12]
        );
    }
    ExitCode::SUCCESS
}
