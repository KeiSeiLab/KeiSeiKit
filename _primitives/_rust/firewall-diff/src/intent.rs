//! Intent YAML schema + loader. See `_blocks/security-firewall-ufw.md` for
//! the reference format. Anything missing is treated as "don't care".

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Allow,
    Deny,
    Limit,
    Reject,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Defaults {
    #[serde(default = "default_deny")]
    pub incoming: Action,
    #[serde(default = "default_allow")]
    pub outgoing: Action,
    #[serde(default = "default_deny")]
    pub routed: Action,
}
fn default_deny() -> Action {
    Action::Deny
}
fn default_allow() -> Action {
    Action::Allow
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Rule {
    pub port: u16,
    #[serde(default = "default_proto")]
    pub proto: String,
    pub action: Action,
    #[serde(default = "default_from")]
    pub from: String,
    #[serde(default)]
    pub comment: String,
}
fn default_proto() -> String {
    "tcp".into()
}
fn default_from() -> String {
    "any".into()
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct Intent {
    pub default: Defaults,
    #[serde(default)]
    pub rules: Vec<Rule>,
}

pub fn load(path: &Path) -> Result<Intent, String> {
    let body = fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
    serde_yaml::from_str(&body).map_err(|e| format!("yaml: {e}"))
}

impl Rule {
    /// Canonical key used to match against a live rule: port/proto/from/action.
    pub fn key(&self) -> String {
        format!(
            "{}/{}::{}::{}",
            self.port,
            self.proto.to_ascii_lowercase(),
            self.from.to_ascii_lowercase(),
            format!("{:?}", self.action).to_ascii_lowercase()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn load_minimal_intent() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("intent.yaml");
        let mut f = fs::File::create(&p).unwrap();
        writeln!(
            f,
            r#"default:
  incoming: deny
  outgoing: allow
  routed: deny
rules:
  - port: 22
    proto: tcp
    action: limit
    from: any
    comment: "ssh"
  - port: 443
    proto: tcp
    action: allow
    from: any
"#
        )
        .unwrap();
        let i = load(&p).unwrap();
        assert_eq!(i.default.incoming, Action::Deny);
        assert_eq!(i.rules.len(), 2);
        assert_eq!(i.rules[0].action, Action::Limit);
        assert_eq!(i.rules[1].port, 443);
    }
}
