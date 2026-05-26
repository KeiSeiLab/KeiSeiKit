//! `tools/list` and `tools/call` — atom registry as MCP tools.
//!
//! Atom→tool mapping:
//!   name        = `<crate>::<verb>` (atom full id)
//!   description = first paragraph of the atom's body
//!   inputSchema = JSON loaded from `meta.input_schema`, or `{}` if missing
//!
//! `tools/call` resolves the binary at `<crate>` (via PATH or
//! `KEI_RUNTIME_BIN_DIR`) and shells out as `<crate> run-atom <verb>`,
//! piping the JSON arguments on stdin. Stdout is parsed back as JSON.
//!
//! MISS-4: the spawn is wrapped in a `tokio::time::timeout` so a hung atom
//! cannot block the JSON-RPC channel. Hard cap is `ATOM_TIMEOUT_SECS` (60s).
//! On timeout the child is killed and a `-32603` error is returned with
//! message `atom timeout`.

use crate::protocol::{err, ok, JsonRpcRequest, JsonRpcResponse, ServerContext, INTERNAL_ERROR, INVALID_PARAMS};
use kei_atom_discovery::{discover_atoms, AtomMeta};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::process::Stdio;
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// Hard cap on how long a single `tools/call` invocation may take. A
/// misbehaving atom is killed at this point; the caller sees a JSON-RPC
/// `-32603 atom timeout` error and the channel stays alive.
const ATOM_TIMEOUT_SECS: u64 = 60;

pub fn list(req: JsonRpcRequest, ctx: &ServerContext) -> JsonRpcResponse {
    let mut tools: Vec<Value> = discover_atoms(&ctx.atoms_root)
        .into_iter()
        .map(atom_to_tool_descriptor)
        .collect();
    // v0.39: built-in spawn_agent tool — exposed to all MCP clients so any
    // CLI (grok / agy / copilot / kimi / claude) can spawn a KeiSeiKit agent
    // as a sub-agent. Bypasses atom discovery (it's an internal handler).
    tools.push(spawn_agent_descriptor());
    tools.sort_by(|a, b| {
        a.get("name").and_then(Value::as_str).unwrap_or("")
            .cmp(b.get("name").and_then(Value::as_str).unwrap_or(""))
    });
    ok(req.id, json!({ "tools": tools }))
}

pub async fn call(req: JsonRpcRequest, ctx: &ServerContext) -> JsonRpcResponse {
    let params = match req.params.clone() {
        Some(p) => p,
        None => return err(req.id, INVALID_PARAMS, "missing params"),
    };
    let name = match params.get("name").and_then(Value::as_str) {
        Some(n) => n.to_string(),
        None => return err(req.id, INVALID_PARAMS, "missing tool name"),
    };
    let args = params.get("arguments").cloned().unwrap_or(json!({}));

    // v0.39: spawn_agent built-in — short-circuit before atom dispatch.
    if name == "spawn_agent" {
        return match invoke_spawn_agent(&args).await {
            Ok(text) => ok(req.id, json!({
                "content": [{ "type": "text", "text": text }],
                "isError": false,
            })),
            Err(e) => err(req.id, INTERNAL_ERROR, e),
        };
    }

    match invoke_atom(&ctx.atoms_root, &name, &args).await {
        Ok(result) => ok(req.id, json!({
            "content": [{ "type": "text", "text": serde_json::to_string(&result).unwrap_or_default() }],
            "isError": false,
        })),
        Err(e) => err(req.id, INTERNAL_ERROR, e),
    }
}

/// v0.39: built-in `spawn_agent` MCP tool descriptor.
/// Exposes KeiSeiKit's cross-CLI agent launcher (`kei-agent-cli.sh`) so any
/// MCP client can spawn an agent on any backend (claude / grok / agy /
/// copilot / kimi). Solves the "non-claude orchestrator can't natively spawn
/// sub-agents" gap — any CLI with MCP support gets the spawn capability.
fn spawn_agent_descriptor() -> Value {
    json!({
        "name": "spawn_agent",
        "description": "Spawn a KeiSeiKit agent as a sub-agent through any configured LLM CLI backend. Reads ~/.claude/agents/<name>.md, composes with the task, and execs the chosen backend non-interactively. Backend resolution: explicit `on` arg → agent manifest's `provider` → ~/.claude/config/primary.toml → claude.",
        "inputSchema": {
            "type": "object",
            "properties": {
                "name": {
                    "type": "string",
                    "description": "Agent name (looked up in ~/.claude/agents/<name>.md)"
                },
                "task": {
                    "type": "string",
                    "description": "The task / question to give the agent"
                },
                "on": {
                    "type": "string",
                    "description": "Optional explicit backend override (claude/grok/agy/copilot/kimi/codex). Default: DNA → primary → claude.",
                    "enum": ["claude", "grok", "agy", "antigravity", "copilot", "kimi", "codex"]
                }
            },
            "required": ["name", "task"]
        }
    })
}

