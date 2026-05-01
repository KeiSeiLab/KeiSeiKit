//! Generic pattern-based gate (Layer C convergence, 2026-04-23). Absorbs
//! 5 of 6 gate impls as `PatternGate` consts. `tools::deny-tools` stays
//! separate (tool-name match). Hardening (audit 2026-04-23): H1 regex
//! cache `Mutex`→`RwLock` (per-pattern `Lazy` needs sibling-gate edits,
//! out of scope). H2 `compile_checked()` → `Result`, no panics.
//! H3 `AllowIfMatch`+task-scope fails closed. S4 `char`-based truncation.
//! L2 single-pass template render; no replace-chain bleed.

use crate::capability::*;
use regex::Regex;
use std::collections::HashMap;
use std::sync::RwLock;

/// How the gate decides on a match.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GateMode {
    DenyIfMatch,
    AllowIfMatch,
    DenyIfUnmatched,
}

/// Static regex list or dynamic glob list pulled from TaskSpec scope.
#[derive(Clone, Copy)]
pub enum PatternSource {
    StaticRegex(&'static [&'static str]),
    TaskWhitelist,
    TaskDenylist,
}

/// Generic pattern-driven PreToolUse gate.
pub struct PatternGate {
    pub name: &'static str,
    pub tools: &'static [&'static str],
    pub field: &'static str,
    pub mode: GateMode,
    pub patterns: PatternSource,
    pub bypass_env: Option<&'static str>,
    pub deny_template: &'static str,
}

impl Capability for PatternGate {
    fn name(&self) -> &'static str {
        self.name
    }

    fn check(&self, ctx: &GateContext) -> GateDecision {
        if !self.tool_applies(ctx.tool_name) {
            return GateDecision::NotApplicable;
        }
        if self.bypass_active(ctx.env) {
            return GateDecision::Allow;
        }
        let Some(value) = self.read_field(ctx.tool_input) else {
            return GateDecision::NotApplicable;
        };
        match self.patterns {
            PatternSource::StaticRegex(arr) => self.decide_regex(&value, arr),
            PatternSource::TaskWhitelist => {
                self.decide_scope(&value, &ctx.task.scope.files_whitelist)
            }
            PatternSource::TaskDenylist => {
                self.decide_scope(&value, &ctx.task.scope.files_denylist)
            }
        }
    }
}

impl PatternGate {
    fn tool_applies(&self, tool: &str) -> bool {
        self.tools.is_empty() || self.tools.iter().any(|t| *t == tool)
    }

    fn bypass_active(&self, env: &HashMap<String, String>) -> bool {
        self.bypass_env
            .and_then(|k| env.get(k))
            .map(|v| v == "1")
            .unwrap_or(false)
    }

    fn read_field(&self, input: &serde_json::Value) -> Option<String> {
        input.get(self.field).and_then(|v| v.as_str()).map(String::from)
    }

    fn decide_regex(&self, value: &str, pats: &[&'static str]) -> GateDecision {
        if matches!(self.mode, GateMode::DenyIfUnmatched) && pats.is_empty() {
            return GateDecision::Allow;
        }
        let hit = match self.scan_regex(value, pats) {
            Ok(h) => h,
            Err(d) => return d,
        };
        match (self.mode, hit) {
            (GateMode::DenyIfMatch, Some(raw)) => self.deny(value, raw),
            (GateMode::DenyIfMatch, None) => GateDecision::Allow,
            (GateMode::AllowIfMatch, Some(_)) => GateDecision::Allow,
            (GateMode::AllowIfMatch, None) => self.deny(value, "<none>"),
            (GateMode::DenyIfUnmatched, Some(_)) => GateDecision::Allow,
            (GateMode::DenyIfUnmatched, None) => self.deny(value, "<unmatched>"),
        }
    }

    /// First matching raw pattern, or `Err(Deny)` on compile failure (H2).
    fn scan_regex<'p>(
        &self,
        value: &str,
        pats: &'p [&'static str],
    ) -> Result<Option<&'p &'static str>, GateDecision> {
        for raw in pats {
            match compile_checked(raw) {
                Ok(rx) if rx.is_match(value) => return Ok(Some(raw)),
                Ok(_) => {}
                Err(e) => return Err(GateDecision::Deny {
                    reason: format!(
                        "{} — capability misconfigured: regex `{}` invalid ({})",
                        self.name, raw, e
                    ),
                }),
            }
        }
        Ok(None)
    }

    fn decide_scope(&self, value: &str, pats: &[String]) -> GateDecision {
        use crate::simulated_merge::glob_match as gm;
        match self.mode {
            GateMode::DenyIfUnmatched if pats.is_empty() => GateDecision::Allow,
            GateMode::DenyIfUnmatched if pats.iter().any(|p| gm(p, value)) => GateDecision::Allow,
            GateMode::DenyIfUnmatched => self.deny(value, "<whitelist>"),
            GateMode::DenyIfMatch => pats
                .iter()
                .find(|p| gm(p, value))
                .map(|p| self.deny(value, p))
                .unwrap_or(GateDecision::Allow),
            // H3: AllowIfMatch + task-scope = misconfigured; fail closed.
            GateMode::AllowIfMatch => GateDecision::Deny {
                reason: format!(
                    "{} — capability misconfigured: AllowIfMatch + task-scope source is invalid; \
                     scope gates must use DenyIfMatch or DenyIfUnmatched",
                    self.name
                ),
            },
        }
    }

    fn deny(&self, value: &str, pat: &str) -> GateDecision {
        GateDecision::Deny { reason: render_template(self.deny_template, self.name, value, pat) }
    }
}

/// Single-pass template render (L2). Tokens: `{name}` `{path}` `{cmd}`
/// `{pat}`; unknown `{...}` emitted verbatim. Substituted text cannot
/// bleed into later tokens (unlike the old replace-chain).
fn render_template(tpl: &str, name: &str, value: &str, pat: &str) -> String {
    let mut out = String::with_capacity(tpl.len() + value.len());
    let cmd = truncate_chars(value, 60);
    let mut rest = tpl;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open..];
        let Some(close) = after.find('}') else {
            out.push_str(after);
            return out;
        };
        let token = &after[1..close];
        match token {
            "name" => out.push_str(name),
            "path" => out.push_str(value),
            "cmd" => out.push_str(&cmd),
            "pat" => out.push_str(pat),
            _ => out.push_str(&after[..=close]),
        }
        rest = &after[close + 1..];
    }
    out.push_str(rest);
    out
}

/// Compile + cache a regex (H1 + H2). `RwLock` cache: read-lock fast
/// path, write-lock only on first compile of each pattern.
fn compile_checked(raw: &str) -> Result<Regex, regex::Error> {
    use once_cell::sync::Lazy;
    static CACHE: Lazy<RwLock<HashMap<String, Regex>>> =
        Lazy::new(|| RwLock::new(HashMap::new()));
    if let Some(rx) = CACHE.read().unwrap_or_else(|p| p.into_inner()).get(raw).cloned() {
        return Ok(rx);
    }
    let rx = Regex::new(raw)?;
    let mut g = CACHE.write().unwrap_or_else(|p| p.into_inner());
    Ok(g.entry(raw.to_string()).or_insert(rx).clone())
}

/// Truncate to `max` chars (S4). Safe for multi-byte code points.
fn truncate_chars(s: &str, max: usize) -> String {
    let mut it = s.chars();
    let mut out: String = it.by_ref().take(max).collect();
    if it.next().is_some() {
        out.push('…');
    }
    out
}
