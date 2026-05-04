//! Formula types — pure data shape for the block formula 4-tuple.
//!
//! Constructor Pattern: this cube owns ONLY the type definitions. Hashing
//! lives in `sha.rs`; SQL persistence in `io.rs`; the public API in
//! `mod.rs`. Mirrors `arch/MATH-DNA-DESIGN.md` §1.1.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::PathBuf;

/// Block formula 4-tuple: (Type, Invariants, Effects, Deps) anchored to a
/// `block_id`. `source` records whether the formula was authored, derived,
/// or both.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BlockFormula {
    pub block_id: i64,
    pub r#type: TypeSignature,
    pub invariants: Vec<Predicate>,
    pub effects: BTreeSet<EffectKind>,
    pub deps: BTreeSet<String>,
    pub source: FormulaSource,
}

/// Function-level type signature: the declared input/output/error atoms.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TypeSignature {
    pub inputs: Vec<TypeAtom>,
    pub output: TypeAtom,
    pub errors: Vec<TypeAtom>,
}

/// Coarse type atoms recognised by the registry. `Custom` carries an
/// opaque label for any project-specific type the registry does not need
/// to introspect.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TypeAtom {
    Unit,
    Bool,
    Int,
    String,
    Path,
    Json,
    Bytes,
    Custom(String),
}

/// Invariant predicate — a checkable assertion about the workspace state
/// or about the block's body itself. Stored as `(kind, args_json)` rows
/// in `block_predicates`; reconstructed via `serde_json::from_str` against
/// this tagged enum.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Predicate {
    ContentRegex {
        file: PathBuf,
        pattern: String,
        min: u32,
        max: Option<u32>,
    },
    ContentNotRegex {
        file: PathBuf,
        pattern: String,
    },
    FileExists {
        path: PathBuf,
    },
    JsonSchema {
        file: PathBuf,
        schema: PathBuf,
    },
    HttpStatus {
        url: String,
        expected: Vec<u16>,
    },
    CargoCheck {
        member: String,
    },
    CargoTest {
        member: String,
        filter: Option<String>,
    },
    SymbolDeclared {
        file: PathBuf,
        name: String,
        symbol_kind: SymbolKind,
    },
    BodyShaEq {
        sha8: String,
    },
}

/// Symbol-kind tag used by the `SymbolDeclared` predicate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SymbolKind {
    Fn,
    Struct,
    Enum,
    Trait,
    Const,
    Impl,
}

/// Side-effect classification. Sorted by `Ord` so the canonical hash is
/// deterministic regardless of insertion order. Read-side effects come
/// first conventionally, but ordering is alphabetic on the `kind` tag.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum EffectKind {
    DbRead { backend: String },
    DbWrite { backend: String },
    EnvRead { var: String },
    EnvWrite { var: String },
    Exec { binary: String },
    FileLock { glob: String },
    FsRead { glob: String },
    FsWrite { glob: String },
    GitMutate,
    HashDigest,
    NetEgress { host_glob: String },
    NetIngress { port: Option<u16> },
    NetListen { port: u16 },
    Other(String),
    Sign,
    Sleep { seconds_max: u32 },
    SpawnAgent,
    Stderr,
    Stdin,
    Stdout,
}

/// Provenance of a formula. `Inferred` carries a 0..100 confidence score
/// from the eventual derivation pass; `Hybrid` means some fields were
/// declared and others inferred.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FormulaSource {
    Declared,
    Inferred { confidence: u8 },
    Hybrid,
}

/// Stable string tag for a `Predicate` variant — used as the `kind`
/// column in `block_predicates`. Centralised here so SQL writers and
/// readers agree on the wire vocabulary.
pub fn predicate_kind(p: &Predicate) -> &'static str {
    match p {
        Predicate::ContentRegex { .. } => "content_regex",
        Predicate::ContentNotRegex { .. } => "content_not_regex",
        Predicate::FileExists { .. } => "file_exists",
        Predicate::JsonSchema { .. } => "json_schema",
        Predicate::HttpStatus { .. } => "http_status",
        Predicate::CargoCheck { .. } => "cargo_check",
        Predicate::CargoTest { .. } => "cargo_test",
        Predicate::SymbolDeclared { .. } => "symbol_declared",
        Predicate::BodyShaEq { .. } => "body_sha_eq",
    }
}