/// v0.39: handler for `tools/call name=spawn_agent`. Shells out to
/// `kei-agent-cli.sh` (located via $HOME/.claude/scripts/) and returns
/// the backend's stdout as the tool result.
async fn invoke_spawn_agent(args: &Value) -> Result<String, String> {
    let name = args.get("name").and_then(Value::as_str)
        .ok_or_else(|| "spawn_agent: missing 'name' argument".to_string())?;
    let task = args.get("task").and_then(Value::as_str)
        .ok_or_else(|| "spawn_agent: missing 'task' argument".to_string())?;
    let on_opt = args.get("on").and_then(Value::as_str);

    // Locate the launcher script. Honors KEI_AGENT_CLI override for testing.
    let script = match std::env::var("KEI_AGENT_CLI") {
        Ok(v) => PathBuf::from(v),
        Err(_) => {
            let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
            PathBuf::from(home).join(".claude/scripts/kei-agent-cli.sh")
        }
    };
    if !script.is_file() {
        return Err(format!("kei-agent-cli.sh not found: {}", script.display()));
    }

    let mut cmd = Command::new(&script);
    if let Some(on) = on_opt {
        cmd.arg(format!("--on={on}"));
    }
    cmd.arg(name).arg(task);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);

    let child = cmd.spawn()
        .map_err(|e| format!("spawn {}: {e}", script.display()))?;
    let fut = child.wait_with_output();

    // Reuse the existing ATOM_TIMEOUT_SECS for the spawn_agent cap too —
    // 60s should suffice for non-interactive prompts; longer tasks would
    // need streaming, which the MCP tools-call contract doesn't support
    // anyway. Hung agents are killed at the timeout.
    match tokio::time::timeout(Duration::from_secs(ATOM_TIMEOUT_SECS), fut).await {
        Ok(Ok(out)) => {
            let stdout = String::from_utf8_lossy(&out.stdout).to_string();
            if !out.status.success() {
                let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
                return Err(format!(
                    "spawn_agent backend exited {}: {stderr}",
                    out.status.code().unwrap_or(-1)
                ));
            }
            Ok(stdout)
        }
        Ok(Err(e)) => Err(format!("wait: {e}")),
        Err(_) => Err("spawn_agent timeout".into()),
    }
}

/// Convert one atom's metadata into the MCP tool-descriptor shape.
fn atom_to_tool_descriptor(meta: AtomMeta) -> Value {
    let description = first_paragraph(&meta.body);
    let input_schema = load_schema(meta.input_schema.as_ref());
    json!({
        "name": meta.full_id,
        "description": description,
        "inputSchema": input_schema,
    })
}

/// Extract the first non-empty paragraph from a markdown body. Headings
/// are stripped (lines that start with `#`). Returns "" if no content.
fn first_paragraph(body: &str) -> String {
    let mut buf = String::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            if !buf.is_empty() {
                break;
            }
            continue;
        }
        if !buf.is_empty() {
            buf.push(' ');
        }
        buf.push_str(trimmed);
    }
    buf
}

/// Read a schema file as JSON. Returns `{}` on any failure (missing file,
/// non-UTF-8, parse error) — the MCP client will see an empty object,
/// not a runtime error.
fn load_schema(schema_path: Option<&PathBuf>) -> Value {
    let path = match schema_path {
        Some(p) => p,
        None => return json!({}),
    };
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return json!({}),
    };
    serde_json::from_str(&text).unwrap_or_else(|_| json!({}))
}

/// Resolve an atom by full id, then shell out to `<crate> run-atom <verb>`
/// with the argument JSON piped on stdin. Wrapped in a 60s timeout.
async fn invoke_atom(root: &std::path::Path, full_id: &str, args: &Value) -> Result<Value, String> {
    let meta = discover_atoms(root)
        .into_iter()
        .find(|a| a.full_id == full_id)
        .ok_or_else(|| format!("unknown tool: {full_id}"))?;
    let bin = resolve_binary(&meta.crate_name)
        .ok_or_else(|| format!("binary not found for crate `{}`", meta.crate_name))?;
    let fut = spawn_and_collect(&bin, &meta.verb, args);
    match tokio::time::timeout(Duration::from_secs(ATOM_TIMEOUT_SECS), fut).await {
        Ok(res) => res,
        Err(_) => Err("atom timeout".into()),
    }
}

/// Spawn `<bin> run-atom <verb>` via `tokio::process::Command`, write the
/// JSON args to stdin, and parse stdout as JSON. The kill-on-drop flag is
/// set so a timeout in `invoke_atom` actually terminates the child.
async fn spawn_and_collect(bin: &std::path::Path, verb: &str, args: &Value) -> Result<Value, String> {
    let mut cmd = Command::new(bin);
    cmd.arg("run-atom")
        .arg(verb)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    let mut child = cmd
        .spawn()
        .map_err(|e| format!("spawn {}: {e}", bin.display()))?;
    write_args_to_stdin(&mut child, args).await?;
    let out = child
        .wait_with_output()
        .await
        .map_err(|e| format!("wait: {e}"))?;
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr).trim().to_string();
        return Err(format!(
            "atom exited {}: {stderr}",
            out.status.code().unwrap_or(-1)
        ));
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(stdout.trim())
        .map_err(|e| format!("atom stdout not JSON: {e}; stdout: {stdout}"))
}

/// Pipe JSON-encoded `args` into the child's stdin and close the write half
/// so the child sees EOF. Errors propagate as strings.
async fn write_args_to_stdin(child: &mut tokio::process::Child, args: &Value) -> Result<(), String> {
    let Some(mut stdin) = child.stdin.take() else {
        return Ok(());
    };
    let payload = serde_json::to_string(args).unwrap_or_else(|_| "{}".into());
    stdin
        .write_all(payload.as_bytes())
        .await
        .map_err(|e| format!("write stdin: {e}"))?;
    stdin
        .shutdown()
        .await
        .map_err(|e| format!("close stdin: {e}"))?;
    Ok(())
}

/// Resolve `<crate>` as an executable: prefer `KEI_RUNTIME_BIN_DIR/<crate>`,
/// fall back to walking `PATH`. Mirrors `kei-runtime::invoke::resolve_binary`.
fn resolve_binary(crate_name: &str) -> Option<PathBuf> {
    if let Ok(bin_dir) = std::env::var("KEI_RUNTIME_BIN_DIR") {
        let candidate = PathBuf::from(bin_dir).join(crate_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    let path = std::env::var("PATH").ok()?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(crate_name);
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}
