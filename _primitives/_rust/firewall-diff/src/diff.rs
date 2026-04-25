//! Compare Intent × Live and emit a structured report.

use crate::intent::{Action, Intent, Rule};
use crate::ufw::{Live, LiveRule};
use serde::Serialize;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize)]
pub struct Report {
    pub active_ok: bool,
    pub default_mismatches: Vec<String>,
    pub missing: Vec<Rule>,   // in intent, not in live
    pub extra: Vec<LiveRule>, // in live, not in intent
}

impl Report {
    pub fn is_clean(&self) -> bool {
        self.active_ok
            && self.default_mismatches.is_empty()
            && self.missing.is_empty()
            && self.extra.is_empty()
    }
}

pub fn compare(intent: &Intent, live: &Live) -> Report {
    let active_ok = live.active;

    let mut default_mismatches = Vec::new();
    if !matches!(intent.default.incoming, Action::Deny | Action::Reject) {
        default_mismatches
            .push("intent.default.incoming must be deny/reject for production".to_string());
    }

    // Build key sets.
    let intent_keys: HashSet<String> = intent.rules.iter().map(Rule::key).collect();
    let live_keys: HashSet<String> = live.rules.iter().map(LiveRule::key).collect();

    let missing: Vec<Rule> = intent
        .rules
        .iter()
        .filter(|r| !live_keys.contains(&r.key()))
        .cloned()
        .collect();
    let extra: Vec<LiveRule> = live
        .rules
        .iter()
        .filter(|r| !intent_keys.contains(&r.key()))
        .cloned()
        .collect();

    Report {
        active_ok,
        default_mismatches,
        missing,
        extra,
    }
}

pub fn render_human(r: &Report) {
    if !r.active_ok {
        println!("[FAIL] ufw is not active.");
    }
    for m in &r.default_mismatches {
        println!("[WARN] default: {m}");
    }
    for m in &r.missing {
        println!(
            "[MISS] intent rule not live: {}/{} from={} action={:?}",
            m.port, m.proto, m.from, m.action
        );
    }
    for e in &r.extra {
        println!(
            "[EXTRA] live rule not in intent: {}/{} from={} action={:?} family={:?}",
            e.port, e.proto, e.from, e.action, e.family
        );
    }
    if r.is_clean() {
        println!("firewall-diff: OK — intent ≡ live.");
    } else {
        println!(
            "firewall-diff: {} missing, {} extra, default-issues={}",
            r.missing.len(),
            r.extra.len(),
            r.default_mismatches.len()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{Action, Defaults, Intent, Rule};
    use crate::ufw::{self, Family, Live, LiveRule};

    fn intent_fx() -> Intent {
        Intent {
            default: Defaults {
                incoming: Action::Deny,
                outgoing: Action::Allow,
                routed: Action::Deny,
            },
            rules: vec![
                Rule {
                    port: 22,
                    proto: "tcp".into(),
                    action: Action::Limit,
                    from: "any".into(),
                    comment: "ssh".into(),
                },
                Rule {
                    port: 443,
                    proto: "tcp".into(),
                    action: Action::Allow,
                    from: "any".into(),
                    comment: "".into(),
                },
            ],
        }
    }

    fn live_fx(items: &[(u16, &str, Action, &str)]) -> Live {
        Live {
            active: true,
            rules: items
                .iter()
                .map(|(p, pr, a, f)| LiveRule {
                    port: *p,
                    proto: (*pr).into(),
                    action: a.clone(),
                    from: (*f).into(),
                    family: Family::V4,
                })
                .collect(),
        }
    }

    #[test]
    fn exact_match_is_clean() {
        let i = intent_fx();
        let l = live_fx(&[
            (22, "tcp", Action::Limit, "any"),
            (443, "tcp", Action::Allow, "any"),
        ]);
        let r = compare(&i, &l);
        assert!(r.is_clean(), "{:#?}", r);
    }

    #[test]
    fn missing_rule_surfaced() {
        let i = intent_fx();
        let l = live_fx(&[(22, "tcp", Action::Limit, "any")]);
        let r = compare(&i, &l);
        assert_eq!(r.missing.len(), 1);
        assert_eq!(r.missing[0].port, 443);
    }

    #[test]
    fn extra_live_rule_surfaced() {
        let i = intent_fx();
        let l = live_fx(&[
            (22, "tcp", Action::Limit, "any"),
            (443, "tcp", Action::Allow, "any"),
            (8080, "tcp", Action::Allow, "any"),
        ]);
        let r = compare(&i, &l);
        assert_eq!(r.extra.len(), 1);
        assert_eq!(r.extra[0].port, 8080);
    }

    #[test]
    fn inactive_ufw_fails() {
        let i = intent_fx();
        let l = Live {
            active: false,
            rules: vec![],
        };
        let r = compare(&i, &l);
        assert!(!r.is_clean());
        assert!(!r.active_ok);
    }

    #[test]
    fn integration_parse_then_diff() {
        // Mimic real `ufw status numbered` column padding (double-space gaps).
        let text = "Status: active\n\n\
                    [ 1] 22/tcp                     LIMIT IN    Anywhere\n\
                    [ 2] 443/tcp                    ALLOW IN    Anywhere\n";
        let live = ufw::parse(text).unwrap();
        let r = compare(&intent_fx(), &live);
        assert!(r.is_clean(), "{:#?}", r);
    }
}
