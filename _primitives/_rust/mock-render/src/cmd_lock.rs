//! `mock-render lock --project <dir> --section <src> [--screenshot <png>]`
//!
//! Extracted from `main.rs` in v0.14.1 per Constructor Pattern.

use crate::cli_args::{flag, require_project_section};
use crate::hash;
use crate::state::{Section, SiteState};
use std::process::ExitCode;

pub fn run(args: &[String]) -> ExitCode {
    let (project, section) = match require_project_section(args) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("lock: {e}");
            return ExitCode::from(1);
        }
    };
    let screenshot = flag(args, "--screenshot");

    let Ok(hash_now) = hash::hash_file(&section) else {
        eprintln!("lock: cannot hash {}", section.display());
        return ExitCode::from(2);
    };

    let mut st = match SiteState::load(&project) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("lock: {e}");
            return ExitCode::from(2);
        }
    };

    let key = SiteState::key_for(&section);
    st.sections.insert(
        key.clone(),
        Section {
            path: section.display().to_string(),
            sha256: hash_now.clone(),
            locked: true,
            screenshot: screenshot.map(String::from),
        },
    );

    if let Err(e) = st.save(&project) {
        eprintln!("lock: {e}");
        return ExitCode::from(2);
    }

    println!("locked {key} ({})", &hash_now[..12]);
    ExitCode::SUCCESS
}
