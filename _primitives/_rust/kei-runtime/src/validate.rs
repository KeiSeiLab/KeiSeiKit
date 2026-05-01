//! JSON Schema draft-07 validation wrappers.
//!
//! Thin façade over the `jsonschema` crate (v0.18). Reads schema from disk
//! per call. Returns a single, readable error message.
//!
//! SSRF / IMDS hardening:
//!   - `default-features = false` on `jsonschema` — no `resolve-http` feature.
//!   - Custom `LocalFileResolver` replaces the default. It rejects any URL
//!     whose scheme isn't `file://` and any path outside the schema's own
//!     directory (anchored at the schema file's parent).

use jsonschema::{JSONSchema, SchemaResolver, SchemaResolverError};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use url::Url;

#[derive(Debug)]
pub struct ValidationError(pub String);

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "validation: {}", self.0)
    }
}

impl std::error::Error for ValidationError {}

/// Validate `input` against JSON Schema at `schema_path`.
pub fn validate_input(schema_path: &Path, input: &Value) -> Result<(), ValidationError> {
    validate_value(schema_path, input)
}

/// Validate `output` against JSON Schema at `schema_path`.
pub fn validate_output(schema_path: &Path, output: &Value) -> Result<(), ValidationError> {
    validate_value(schema_path, output)
}

fn validate_value(schema_path: &Path, value: &Value) -> Result<(), ValidationError> {
    let schema_text = std::fs::read_to_string(schema_path)
        .map_err(|e| ValidationError(format!("read {}: {e}", schema_path.display())))?;
    let mut schema_json: Value = serde_json::from_str(&schema_text)
        .map_err(|e| ValidationError(format!("parse {}: {e}", schema_path.display())))?;
    // jsonschema 0.18 requires an absolute base URI for the schema. Our atom
    // schemas typically declare a relative `$id` like
    // "kei-task/atoms/schemas/create-input.json" which fails compile with
    // "relative URL without a base". Inject a synthetic `file://` $id keyed
    // to the actual schema path so any internal `$ref` still resolves
    // relative to the file (and our LocalFileResolver confines to the
    // schema's parent dir for safety).
    inject_absolute_id(&mut schema_json, schema_path);
    let root = schema_path.parent().unwrap_or(schema_path).to_path_buf();
    let compiled = JSONSchema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .with_resolver(LocalFileResolver::new(root))
        .compile(&schema_json)
        .map_err(|e| ValidationError(format!("compile: {e}")))?;
    if let Err(errors) = compiled.validate(value) {
        let msg = errors.map(|e| e.to_string()).collect::<Vec<_>>().join("; ");
        return Err(ValidationError(msg));
    }
    Ok(())
}

fn inject_absolute_id(schema: &mut Value, schema_path: &Path) {
    let obj = match schema.as_object_mut() {
        Some(o) => o,
        None => return,
    };
    let needs_replace = match obj.get("$id").and_then(|v| v.as_str()) {
        None => true,                                          // missing
        Some(s) => Url::parse(s).is_err(),                     // non-absolute
    };
    if !needs_replace {
        return;
    }
    if let Ok(canon) = schema_path.canonicalize() {
        if let Ok(url) = Url::from_file_path(&canon) {
            obj.insert("$id".to_string(), Value::String(url.to_string()));
        }
    }
}

/// `$ref` resolver that rejects every scheme except `file://`, AND rejects
/// any path that is not inside `root` OR the shared `_schemas/fragments/` dir
/// (canonicalised). The fragments dir is resolved by walking up from `root`
/// until a sibling `_schemas/fragments/` is found or we reach filesystem root.
#[derive(Debug)]
pub struct LocalFileResolver {
    root: PathBuf,
}

impl LocalFileResolver {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Walk up from root to find workspace's `_schemas/fragments/`. Returns
    /// canonicalised path if found. Allows atom schemas to $ref shared
    /// fragments without opening the entire filesystem.
    fn find_fragments_root(&self) -> Option<PathBuf> {
        let mut cur = self.root.as_path();
        loop {
            let candidate = cur.join("_schemas").join("fragments");
            if let Ok(canon) = candidate.canonicalize() {
                return Some(canon);
            }
            cur = cur.parent()?;
        }
    }
}

impl SchemaResolver for LocalFileResolver {
    fn resolve(
        &self,
        _root_schema: &Value,
        url: &Url,
        _original_reference: &str,
    ) -> Result<Arc<Value>, SchemaResolverError> {
        if url.scheme() != "file" {
            return Err(anyhow::anyhow!(
                "remote $ref rejected — only file:// is allowed (got {})",
                url.scheme()
            ));
        }
        let path = url
            .to_file_path()
            .map_err(|_| anyhow::anyhow!("invalid file URL: {url}"))?;
        let canon = path
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("canonicalize {}: {e}", path.display()))?;
        let root_canon = self
            .root
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("canonicalize root {}: {e}", self.root.display()))?;
        let fragments_canon = self.find_fragments_root();
        let in_root = canon.starts_with(&root_canon);
        let in_fragments = fragments_canon
            .as_ref()
            .map(|f| canon.starts_with(f))
            .unwrap_or(false);
        if !in_root && !in_fragments {
            return Err(anyhow::anyhow!(
                "file $ref escapes both schema root and fragments dir: {} not under {} or _schemas/fragments/",
                canon.display(),
                root_canon.display()
            ));
        }
        let f = std::fs::File::open(&canon)
            .map_err(|e| anyhow::anyhow!("open {}: {e}", canon.display()))?;
        let mut doc: Value = serde_json::from_reader(f)
            .map_err(|e| anyhow::anyhow!("parse {}: {e}", canon.display()))?;
        // Override any relative `$id` in the loaded fragment with its
        // absolute file:// URL. Without this, a fragment declaring e.g.
        // `$id: "_schemas/fragments/titled-content.json"` would be
        // resolved relative to the parent schema's $id by jsonschema,
        // producing a doubled prefix (`_schemas/fragments/_schemas/...`).
        if let Some(obj) = doc.as_object_mut() {
            if let Ok(abs_url) = Url::from_file_path(&canon) {
                obj.insert("$id".to_string(), Value::String(abs_url.to_string()));
            }
        }
        Ok(Arc::new(doc))
    }
}
