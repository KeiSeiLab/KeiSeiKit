//! Playwright subprocess wrapper (RULE 0.2 exception 6 — JS-only binding).
//! Calls `npx playwright screenshot` with clear error messages.
//!
//! Requires Node + `npx`. Playwright browsers installable via:
//!   npx playwright install chromium

use std::path::Path;
use std::process::Command;

/// Render a URL (typically http://localhost:<port>/<page>) or a file:// URL
/// to a PNG via Playwright's CLI.
pub fn screenshot(url: &str, out: &Path, viewport: Option<(u32, u32)>) -> Result<(), String> {
    let mut args = vec![
        "--yes".to_string(),
        "playwright".to_string(),
        "screenshot".to_string(),
        "--full-page".to_string(),
    ];

    if let Some((w, h)) = viewport {
        args.push("--viewport-size".to_string());
        args.push(format!("{w},{h}"));
    }

    args.push(url.to_string());
    args.push(out.display().to_string());

    let output = Command::new("npx")
        .args(&args)
        .output()
        .map_err(|e| format!("npx spawn: {e} — is Node installed?"))?;

    if !output.status.success() {
        return Err(format!(
            "playwright screenshot failed (exit {}):\n{}",
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    if !out.exists() {
        return Err(format!(
            "playwright claimed success but {} was not written",
            out.display()
        ));
    }

    Ok(())
}
