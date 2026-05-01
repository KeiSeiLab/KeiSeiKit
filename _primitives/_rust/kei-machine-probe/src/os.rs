//! OS detection.
//!
//! Primary path: `sw_vers -productVersion` + `sw_vers -buildVersion` on
//! macOS. If `sw_vers` fails (Linux / other), fall back to
//! `uname -sr` to recover at least family and version. Anything we
//! can't classify reports `OsFamily::Other` rather than panicking.

use crate::profile::{OsFamily, OsInfo};
use crate::runner::Runner;

pub fn detect_os(runner: &dyn Runner) -> OsInfo {
    if let Some(info) = try_detect_macos(runner) {
        return info;
    }
    if let Some(info) = try_detect_unix(runner) {
        return info;
    }
    OsInfo {
        family: OsFamily::Other,
        version: String::new(),
        build: String::new(),
    }
}

fn try_detect_macos(runner: &dyn Runner) -> Option<OsInfo> {
    let version = runner
        .run("sw_vers", &["-productVersion"])
        .ok()?
        .trim()
        .to_string();
    if version.is_empty() {
        return None;
    }
    let build = runner
        .run("sw_vers", &["-buildVersion"])
        .map(|s| s.trim().to_string())
        .unwrap_or_default();
    Some(OsInfo {
        family: OsFamily::Macos,
        version,
        build,
    })
}

fn try_detect_unix(runner: &dyn Runner) -> Option<OsInfo> {
    let raw = runner.run("uname", &["-sr"]).ok()?;
    let trimmed = raw.trim();
    let mut parts = trimmed.splitn(2, ' ');
    let kernel = parts.next().unwrap_or("").to_string();
    let version = parts.next().unwrap_or("").to_string();
    let family = match kernel.as_str() {
        "Linux" => OsFamily::Linux,
        "Darwin" => OsFamily::Macos,
        "" => return None,
        _ => OsFamily::Other,
    };
    Some(OsInfo {
        family,
        version,
        build: String::new(),
    })
}
