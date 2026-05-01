//! Matrix loader — parses leak-matrix.toml, compiles every regex upfront.
//!
//! Pattern strings are IP. Never echoed outside the in-memory regex.
//! Public-facing fields: id, category, severity, scope, rationale, added.

use anyhow::{bail, Context, Result};
use regex::Regex;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity { Block, Warn, Substitute, Exclude }

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Block => "block",
            Severity::Warn => "warn",
            Severity::Substitute => "substitute",
            Severity::Exclude => "exclude",
        }
    }
}

impl FromStr for Severity {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "block" => Ok(Severity::Block),
            "warn" => Ok(Severity::Warn),
            "substitute" => Ok(Severity::Substitute),
            "exclude" => Ok(Severity::Exclude),
            o => bail!("unknown severity: {o}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope { AllWrites, PublicMirror, GithubPush, CommitMsg }

impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self {
            Scope::AllWrites => "all-writes",
            Scope::PublicMirror => "public-mirror",
            Scope::GithubPush => "github-push",
            Scope::CommitMsg => "commit-msg",
        }
    }
}

impl FromStr for Scope {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "all-writes" => Ok(Scope::AllWrites),
            "public-mirror" => Ok(Scope::PublicMirror),
            "github-push" => Ok(Scope::GithubPush),
            "commit-msg" => Ok(Scope::CommitMsg),
            o => bail!("unknown scope: {o}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category { PatentIp, Secret, Personal, InternalInfra, PrivateProject }

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::PatentIp => "patent-ip",
            Category::Secret => "secret",
            Category::Personal => "personal",
            Category::InternalInfra => "internal-infra",
            Category::PrivateProject => "private-project",
        }
    }
}

impl FromStr for Category {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "patent-ip" => Ok(Category::PatentIp),
            "secret" => Ok(Category::Secret),
            "personal" => Ok(Category::Personal),
            "internal-infra" => Ok(Category::InternalInfra),
            "private-project" => Ok(Category::PrivateProject),
            o => bail!("unknown category: {o}"),
        }
    }
}

/// One compiled rule. `pattern` is private — only `regex` is exposed.
#[derive(Debug, Clone)]
pub struct Rule {
    pub id: String,
    pub regex: Regex,
    pub category: Category,
    pub severity: Severity,
    pub substitute_with: Option<String>,
    pub scope: Vec<Scope>,
    pub rationale: String,
    pub added: String,
}

impl Rule {
    pub fn matches_scope(&self, requested: Scope) -> bool {
        self.scope.iter().any(|s| *s == requested || *s == Scope::AllWrites)
    }
}

#[derive(Debug, Deserialize)]
struct RawDoc { rule: Vec<RawRule> }

#[derive(Debug, Deserialize)]
struct RawRule {
    id: String,
    pattern: String,
    category: String,
    severity: String,
    #[serde(default)] substitute_with: Option<String>,
    scope: Vec<String>,
    rationale: String,
    added: String,
}

#[derive(Debug, Clone)]
pub struct Matrix { pub rules: Vec<Rule> }

impl Matrix {
    pub fn load(path: &Path) -> Result<Self> {
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("read matrix: {}", path.display()))?;
        let doc: RawDoc = toml::from_str(&text)
            .with_context(|| format!("parse matrix: {}", path.display()))?;
        let mut out = Vec::with_capacity(doc.rule.len());
        for raw in doc.rule { out.push(Self::compile(raw)?); }
        Ok(Matrix { rules: out })
    }

    fn compile(raw: RawRule) -> Result<Rule> {
        let regex = Regex::new(&raw.pattern)
            .with_context(|| format!("regex compile failed for rule {}", raw.id))?;
        let category = Category::from_str(&raw.category)
            .with_context(|| format!("rule {}", raw.id))?;
        let severity = Severity::from_str(&raw.severity)
            .with_context(|| format!("rule {}", raw.id))?;
        let mut scope = Vec::with_capacity(raw.scope.len());
        for s in &raw.scope {
            scope.push(Scope::from_str(s).with_context(|| format!("rule {}", raw.id))?);
        }
        Ok(Rule {
            id: raw.id, regex, category, severity,
            substitute_with: raw.substitute_with,
            scope, rationale: raw.rationale, added: raw.added,
        })
    }
}

/// Default matrix path: $KEI_LEAK_MATRIX_PATH or ~/Projects/KeiSeiKit/security/leak-matrix.toml
pub fn default_matrix_path() -> PathBuf {
    if let Ok(p) = std::env::var("KEI_LEAK_MATRIX_PATH") { return PathBuf::from(p); }
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join("Projects/KeiSeiKit/security/leak-matrix.toml")
}

/// Handler: print rules as a markdown table; optional category filter.
/// IP-safe: never prints the regex source — only id / category / severity / scope / rationale / added.
pub fn cmd_list(matrix: &Matrix, filter: Option<Category>) -> i32 {
    println!("| id | category | severity | scope | rationale | added |");
    println!("|----|----------|----------|-------|-----------|-------|");
    for r in &matrix.rules {
        if let Some(c) = filter { if r.category != c { continue; } }
        let scopes = r.scope.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ");
        println!("| {} | {} | {} | {} | {} | {} |",
            r.id, r.category.as_str(), r.severity.as_str(),
            scopes, r.rationale, r.added);
    }
    0
}

/// Handler: lint — does any existing rule already cover the candidate input?
/// Test the input against each compiled regex (do NOT compile the candidate).
pub fn cmd_lint(matrix: &Matrix, candidate: &str) -> i32 {
    for r in &matrix.rules {
        if r.regex.is_match(candidate) {
            println!("{}", r.id);
            return 0;
        }
    }
    println!("no match (suggested category: secret)");
    0
}
