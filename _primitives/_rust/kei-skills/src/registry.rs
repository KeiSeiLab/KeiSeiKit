//! Name-keyed in-memory skill store with optional hot-reload.
//!
//! Constructor Pattern: registry owns the `HashMap`, the optional
//! `notify` watcher, and a thread-safe `RwLock` for read-mostly access.
//!
//! Hot-reload semantics: when the watcher fires for any path under the
//! root directory, the registry re-runs `load_all` and atomically swaps
//! the inner map. Brief readers see either the old set or the new set —
//! no torn reads, no half-loaded skills.

use crate::format::Skill;
use crate::loader::{load_all, loaded_only};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};

/// Public registry handle. Cloneable — the inner state is `Arc`-shared.
#[derive(Clone)]
pub struct SkillRegistry {
    root: PathBuf,
    skills: Arc<RwLock<HashMap<String, Skill>>>,
    /// Held to keep the watcher alive. `None` until `enable_hot_reload`.
    _watcher: Arc<RwLock<Option<RecommendedWatcher>>>,
}

impl SkillRegistry {
    /// Build a registry by walking `root` once. No watcher is started;
    /// call [`enable_hot_reload`] to wire one up.
    pub fn new(root: &Path) -> Self {
        let initial = loaded_only(load_all(root));
        let map = build_map(initial);
        SkillRegistry {
            root: root.to_path_buf(),
            skills: Arc::new(RwLock::new(map)),
            _watcher: Arc::new(RwLock::new(None)),
        }
    }

    /// Look up a skill by name. Returns `None` if absent.
    pub fn get(&self, name: &str) -> Option<Skill> {
        self.skills.read().ok().and_then(|m| m.get(name).cloned())
    }

    /// Snapshot the registry. O(N) clone — callers that iterate often
    /// should hold the result rather than re-call.
    pub fn list(&self) -> Vec<Skill> {
        self.skills.read().map(|m| m.values().cloned().collect()).unwrap_or_default()
    }

    /// Filter snapshot by `category` field. Empty result when no skill
    /// has that category.
    pub fn list_by_category(&self, category: &str) -> Vec<Skill> {
        self.list()
            .into_iter()
            .filter(|s| s.frontmatter.category.as_deref() == Some(category))
            .collect()
    }

    /// Force a re-scan from disk. Atomic swap — readers never observe
    /// a partially-loaded state.
    pub fn reload(&self) {
        let fresh = loaded_only(load_all(&self.root));
        if let Ok(mut guard) = self.skills.write() {
            *guard = build_map(fresh);
        }
    }

    /// Start a notify watcher that calls `reload` on any FS event under
    /// the root. Returns `Err(notify::Error)` if the platform watcher
    /// cannot be created. Subsequent calls are idempotent — replaces any
    /// prior watcher.
    pub fn enable_hot_reload(&self) -> notify::Result<()> {
        let me = self.clone();
        let handler = move |res: notify::Result<notify::Event>| {
            if res.is_ok() {
                me.reload();
            }
        };
        let mut w: RecommendedWatcher = notify::recommended_watcher(handler)?;
        w.watch(&self.root, RecursiveMode::Recursive)?;
        if let Ok(mut slot) = self._watcher.write() {
            *slot = Some(w);
        }
        Ok(())
    }
}

fn build_map(skills: Vec<Skill>) -> HashMap<String, Skill> {
    let mut m = HashMap::with_capacity(skills.len());
    for s in skills {
        // Last write wins on duplicate names; loader walk order is filesystem-dependent.
        m.insert(s.frontmatter.name.clone(), s);
    }
    m
}
