//! Artifact CRUD — register_schema / emit / get / list / chain / validate.
//!
//! Constructor Pattern: one concern per public fn, each < 30 LOC.
//! Every write path uses `artifact_id` for idempotency.

use crate::hash::artifact_id;
use crate::store::Store;
use crate::validate::{validate_content, warn_unsupported_keywords};
use anyhow::{anyhow, Context, Result};
use chrono::Utc;
use rusqlite::{params, OptionalExtension};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Clone)]
pub struct Artifact {
    pub id: String,
    pub schema_name: String,
    pub source_agent: String,
    pub content: Vec<u8>,
    pub meta_json: Option<String>,
    pub parent_artifact_id: Option<String>,
    pub created_at: i64,
}

#[derive(Debug, Default, Clone)]
pub struct ArtifactFilter {
    pub schema_name: Option<String>,
    pub source_agent: Option<String>,
    pub since: Option<i64>,
}

/// Insert a schema under `name`. Overwrite if present (idempotent registry).
///
/// Non-fatal audit: unsupported JSON Schema keywords (`pattern`, `format`,
/// `oneOf`, `$ref`, etc.) are logged to stderr via
/// [`warn_unsupported_keywords`] so the operator knows they are stored but not
/// runtime-enforced. This keeps the validator surface minimal while letting
/// humans leave documentation-style keywords in place.
pub fn register_schema(store: &Store, name: &str, json_schema: &str) -> Result<()> {
    let parsed: Value = serde_json::from_str(json_schema).context("schema is not valid JSON")?;
    if !parsed.is_object() {
        return Err(anyhow!("schema must be a JSON object"));
    }
    warn_unsupported_keywords(&parsed);
    let now = Utc::now().timestamp();
    store.conn().execute(
        "INSERT INTO schemas (name, json_schema, registered_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(name) DO UPDATE SET json_schema=excluded.json_schema,
                                         registered_at=excluded.registered_at",
        params![name, json_schema, now],
    )?;
    Ok(())
}

pub fn list_schemas(store: &Store) -> Result<Vec<String>> {
    let mut stmt = store.conn().prepare("SELECT name FROM schemas ORDER BY name")?;
    let rows = stmt
        .query_map([], |r| r.get::<_, String>(0))?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

fn load_schema(store: &Store, name: &str) -> Result<Value> {
    let raw: String = store
        .conn()
        .query_row(
            "SELECT json_schema FROM schemas WHERE name = ?1",
            params![name],
            |r| r.get(0),
        )
        .optional()?
        .ok_or_else(|| anyhow!("unknown schema '{name}' — register it first"))?;
    serde_json::from_str(&raw).context("stored schema is not valid JSON")
}

/// Emit a typed artifact. Returns the id. Idempotent (same bytes → same id).
pub fn emit(
    store: &Store,
    schema_name: &str,
    source_agent: &str,
    content: &[u8],
    meta_json: Option<&str>,
    parent: Option<&str>,
) -> Result<String> {
    if let Some(pid) = parent {
        if !has_artifact(store, pid)? {
            return Err(anyhow!("parent artifact '{pid}' not found"));
        }
    }
    let schema = load_schema(store, schema_name)?;
    let value: Value = serde_json::from_slice(content).context("artifact content not JSON")?;
    validate_content(&schema, &value).map_err(|e| anyhow!("schema-validation: {e}"))?;
    let id = artifact_id(schema_name, content);
    let now = Utc::now().timestamp();
    store.conn().execute(
        "INSERT OR IGNORE INTO artifacts
           (id, schema_name, source_agent, content, meta_json, parent_artifact_id, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, schema_name, source_agent, content, meta_json, parent, now],
    )?;
    Ok(id)
}

fn has_artifact(store: &Store, id: &str) -> Result<bool> {
    let n: i64 = store.conn().query_row(
        "SELECT COUNT(*) FROM artifacts WHERE id = ?1",
        params![id],
        |r| r.get(0),
    )?;
    Ok(n > 0)
}

pub fn get(store: &Store, id: &str) -> Result<Option<Artifact>> {
    let row = store
        .conn()
        .query_row(
            "SELECT id, schema_name, source_agent, content, meta_json,
                    parent_artifact_id, created_at
             FROM artifacts WHERE id = ?1",
            params![id],
            row_to_artifact,
        )
        .optional()?;
    Ok(row)
}

fn row_to_artifact(r: &rusqlite::Row) -> rusqlite::Result<Artifact> {
    Ok(Artifact {
        id: r.get(0)?,
        schema_name: r.get(1)?,
        source_agent: r.get(2)?,
        content: r.get(3)?,
        meta_json: r.get(4)?,
        parent_artifact_id: r.get(5)?,
        created_at: r.get(6)?,
    })
}

/// Re-validate a stored artifact against its schema. Useful after schema
/// revision to detect rows that no longer satisfy the contract.
pub fn validate_by_id(store: &Store, id: &str) -> Result<()> {
    let a = get(store, id)?.ok_or_else(|| anyhow!("artifact '{id}' not found"))?;
    let schema = load_schema(store, &a.schema_name)?;
    let value: Value = serde_json::from_slice(&a.content).context("artifact content not JSON")?;
    validate_content(&schema, &value).map_err(|e| anyhow!("schema-validation: {e}"))?;
    Ok(())
}

/// Filter-based listing; ORDER BY created_at DESC.
pub fn list(store: &Store, filter: &ArtifactFilter) -> Result<Vec<Artifact>> {
    let (sql, args) = build_list_sql(filter);
    let mut stmt = store.conn().prepare(&sql)?;
    let rows = stmt
        .query_map(rusqlite::params_from_iter(args.iter()), row_to_artifact)?
        .collect::<rusqlite::Result<Vec<_>>>()?;
    Ok(rows)
}

fn build_list_sql(f: &ArtifactFilter) -> (String, Vec<String>) {
    let mut sql = String::from(
        "SELECT id, schema_name, source_agent, content, meta_json, \
         parent_artifact_id, created_at FROM artifacts",
    );
    let mut args: Vec<String> = Vec::new();
    let mut clauses: Vec<String> = Vec::new();
    if let Some(s) = &f.schema_name {
        clauses.push(format!("schema_name = ?{}", clauses.len() + 1));
        args.push(s.clone());
    }
    if let Some(a) = &f.source_agent {
        clauses.push(format!("source_agent = ?{}", clauses.len() + 1));
        args.push(a.clone());
    }
    if let Some(since) = f.since {
        clauses.push(format!("created_at >= ?{}", clauses.len() + 1));
        args.push(since.to_string());
    }
    if !clauses.is_empty() {
        sql.push_str(" WHERE ");
        sql.push_str(&clauses.join(" AND "));
    }
    sql.push_str(" ORDER BY created_at DESC");
    (sql, args)
}

/// Walk the parent chain upward from `id`. Root first, youngest last.
pub fn chain(store: &Store, id: &str) -> Result<Vec<Artifact>> {
    let mut out: Vec<Artifact> = Vec::new();
    let mut current = Some(id.to_string());
    while let Some(cid) = current {
        let a = get(store, &cid)?.ok_or_else(|| anyhow!("artifact '{cid}' not found"))?;
        current = a.parent_artifact_id.clone();
        out.push(a);
    }
    out.reverse();
    Ok(out)
}
