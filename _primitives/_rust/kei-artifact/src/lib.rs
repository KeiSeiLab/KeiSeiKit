//! kei-artifact — typed artifact handoff store.
//!
//! Constructor Pattern: one concern per file.
//! - `schema`    — SQL DDL + schema registry table.
//! - `store`     — `Store` cube (Connection wrapper).
//! - `hash`      — sha256 artifact id helper.
//! - `schemas`   — built-in schema registration (spec/plan/patch/review/research).
//! - `validate`  — minimal JSON Schema (strict subset of draft 2020-12).
//! - `artifact`  — CRUD on `artifacts` table (emit / get / list / chain).
//! - `export`    — v0.16 schema-registry export for the assembler.
//!
//! Storage path (CLI default): `~/.claude/artifacts/artifacts.sqlite` or
//! `$KEI_ARTIFACT_DB`.

pub mod artifact;
pub mod export;
pub mod hash;
pub mod schema;
pub mod schemas;
pub mod store;
pub mod validate;

pub use artifact::{Artifact, ArtifactFilter};
pub use hash::artifact_id;
pub use store::Store;
pub use validate::{validate_content, warn_unsupported_keywords};
