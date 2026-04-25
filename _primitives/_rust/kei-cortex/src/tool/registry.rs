//! `ToolRegistry` — name → executor dispatch table.
//!
//! Each entry is a boxed async fn that takes the parsed `ToolCall.input`
//! JSON and returns either `String` (success content) or `ToolError`. The
//! registry is built once at daemon startup via `with_project_root(...)`;
//! the agentic loop borrows it for each turn.
//!
//! Project-root chroot: tools that touch the filesystem (`read`, `write`,
//! `edit`, `glob`, `grep`) all enforce that requested paths resolve
//! INSIDE the configured `project_root`. The registry is constructed
//! once with the `project_root` captured into each tool's closure so
//! the path is immutable for the daemon's lifetime.
//!
//! Constructor Pattern: this module owns ONLY the dispatch table. Each
//! tool's logic lives in its own sibling cube (`read.rs`, `write.rs`, …).

use super::types::{ToolCall, ToolError, ToolResult};
use super::{agent, bash, edit, glob_tool, grep, read, webfetch, write};
use futures::future::BoxFuture;
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

/// Async executor signature. Each tool's implementation module exports a
/// public `run(input: Value, project_root: Arc<PathBuf>) -> Result<...>`
/// matching this. We capture `project_root` by-clone into each closure.
pub type Executor = Box<dyn Fn(Value) -> BoxFuture<'static, Result<String, ToolError>> + Send + Sync>;

/// Registry of tools, keyed by Anthropic `tool_use.name`.
pub struct ToolRegistry {
    table: HashMap<String, Executor>,
    project_root: PathBuf,
}

impl ToolRegistry {
    /// Empty registry with a project root. Useful for tests.
    pub fn empty(project_root: PathBuf) -> Self {
        Self {
            table: HashMap::new(),
            project_root,
        }
    }

    /// Register a tool by name. Overwrites any existing entry for the
    /// same name.
    pub fn register(&mut self, name: impl Into<String>, exec: Executor) {
        self.table.insert(name.into(), exec);
    }

    /// True iff the registry knows this name.
    pub fn has(&self, name: &str) -> bool {
        self.table.contains_key(name)
    }

    /// Names currently registered, sorted for deterministic listings.
    pub fn names(&self) -> Vec<String> {
        let mut v: Vec<String> = self.table.keys().cloned().collect();
        v.sort();
        v
    }

    /// The project root captured at construction.
    pub fn project_root(&self) -> &std::path::Path {
        &self.project_root
    }

    /// Dispatch one call. Unknown names produce a `ToolResult` with
    /// `is_error = true` so the model can recover (e.g., retry with a
    /// known name) instead of the loop crashing.
    pub async fn dispatch(&self, call: ToolCall) -> ToolResult {
        let Some(exec) = self.table.get(&call.name) else {
            return ToolResult::err(
                &call.id,
                format!("unknown tool: {}", call.name),
            );
        };
        match exec(call.input).await {
            Ok(content) => ToolResult::ok(&call.id, content),
            Err(e) => ToolResult::err(&call.id, e.as_message()),
        }
    }

    /// Build the production registry with all 8 tools wired in,
    /// capturing `project_root` for every filesystem-touching tool.
    pub fn with_project_root(project_root: PathBuf) -> Self {
        let mut r = Self::empty(project_root.clone());
        let root: Arc<PathBuf> = Arc::new(project_root);
        let r1 = root.clone();
        r.register("read", Box::new(move |v| {
            let p = r1.clone();
            Box::pin(async move { read::run(v, &p).await })
        }));
        let r2 = root.clone();
        r.register("write", Box::new(move |v| {
            let p = r2.clone();
            Box::pin(async move { write::run(v, &p).await })
        }));
        let r3 = root.clone();
        r.register("edit", Box::new(move |v| {
            let p = r3.clone();
            Box::pin(async move { edit::run(v, &p).await })
        }));
        r.register("bash", Box::new(|v| Box::pin(bash::run(v))));
        let r4 = root.clone();
        r.register("glob", Box::new(move |v| {
            let p = r4.clone();
            Box::pin(async move { glob_tool::run(v, &p).await })
        }));
        let r5 = root.clone();
        r.register("grep", Box::new(move |v| {
            let p = r5.clone();
            Box::pin(async move { grep::run(v, &p).await })
        }));
        r.register("webfetch", Box::new(|v| Box::pin(webfetch::run(v))));
        r.register("agent", Box::new(|v| Box::pin(agent::run(v))));
        r
    }
}

impl Default for ToolRegistry {
    /// Default builds with `project_root = "."` (cwd). Production code
    /// MUST call `with_project_root(state.config().project_root.clone())`
    /// instead — this default is only safe for tests + opaque defaults.
    fn default() -> Self {
        Self::with_project_root(PathBuf::from("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_has_all_eight() {
        let r = ToolRegistry::default();
        for name in ["read", "write", "edit", "bash", "glob", "grep", "webfetch", "agent"] {
            assert!(r.has(name), "missing tool: {name}");
        }
        assert_eq!(r.names().len(), 8);
    }

    #[tokio::test]
    async fn unknown_tool_produces_error_result() {
        let r = ToolRegistry::empty(PathBuf::from("."));
        let call = ToolCall {
            id: "tu_x".into(),
            name: "nonexistent".into(),
            input: serde_json::json!({}),
        };
        let res = r.dispatch(call).await;
        assert!(res.is_error);
        assert!(res.content.contains("unknown tool"));
    }

    #[tokio::test]
    async fn registered_executor_returns_ok() {
        let mut r = ToolRegistry::empty(PathBuf::from("."));
        r.register(
            "echo",
            Box::new(|v| Box::pin(async move { Ok(v.to_string()) })),
        );
        let call = ToolCall {
            id: "tu_e".into(),
            name: "echo".into(),
            input: serde_json::json!({"a": 1}),
        };
        let res = r.dispatch(call).await;
        assert!(!res.is_error);
        assert!(res.content.contains("\"a\":1"));
    }

    #[test]
    fn with_project_root_records_root() {
        let r = ToolRegistry::with_project_root(PathBuf::from("/tmp"));
        assert_eq!(r.project_root(), std::path::Path::new("/tmp"));
    }
}
