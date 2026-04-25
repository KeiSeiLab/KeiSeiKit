//! JSON → Tokens model. Flat string-keyed maps per category (colors, fonts,
//! spacing, radius). Unknown categories are ignored; missing categories
//! default to empty.

use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Default)]
pub struct Tokens {
    #[serde(default)]
    pub colors: BTreeMap<String, String>,
    #[serde(default)]
    pub fonts: BTreeMap<String, String>,
    #[serde(default)]
    pub spacing: BTreeMap<String, String>,
    #[serde(default)]
    pub radius: BTreeMap<String, String>,
}

pub fn load(path: &Path) -> Result<Tokens, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_json::from_str(&text).map_err(|e| format!("parse {}: {e}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn parses_full_shape() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(
            f,
            r#"{{
              "colors":  {{ "primary": "oklch(0.6 0.2 250)", "surface": "oklch(0.995 0 0)" }},
              "fonts":   {{ "display": "Fraunces, serif", "body": "Inter, sans-serif" }},
              "spacing": {{ "sm": "0.5rem", "md": "1rem" }},
              "radius":  {{ "card": "0.75rem" }}
            }}"#
        )
        .unwrap();
        let tokens = load(f.path()).unwrap();
        assert_eq!(tokens.colors.len(), 2);
        assert_eq!(tokens.colors.get("primary").unwrap(), "oklch(0.6 0.2 250)");
        assert_eq!(tokens.fonts.get("body").unwrap(), "Inter, sans-serif");
        assert_eq!(tokens.spacing.len(), 2);
        assert_eq!(tokens.radius.len(), 1);
    }

    #[test]
    fn missing_categories_default_empty() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        let payload = r##"{ "colors": { "primary": "#000" } }"##;
        f.write_all(payload.as_bytes()).unwrap();
        let tokens = load(f.path()).unwrap();
        assert_eq!(tokens.colors.len(), 1);
        assert!(tokens.fonts.is_empty());
        assert!(tokens.spacing.is_empty());
        assert!(tokens.radius.is_empty());
    }

    #[test]
    fn invalid_json_errors() {
        let mut f = tempfile::NamedTempFile::new().unwrap();
        writeln!(f, "not json").unwrap();
        assert!(load(f.path()).is_err());
    }
}
