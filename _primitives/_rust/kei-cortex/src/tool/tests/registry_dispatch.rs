//! Validates the registry's dispatch table:
//! - default registry has all 8 named tools
//! - dispatch on unknown name yields `is_error = true` (no panic)
//! - dispatch on registered tool returns a successful `ToolResult`
//! - registry survives concurrent dispatches (Send + Sync via Arc)

use crate::tool::registry::ToolRegistry;
use crate::tool::types::ToolCall;
use std::path::PathBuf;
use std::sync::Arc;

#[tokio::test]
async fn default_registry_lists_all_eight() {
    let r = ToolRegistry::default();
    let names = r.names();
    assert_eq!(
        names,
        vec![
            "agent".to_string(),
            "bash".into(),
            "edit".into(),
            "glob".into(),
            "grep".into(),
            "read".into(),
            "webfetch".into(),
            "write".into(),
        ]
    );
}

#[tokio::test]
async fn dispatch_unknown_name_returns_error_result() {
    let r = ToolRegistry::default();
    let call = ToolCall {
        id: "tu_unknown".into(),
        name: "does_not_exist".into(),
        input: serde_json::json!({}),
    };
    let res = r.dispatch(call).await;
    assert!(res.is_error);
    assert_eq!(res.tool_use_id, "tu_unknown");
    assert!(res.content.contains("unknown tool"));
}

#[tokio::test]
async fn dispatch_read_returns_file_contents() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("hello.txt");
    tokio::fs::write(&path, "alpha\nbeta\n").await.unwrap();
    // Build registry rooted at the tempdir so the read passes the
    // chroot check.
    let r = ToolRegistry::with_project_root(dir.path().to_path_buf());
    let call = ToolCall {
        id: "tu_read".into(),
        name: "read".into(),
        input: serde_json::json!({"path": path.to_str().unwrap()}),
    };
    let res = r.dispatch(call).await;
    assert!(!res.is_error, "got error: {}", res.content);
    assert!(res.content.contains("alpha"));
    assert!(res.content.contains("beta"));
}

#[tokio::test]
async fn dispatch_read_outside_root_errors() {
    let dir = tempfile::tempdir().unwrap();
    let other = tempfile::tempdir().unwrap();
    let path = other.path().join("escape.txt");
    tokio::fs::write(&path, "secret").await.unwrap();
    let r = ToolRegistry::with_project_root(dir.path().to_path_buf());
    let call = ToolCall {
        id: "tu_x".into(),
        name: "read".into(),
        input: serde_json::json!({"path": path.to_str().unwrap()}),
    };
    let res = r.dispatch(call).await;
    assert!(res.is_error);
    assert!(res.content.contains("project_root") || res.content.contains("not inside"));
}

#[tokio::test]
async fn registry_supports_concurrent_dispatch() {
    let dir = tempfile::tempdir().unwrap();
    let p1 = dir.path().join("a");
    let p2 = dir.path().join("b");
    tokio::fs::write(&p1, "AAA").await.unwrap();
    tokio::fs::write(&p2, "BBB").await.unwrap();
    let r = Arc::new(ToolRegistry::with_project_root(dir.path().to_path_buf()));
    let r1 = r.clone();
    let r2 = r.clone();
    let p1s = p1.to_string_lossy().to_string();
    let p2s = p2.to_string_lossy().to_string();
    let h1 = tokio::spawn(async move {
        r1.dispatch(ToolCall {
            id: "1".into(),
            name: "read".into(),
            input: serde_json::json!({"path": p1s}),
        })
        .await
    });
    let h2 = tokio::spawn(async move {
        r2.dispatch(ToolCall {
            id: "2".into(),
            name: "read".into(),
            input: serde_json::json!({"path": p2s}),
        })
        .await
    });
    let (a, b) = (h1.await.unwrap(), h2.await.unwrap());
    assert!(!a.is_error && a.content.contains("AAA"));
    assert!(!b.is_error && b.content.contains("BBB"));
    let _ = PathBuf::new();
}
