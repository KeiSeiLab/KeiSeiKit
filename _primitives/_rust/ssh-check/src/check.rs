//! Evaluate the hardened rule matrix against a merged sshd_config view.

use crate::parse::Merged;
use crate::rules::{Expect, Rule};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Severity {
    Ok,
    Warn,
    Fail,
}

impl Severity {
    pub fn label(&self) -> &'static str {
        match self {
            Severity::Ok => "OK",
            Severity::Warn => "WARN",
            Severity::Fail => "FAIL",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub directive: String,
    pub severity: Severity,
    pub source: String,
    pub note: String,
}

pub fn evaluate(merged: &Merged, rules: &[Rule]) -> Vec<Finding> {
    let mut out = Vec::with_capacity(rules.len());
    for r in rules {
        out.push(eval_rule(merged, r));
    }
    out
}

fn eval_rule(merged: &Merged, r: &Rule) -> Finding {
    let occ = merged.effective.get(r.directive);
    match (occ, r.required) {
        (None, true) => Finding {
            directive: r.directive.into(),
            severity: Severity::Fail,
            source: "(missing)".into(),
            note: format!("required directive absent — {}", r.rationale),
        },
        (None, false) => Finding {
            directive: r.directive.into(),
            severity: Severity::Warn,
            source: "(missing)".into(),
            note: format!("recommended: {}", r.rationale),
        },
        (Some(o), _) => {
            let ok = value_matches(&o.value, &r.expect);
            Finding {
                directive: r.directive.into(),
                severity: if ok { Severity::Ok } else { Severity::Fail },
                source: o.source.clone(),
                note: if ok {
                    "ok".into()
                } else {
                    format!("value '{}' violates policy — {}", o.value, r.rationale)
                },
            }
        }
    }
}

fn value_matches(value: &str, expect: &Expect) -> bool {
    let v = value.trim().to_ascii_lowercase();
    match expect {
        Expect::Equals(target) => v == target.to_ascii_lowercase(),
        Expect::OneOf(list) => list.iter().any(|s| v == s.to_ascii_lowercase()),
        Expect::MaxInt(max) => v.parse::<u32>().map(|n| n <= *max).unwrap_or(false),
        Expect::ContainsAll(tokens) => tokens.iter().all(|t| v.contains(&t.to_ascii_lowercase())),
        Expect::DeniesAny(tokens) => {
            let parts: Vec<&str> = v.split(',').map(str::trim).collect();
            !tokens
                .iter()
                .any(|t| parts.iter().any(|p| p == &t.to_ascii_lowercase()))
        }
        Expect::AllowedUsersSubset(allow) => {
            let parts: Vec<String> = v
                .split_whitespace()
                .map(|s| s.to_string())
                .collect();
            !parts.is_empty() && parts.iter().all(|u| allow.contains(u))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{Merged, Occurrence};
    use crate::rules::hardened_matrix;
    use std::collections::BTreeMap;

    fn merged(pairs: &[(&str, &str)]) -> Merged {
        let mut m = Merged {
            effective: BTreeMap::new(),
            all: BTreeMap::new(),
        };
        for (k, v) in pairs {
            let occ = Occurrence {
                value: (*v).to_string(),
                source: "test:1".into(),
            };
            m.effective.insert((*k).to_string(), occ.clone());
            m.all.insert((*k).to_string(), vec![occ]);
        }
        m
    }

    #[test]
    fn hardened_baseline_passes() {
        let rules = hardened_matrix(&["keiadmin".into()]);
        let mg = merged(&[
            ("passwordauthentication", "no"),
            ("permitrootlogin", "prohibit-password"),
            ("maxauthtries", "3"),
            ("allowusers", "keiadmin"),
            ("ciphers", "chacha20-poly1305@openssh.com,aes256-gcm@openssh.com"),
            ("macs", "hmac-sha2-512-etm@openssh.com"),
            ("hostkeyalgorithms", "ssh-ed25519,rsa-sha2-512"),
        ]);
        let findings = evaluate(&mg, &rules);
        let fails: Vec<_> = findings.iter().filter(|f| f.severity == Severity::Fail).collect();
        assert!(fails.is_empty(), "unexpected fails: {fails:#?}");
    }

    #[test]
    fn password_auth_yes_fails() {
        let rules = hardened_matrix(&["keiadmin".into()]);
        let mg = merged(&[
            ("passwordauthentication", "yes"),
            ("permitrootlogin", "no"),
            ("maxauthtries", "3"),
            ("allowusers", "keiadmin"),
        ]);
        let findings = evaluate(&mg, &rules);
        let f = findings
            .iter()
            .find(|f| f.directive == "passwordauthentication")
            .unwrap();
        assert_eq!(f.severity, Severity::Fail);
    }

    #[test]
    fn cbc_cipher_fails() {
        let rules = hardened_matrix(&["keiadmin".into()]);
        let mg = merged(&[
            ("passwordauthentication", "no"),
            ("permitrootlogin", "no"),
            ("maxauthtries", "3"),
            ("allowusers", "keiadmin"),
            ("ciphers", "aes256-cbc,chacha20-poly1305@openssh.com"),
        ]);
        let findings = evaluate(&mg, &rules);
        let f = findings.iter().find(|f| f.directive == "ciphers").unwrap();
        assert_eq!(f.severity, Severity::Fail);
    }

    #[test]
    fn allow_users_not_in_whitelist_fails() {
        let rules = hardened_matrix(&["keiadmin".into()]);
        let mg = merged(&[
            ("passwordauthentication", "no"),
            ("permitrootlogin", "no"),
            ("maxauthtries", "3"),
            ("allowusers", "root attacker"),
        ]);
        let findings = evaluate(&mg, &rules);
        let f = findings.iter().find(|f| f.directive == "allowusers").unwrap();
        assert_eq!(f.severity, Severity::Fail);
    }

    #[test]
    fn missing_required_directive_fails() {
        let rules = hardened_matrix(&["keiadmin".into()]);
        let mg = merged(&[
            ("permitrootlogin", "no"),
            ("maxauthtries", "3"),
            ("allowusers", "keiadmin"),
        ]);
        let findings = evaluate(&mg, &rules);
        let f = findings
            .iter()
            .find(|f| f.directive == "passwordauthentication")
            .unwrap();
        assert_eq!(f.severity, Severity::Fail);
        assert_eq!(f.source, "(missing)");
    }

    #[test]
    fn maxauthtries_too_high_fails() {
        let rules = hardened_matrix(&["keiadmin".into()]);
        let mg = merged(&[
            ("passwordauthentication", "no"),
            ("permitrootlogin", "no"),
            ("maxauthtries", "10"),
            ("allowusers", "keiadmin"),
        ]);
        let findings = evaluate(&mg, &rules);
        let f = findings
            .iter()
            .find(|f| f.directive == "maxauthtries")
            .unwrap();
        assert_eq!(f.severity, Severity::Fail);
    }
}
