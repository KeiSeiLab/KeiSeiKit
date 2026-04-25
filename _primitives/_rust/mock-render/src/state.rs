//! site-state.json — single file that tracks which sections are locked
//! and what their approved SHA-256 is. One row per section.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_STATE: &str = "site-state.json";

#[derive(Serialize, Deserialize, Default)]
pub struct SiteState {
    pub sections: BTreeMap<String, Section>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Section {
    pub path: String,
    pub sha256: String,
    pub locked: bool,
    pub screenshot: Option<String>,
}

impl SiteState {
    pub fn load(project: &Path) -> Result<Self, String> {
        let p = Self::path(project);
        if !p.exists() {
            return Ok(Self::default());
        }
        let text = fs::read_to_string(&p).map_err(|e| format!("read state: {e}"))?;
        serde_json::from_str(&text).map_err(|e| format!("parse state: {e}"))
    }

    pub fn save(&self, project: &Path) -> Result<(), String> {
        let p = Self::path(project);
        let text = serde_json::to_string_pretty(self).map_err(|e| format!("serialize: {e}"))?;
        fs::write(&p, text).map_err(|e| format!("write state: {e}"))
    }

    fn path(project: &Path) -> PathBuf {
        project.join(DEFAULT_STATE)
    }

    pub fn key_for(section_path: &Path) -> String {
        section_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_uses_file_stem() {
        let k = SiteState::key_for(Path::new("src/sections/Hero.astro"));
        assert_eq!(k, "Hero");
    }

    #[test]
    fn load_missing_returns_default() {
        let tmp = tempfile::tempdir().unwrap();
        let st = SiteState::load(tmp.path()).unwrap();
        assert!(st.sections.is_empty());
    }

    #[test]
    fn save_and_reload_roundtrips() {
        let tmp = tempfile::tempdir().unwrap();
        let mut st = SiteState::default();
        st.sections.insert(
            "Hero".into(),
            Section {
                path: "src/sections/Hero.astro".into(),
                sha256: "abcdef".repeat(10),
                locked: true,
                screenshot: Some("mocks/Hero.png".into()),
            },
        );
        st.save(tmp.path()).unwrap();
        let reloaded = SiteState::load(tmp.path()).unwrap();
        assert_eq!(reloaded.sections.len(), 1);
        assert!(reloaded.sections.get("Hero").unwrap().locked);
    }
}
