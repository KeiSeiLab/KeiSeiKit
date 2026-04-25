//! Anthropic tool-use schema definitions.
//!
//! `tool_definitions()` returns the JSON array the daemon sends to
//! Anthropic in the `tools` field of `messages.create`. Each entry is a
//! `{name, description, input_schema}` object. The `input_schema` is a
//! JSON-Schema describing what the model must emit in `tool_use.input`.
//!
//! Constructor Pattern: schema-only module, no executor logic. Each tool
//! is one small builder fn (≤30 LOC) so additions stay surgical.

use serde_json::{json, Value};

/// All 8 tool definitions. Order matches the registry default.
pub fn tool_definitions() -> Vec<Value> {
    vec![
        read_def(),
        write_def(),
        edit_def(),
        bash_def(),
        glob_def(),
        grep_def(),
        webfetch_def(),
        agent_def(),
    ]
}

fn read_def() -> Value {
    json!({
        "name": "read",
        "description": "Read a file from disk. Returns the file contents with line numbers.",
        "input_schema": {
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Absolute path to read." },
                "offset": { "type": "integer", "description": "Optional 1-indexed start line." },
                "limit": { "type": "integer", "description": "Optional max lines to return." }
            },
            "required": ["path"]
        }
    })
}

fn write_def() -> Value {
    json!({
        "name": "write",
        "description": "Write a file to disk (atomic via tempfile + rename).",
        "input_schema": {
            "type": "object",
            "properties": {
                "path": { "type": "string", "description": "Absolute destination path." },
                "content": { "type": "string", "description": "File contents." }
            },
            "required": ["path", "content"]
        }
    })
}

fn edit_def() -> Value {
    json!({
        "name": "edit",
        "description": "Replace `old_string` with `new_string` in a file. Old string must be unique unless replace_all is true.",
        "input_schema": {
            "type": "object",
            "properties": {
                "path": { "type": "string" },
                "old_string": { "type": "string" },
                "new_string": { "type": "string" },
                "replace_all": { "type": "boolean", "default": false }
            },
            "required": ["path", "old_string", "new_string"]
        }
    })
}

fn bash_def() -> Value {
    json!({
        "name": "bash",
        "description": "Execute a shell command (60s timeout). Some destructive patterns are denied.",
        "input_schema": {
            "type": "object",
            "properties": {
                "command": { "type": "string" }
            },
            "required": ["command"]
        }
    })
}

fn glob_def() -> Value {
    json!({
        "name": "glob",
        "description": "Find files by glob pattern (e.g. **/*.rs). Sorted by mtime desc, capped at 100.",
        "input_schema": {
            "type": "object",
            "properties": {
                "pattern": { "type": "string" },
                "path": { "type": "string", "description": "Optional search root, defaults to cwd." }
            },
            "required": ["pattern"]
        }
    })
}

fn grep_def() -> Value {
    json!({
        "name": "grep",
        "description": "Search file contents with a regex. Returns matching files OR matching lines depending on output_mode.",
        "input_schema": {
            "type": "object",
            "properties": {
                "pattern": { "type": "string" },
                "path": { "type": "string" },
                "glob": { "type": "string", "description": "Restrict to files matching this glob." },
                "output_mode": { "type": "string", "enum": ["files_with_matches", "content"] }
            },
            "required": ["pattern"]
        }
    })
}

fn webfetch_def() -> Value {
    json!({
        "name": "webfetch",
        "description": "Fetch a URL and return its text content (HTML stripped to readable text). 30s timeout, 15-min cache.",
        "input_schema": {
            "type": "object",
            "properties": {
                "url": { "type": "string" },
                "prompt": { "type": "string", "description": "What to look for (passed to caller, not used inside fetch)." }
            },
            "required": ["url"]
        }
    })
}

fn agent_def() -> Value {
    json!({
        "name": "agent",
        "description": "Launch a sub-agent for a focused subtask. Returns the agent's final message.",
        "input_schema": {
            "type": "object",
            "properties": {
                "description": { "type": "string", "description": "One-line task summary." },
                "prompt": { "type": "string", "description": "Detailed instructions." },
                "subagent_type": { "type": "string", "description": "Optional manifest slug." }
            },
            "required": ["description", "prompt"]
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_eight_definitions() {
        let defs = tool_definitions();
        assert_eq!(defs.len(), 8);
    }

    #[test]
    fn each_def_has_required_fields() {
        for d in tool_definitions() {
            assert!(d.get("name").and_then(|v| v.as_str()).is_some());
            assert!(d.get("description").is_some());
            assert!(d.get("input_schema").is_some());
        }
    }

    #[test]
    fn names_match_registry_defaults() {
        let defs = tool_definitions();
        let names: Vec<&str> = defs
            .iter()
            .filter_map(|d| d.get("name").and_then(|v| v.as_str()))
            .collect();
        assert_eq!(
            names,
            vec!["read", "write", "edit", "bash", "glob", "grep", "webfetch", "agent"]
        );
    }
}
