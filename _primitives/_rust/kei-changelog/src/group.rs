//! Group commits by kind, preserving insertion order within each bucket.

use crate::commit::{Commit, CommitKind};
use std::collections::BTreeMap;

/// Commits grouped by `CommitKind`, sorted by `CommitKind::sort_key`.
#[derive(Debug, Default)]
pub struct Grouped {
    pub by_kind: BTreeMap<u8, (CommitKind, Vec<Commit>)>,
    pub breaking: Vec<Commit>,
}

impl Grouped {
    /// Build a `Grouped` from an ordered slice of commits.
    ///
    /// Breaking commits are additionally copied into `breaking` so renderers
    /// can surface them in a "BREAKING CHANGES" section.
    #[must_use]
    pub fn from_commits(commits: &[Commit]) -> Self {
        let mut g = Self::default();
        for c in commits {
            if c.breaking {
                g.breaking.push(c.clone());
            }
            let key = c.kind.sort_key();
            g.by_kind
                .entry(key)
                .or_insert_with(|| (c.kind.clone(), Vec::new()))
                .1
                .push(c.clone());
        }
        g
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_kind.is_empty() && self.breaking.is_empty()
    }
}
