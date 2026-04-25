//! git2 walker — collect commits between two refs.

use crate::commit::Commit;
use crate::parse::parse_subject;
use anyhow::{Context, Result};
use git2::{Oid, Repository, Sort};

/// Range specification passed in from CLI.
#[derive(Debug, Clone)]
pub struct WalkRange {
    pub from: Option<String>,
    pub to: String,
}

fn resolve(repo: &Repository, name: &str) -> Result<Oid> {
    let obj = repo
        .revparse_single(name)
        .with_context(|| format!("cannot resolve ref: {name}"))?;
    Ok(obj.id())
}

/// Walk commits in topological order (newest first) from `to` back to `from`.
/// If `from` is `None`, walks the full history reachable from `to`.
pub fn walk_range(repo_path: &std::path::Path, range: &WalkRange) -> Result<Vec<Commit>> {
    let repo = Repository::discover(repo_path)
        .with_context(|| format!("not a git repo: {}", repo_path.display()))?;
    let to_oid = resolve(&repo, &range.to)?;
    let from_oid = match &range.from {
        Some(name) => Some(resolve(&repo, name)?),
        None => None,
    };

    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::TOPOLOGICAL)?;
    revwalk.push(to_oid)?;
    if let Some(f) = from_oid {
        revwalk.hide(f)?;
    }

    let mut out: Vec<Commit> = Vec::new();
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let first = commit.summary().unwrap_or("").to_string();
        let body = commit.body().unwrap_or("");
        let (kind, scope, subject, breaking_bang) = parse_subject(&first);
        let breaking = breaking_bang || body.contains("BREAKING CHANGE");
        out.push(Commit {
            sha: oid.to_string(),
            kind,
            scope,
            subject,
            breaking,
        });
    }
    Ok(out)
}
