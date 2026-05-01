//! Render a `Grouped` set of commits as a CHANGELOG.md section.

use crate::group::Grouped;
use chrono::{DateTime, Utc};

/// Options governing the rendered section.
#[derive(Debug, Clone)]
pub struct RenderOpts {
    /// Heading for the version block, e.g. "v0.7.0" or "Unreleased".
    pub version: String,
    /// Optional release date. If `None`, uses today (UTC).
    pub date: Option<DateTime<Utc>>,
    /// If true, include short (7-char) SHA suffix on each line.
    pub include_sha: bool,
}

impl RenderOpts {
    #[must_use]
    pub fn new(version: impl Into<String>) -> Self {
        Self {
            version: version.into(),
            date: None,
            include_sha: true,
        }
    }
}

fn fmt_line(subj: &str, scope: Option<&str>, sha: Option<&str>) -> String {
    let mut s = String::new();
    s.push_str("- ");
    if let Some(sc) = scope {
        s.push_str("**");
        s.push_str(sc);
        s.push_str(":** ");
    }
    s.push_str(subj);
    if let Some(h) = sha {
        s.push_str(&format!(" (`{}`)", &h[..h.len().min(7)]));
    }
    s
}

/// Render the grouped commits into markdown. Returns an empty string if the
/// grouping has no entries (caller can detect via `Grouped::is_empty`).
#[must_use]
pub fn render_markdown(grouped: &Grouped, opts: &RenderOpts) -> String {
    if grouped.is_empty() {
        return String::new();
    }
    let date = opts.date.unwrap_or_else(Utc::now).format("%Y-%m-%d");
    let mut out = String::new();
    out.push_str(&format!("## {} — {date}\n\n", opts.version));

    if !grouped.breaking.is_empty() {
        out.push_str("### BREAKING CHANGES\n\n");
        for c in &grouped.breaking {
            let sha = if opts.include_sha { Some(c.sha.as_str()) } else { None };
            out.push_str(&fmt_line(&c.subject, c.scope.as_deref(), sha));
            out.push('\n');
        }
        out.push('\n');
    }

    for (_, (kind, commits)) in &grouped.by_kind {
        out.push_str(&format!("### {}\n\n", kind.heading()));
        for c in commits {
            let sha = if opts.include_sha { Some(c.sha.as_str()) } else { None };
            out.push_str(&fmt_line(&c.subject, c.scope.as_deref(), sha));
            out.push('\n');
        }
        out.push('\n');
    }
    out
}
