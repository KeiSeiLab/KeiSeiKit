use chrono::{TimeZone, Utc};
use kei_changelog::{render_markdown, Commit, CommitKind, Grouped, RenderOpts};

fn mk(kind: CommitKind, scope: Option<&str>, subject: &str, breaking: bool, sha: &str) -> Commit {
    Commit {
        sha: sha.to_string(),
        kind,
        scope: scope.map(str::to_string),
        subject: subject.to_string(),
        breaking,
    }
}

#[test]
fn renders_feat_and_fix_sections() {
    let commits = vec![
        mk(CommitKind::Feat, Some("blocks"), "5 blocks", false, "abcdef1234"),
        mk(CommitKind::Fix, None, "regex bug", false, "1234567890"),
    ];
    let grouped = Grouped::from_commits(&commits);
    let mut opts = RenderOpts::new("v0.1.0");
    opts.date = Some(Utc.with_ymd_and_hms(2026, 4, 21, 0, 0, 0).unwrap());
    let out = render_markdown(&grouped, &opts);
    assert!(out.starts_with("## v0.1.0 — 2026-04-21\n"));
    assert!(out.contains("### Features"));
    assert!(out.contains("### Fixes"));
    assert!(out.contains("**blocks:** 5 blocks (`abcdef1`)"));
    assert!(out.contains("- regex bug (`1234567`)"));
}

#[test]
fn breaking_section_comes_first() {
    let commits = vec![
        mk(CommitKind::Feat, None, "non-breaking", false, "aaaaaaa"),
        mk(CommitKind::Feat, Some("api"), "rename", true, "bbbbbbb"),
    ];
    let grouped = Grouped::from_commits(&commits);
    let out = render_markdown(&grouped, &RenderOpts::new("v1.0.0"));
    let bi = out.find("BREAKING CHANGES").expect("section present");
    let fi = out.find("### Features").expect("features present");
    assert!(bi < fi, "BREAKING must come before Features");
}

#[test]
fn empty_grouped_renders_empty() {
    let grouped = Grouped::from_commits(&[]);
    let out = render_markdown(&grouped, &RenderOpts::new("v0.0.0"));
    assert!(out.is_empty());
}

#[test]
fn include_sha_false_hides_hash() {
    let commits = vec![mk(CommitKind::Feat, None, "x", false, "deadbeef0")];
    let grouped = Grouped::from_commits(&commits);
    let mut opts = RenderOpts::new("v0.1.0");
    opts.include_sha = false;
    let out = render_markdown(&grouped, &opts);
    assert!(!out.contains("deadbee"));
    assert!(out.contains("- x"));
}
