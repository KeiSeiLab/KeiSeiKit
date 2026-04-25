//! Parse `ufw status numbered` output.
//!
//! Typical shape (Ubuntu 22.04, ufw 0.36):
//!
//!     Status: active
//!
//!          To                         Action      From
//!          --                         ------      ----
//!     [ 1] 22/tcp                     LIMIT IN    Anywhere
//!     [ 2] 443/tcp                    ALLOW IN    Anywhere
//!     [ 3] 22/tcp (v6)                LIMIT IN    Anywhere (v6)
//!
//! We normalise "(v6)" to a separate family tag but key rules on port/proto
//! only (v6 and v4 rules with the same port/proto are treated as duplicates
//! of intent, which is usually the desired behaviour for parity checks).

use crate::intent::Action;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LiveRule {
    pub port: u16,
    pub proto: String,
    pub action: Action,
    pub from: String,
    pub family: Family,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum Family {
    V4,
    V6,
}

#[derive(Debug, Clone, Serialize)]
pub struct Live {
    pub active: bool,
    pub rules: Vec<LiveRule>,
}

pub fn parse(text: &str) -> Result<Live, String> {
    let mut active = false;
    let mut rules = Vec::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(rest) = line.strip_prefix("Status:") {
            active = rest.trim().eq_ignore_ascii_case("active");
            continue;
        }
        if line.starts_with("To") || line.starts_with("--") {
            continue;
        }
        if let Some(r) = parse_rule(line) {
            rules.push(r);
        }
    }
    if text.trim().is_empty() {
        return Err("could not detect an `ufw status` block (empty input)".into());
    }
    Ok(Live { active, rules })
}

/// Parse one numbered rule line. Returns None if the line is not a rule.
fn parse_rule(line: &str) -> Option<LiveRule> {
    // Strip leading "[ N]" if present.
    let body = if let Some(idx) = line.find(']') {
        line[idx + 1..].trim()
    } else {
        line
    };
    // Columns: <to> <ACTION IN|OUT|FWD> <from>
    // We split on 2+ whitespace runs which ufw pads with.
    let parts: Vec<&str> = body.split("  ").filter(|s| !s.is_empty()).map(str::trim).collect();
    if parts.len() < 3 {
        return None;
    }
    let to = parts[0];
    let action_raw = parts[1];
    let from = parts[2];

    let (to_clean, family) = if to.contains("(v6)") {
        (to.replace("(v6)", "").trim().to_string(), Family::V6)
    } else {
        (to.to_string(), Family::V4)
    };

    let (port, proto) = split_port_proto(&to_clean)?;
    let action = parse_action(action_raw)?;
    Some(LiveRule {
        port,
        proto,
        action,
        from: from.replace("(v6)", "").trim().to_string(),
        family,
    })
}

fn split_port_proto(tok: &str) -> Option<(u16, String)> {
    // "22/tcp" | "53" | "443/udp"
    if let Some((port_s, proto_s)) = tok.split_once('/') {
        Some((port_s.parse().ok()?, proto_s.to_ascii_lowercase()))
    } else {
        Some((tok.parse().ok()?, "tcp".into()))
    }
}

fn parse_action(raw: &str) -> Option<Action> {
    let up = raw.to_ascii_uppercase();
    if up.starts_with("ALLOW") {
        Some(Action::Allow)
    } else if up.starts_with("DENY") {
        Some(Action::Deny)
    } else if up.starts_with("LIMIT") {
        Some(Action::Limit)
    } else if up.starts_with("REJECT") {
        Some(Action::Reject)
    } else {
        None
    }
}

impl LiveRule {
    pub fn key(&self) -> String {
        let from = if self.from.eq_ignore_ascii_case("Anywhere") {
            "any"
        } else {
            &self.from
        };
        format!(
            "{}/{}::{}::{}",
            self.port,
            self.proto,
            from.to_ascii_lowercase(),
            format!("{:?}", self.action).to_ascii_lowercase()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
Status: active

     To                         Action      From
     --                         ------      ----
[ 1] 22/tcp                     LIMIT IN    Anywhere
[ 2] 443/tcp                    ALLOW IN    Anywhere
[ 3] 22/tcp (v6)                LIMIT IN    Anywhere (v6)
"#;

    #[test]
    fn parses_active_and_rules() {
        let l = parse(SAMPLE).unwrap();
        assert!(l.active);
        assert_eq!(l.rules.len(), 3);
        assert_eq!(l.rules[0].port, 22);
        assert_eq!(l.rules[0].proto, "tcp");
        assert_eq!(l.rules[0].action, Action::Limit);
        assert_eq!(l.rules[2].family, Family::V6);
    }

    #[test]
    fn inactive_status_rejects_only_if_no_rules() {
        let l = parse("Status: inactive\n").unwrap();
        assert!(!l.active);
        assert!(l.rules.is_empty());
    }
}
