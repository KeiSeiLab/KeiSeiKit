//! sshd_config parser — read main file + drop-ins, merge with last-wins
//! precedence per OpenSSH rules (main file first, then drop-ins in
//! filename-sort order; first occurrence of a directive wins in sshd,
//! BUT we surface ALL occurrences to report duplicates).

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// A single directive occurrence (name, value, source path, line number).
#[derive(Debug, Clone)]
pub struct Occurrence {
    pub value: String,
    pub source: String, // "<file>:<line>"
}

/// Merged view: directive name (lowercased) → first-occurrence value +
/// every occurrence for duplicate detection.
#[derive(Debug, Default)]
pub struct Merged {
    pub effective: BTreeMap<String, Occurrence>,
    pub all: BTreeMap<String, Vec<Occurrence>>,
}

pub fn load_merged(main: &Path, drop_in: &Path) -> Result<Merged, String> {
    let mut files: Vec<PathBuf> = Vec::new();
    if main.exists() {
        files.push(main.to_path_buf());
    } else {
        return Err(format!("main config not found: {}", main.display()));
    }
    // Drop-in dir is optional; pass empty path to skip.
    if !drop_in.as_os_str().is_empty() && drop_in.is_dir() {
        let mut dropins: Vec<PathBuf> = fs::read_dir(drop_in)
            .map_err(|e| format!("read {}: {e}", drop_in.display()))?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.extension().map(|s| s == "conf").unwrap_or(false))
            .collect();
        dropins.sort();
        files.extend(dropins);
    }

    let mut merged = Merged::default();
    for path in files {
        let body =
            fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        for (lineno, raw) in body.lines().enumerate() {
            if let Some((k, v)) = parse_line(raw) {
                let occ = Occurrence {
                    value: v,
                    source: format!("{}:{}", path.display(), lineno + 1),
                };
                merged
                    .all
                    .entry(k.clone())
                    .or_default()
                    .push(occ.clone());
                // First occurrence wins in OpenSSH — do NOT overwrite.
                merged.effective.entry(k).or_insert(occ);
            }
        }
    }
    Ok(merged)
}

/// Parse one config line. Returns (lowercased_directive, raw_value) or None
/// for comments / blanks / Include (we don't recurse includes by design —
/// the skill wires explicit paths).
fn parse_line(raw: &str) -> Option<(String, String)> {
    let stripped = raw.split('#').next().unwrap_or("").trim();
    if stripped.is_empty() {
        return None;
    }
    let mut parts = stripped.splitn(2, char::is_whitespace);
    let name = parts.next()?.trim().to_ascii_lowercase();
    let value = parts.next().unwrap_or("").trim().to_string();
    if name == "include" || name == "match" {
        return None;
    }
    Some((name, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write(dir: &Path, name: &str, body: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, body).unwrap();
        p
    }

    #[test]
    fn parses_directives_and_ignores_comments() {
        let dir = tempfile::tempdir().unwrap();
        let main = write(dir.path(), "sshd_config", "# header\nPort 22\nPasswordAuthentication no\n");
        let m = load_merged(&main, Path::new("")).unwrap();
        assert_eq!(m.effective["port"].value, "22");
        assert_eq!(m.effective["passwordauthentication"].value, "no");
    }

    #[test]
    fn drop_in_does_not_override_main_effective_value() {
        // OpenSSH: first occurrence wins. Main is read first.
        let dir = tempfile::tempdir().unwrap();
        let main = write(dir.path(), "sshd_config", "Port 22\n");
        let d = dir.path().join("sshd_config.d");
        fs::create_dir(&d).unwrap();
        write(&d, "99-kei.conf", "Port 2222\n");
        let m = load_merged(&main, &d).unwrap();
        assert_eq!(m.effective["port"].value, "22");
        assert_eq!(m.all["port"].len(), 2, "both occurrences recorded");
    }

    #[test]
    fn include_and_match_are_skipped() {
        let dir = tempfile::tempdir().unwrap();
        let main = write(
            dir.path(),
            "sshd_config",
            "Include /etc/ssh/foo.d/*.conf\nMatch User root\n\tPasswordAuthentication yes\n",
        );
        let m = load_merged(&main, Path::new("")).unwrap();
        assert!(!m.effective.contains_key("include"));
        assert!(!m.effective.contains_key("match"));
    }
}
