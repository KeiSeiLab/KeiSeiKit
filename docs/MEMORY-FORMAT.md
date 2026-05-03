# MEMORY-FORMAT — Portable Specification

> How to read `~/.claude/memory/` WITHOUT the `kei-memory` binary.
> All formats are newline-delimited JSON (JSONL) or SQLite.
> Derived from source: `_primitives/_rust/kei-memory/src/` (2026-05-02).

---

## Section 1 — Event JSONL (Claude Code trace format)

Each session produces one `.jsonl` file in `~/.claude/memory/traces/`.
Two wire formats coexist; `kei-memory` handles both.

### 1a. Real Claude Code trace (primary format, 2026-04-30+)

```jsonc
{
  "type":       "assistant" | "user" | "system" | "result",
  "timestamp":  "2026-04-30T18:27:10Z",   // RFC-3339, UTC
  "sessionId":  "bf053cbd-...",            // UUID4, groups all lines in session
  "cwd":        "/Users/x/Projects/Foo",  // working directory at event time
  "gitBranch":  "main",
  "uuid":       "u1",                      // event UUID
  "parentUuid": "u0",                      // preceding event UUID (chain)
  "subtype":    "tool_use" | "tool_result" | null,
  "message": {
    "role":    "assistant" | "user",
    "content": [ /* ContentBlock array — see §1c */ ]
  },
  "toolUseID":     "toolu_...",   // present on tool_result lines
  "toolUseResult": { ... }        // present on tool_result lines
}
```

### 1b. Legacy KeiSeiKit flat format (back-compat)

```jsonc
{
  "ts":          1700000000,     // Unix epoch seconds (integer)
  "kind":        "tool_use" | "tool_result" | "other",
  "tool":        "Bash" | "Read" | "Edit" | "Write" | ...,
  "file_path":   "/abs/path/file.rs",
  "is_error":    false,
  "event_class": "tool_use:Read",  // pre-classified string
  "message":     "stdout text"
}
```

### 1c. ContentBlock inside `message.content`

```jsonc
// tool_use block (in assistant messages)
{ "type": "tool_use", "id": "t1", "name": "Read", "input": {"file_path": "/a"} }

// tool_result block (in user messages)
{ "type": "tool_result", "tool_use_id": "t1", "content": "...", "is_error": false }
```

### 1d. event_class labels (assigned by classifier)

`tool_use:<Name>` — assistant tool call  |  `tool_result` — user result  |  `tool_error[:<Name>]` — is_error=true  |  `permission_denied` — message matches `/permission\s+denied/i`  |  `user_correction` — `again`, `опять`, `stop doing`  |  `worktree_error` — `worktree.*(error|denied|fail)`  |  `cargo_workspace` — `cargo.*workspace`  |  `retry_loop` — `retry|retrying|attempt \d+`  |  `<kind>` — type field fallback  |  `other` — default

---

## Section 2 — Time-metrics journals (RULE 0.18)

Path: `~/.claude/memory/time-metrics/{sessions,tasks,numeric-claims}.jsonl`

### sessions.jsonl

```jsonc
{
  "kind":        "session",
  "id":          "bf053cbd-a6f8-47a6-a80f-11b829d63980",  // Claude Code sessionId
  "start_epoch": 1777473449,   // Unix epoch seconds
  "end_epoch":   1777473560,
  "duration_s":  111,
  "ts":          "2026-04-29T14:39:20Z"  // RFC-3339 wall-clock
}
```

### tasks.jsonl

```jsonc
{
  "kind":        "task" | "start" | "stop",
  "name":        "wave10-agent-decomposition",
  "start_epoch": 1777438080,
  "end_epoch":   1777438665,
  "duration_s":  585,
  "exit":        0,             // present on "stop" records
  "metric": {                   // optional, task-specific counters
    "new_atomars": 25,
    "new_rules":   1
  },
  "source": "~/.claude/agents/_manifests/",
  "ts":     "2026-04-29T04:57:45Z"
}
```

### numeric-claims.jsonl

```jsonc
{
  "kind":          "claim",
  "value":         "wave 5 took 18 min",
  "evidence_tier": "REAL" | "FROM-JOURNAL" | "ESTIMATE-HTC",
  "pointer":       "tasks.jsonl#wave5-2026-04-29",
  "ts":            "2026-04-29T05:00:00Z",
  "session_id":    "bf053cbd-..."  // optional
}
```

---

## Section 3 — Reading with jq

**Q1: all `tool_use:Bash` events since 2026-04-25**
```sh
cat ~/.claude/memory/traces/*.jsonl \
  | jq -c 'select(.event_class == "tool_use:Bash")
           | select((.timestamp // "") >= "2026-04-25")'
```
**Q2: median event-count per sessionId**
```sh
cat ~/.claude/memory/traces/*.jsonl \
  | jq -s 'group_by(.sessionId // .session_id)
           | map({s: .[0].sessionId, n: length}) | sort_by(.n) | .[length/2|floor]'
```
**Q3: errors-per-session timeline**
```sh
cat ~/.claude/memory/traces/*.jsonl \
  | jq -c 'select(.is_error == true or (.event_class // "" | startswith("tool_error")))
           | {session: (.sessionId // .session_id), ts: (.timestamp // .ts)}' \
  | sort | uniq -c
```
**Q4: median duration_s for tasks containing "audit"**
```sh
jq -s '[.[] | select(.kind=="task" and (.name|test("audit"))) | .duration_s]
       | sort | .[length/2|floor]' ~/.claude/memory/time-metrics/tasks.jsonl
```
**Q5: token cost outliers (v9 ledger)**
```sh
sqlite3 ~/.claude/agents/ledger.sqlite \
  "SELECT id, COALESCE(tokens_in,0)+COALESCE(tokens_out,0) AS tok
   FROM agents WHERE tok > 100000 ORDER BY tok DESC LIMIT 20;"
```

---

## Section 4 — Reading with pandas

```python
import pandas as pd, json, pathlib, glob

def load_traces(pattern="~/.claude/memory/traces/*.jsonl"):
    rows = [json.loads(l) for f in glob.glob(str(pathlib.Path(pattern).expanduser()))
            for l in open(f) if l.strip()]
    return pd.DataFrame(rows)

# Recipe 1: event class frequency
print(load_traces()["event_class"].value_counts().head(20))

# Recipe 2: sessions.jsonl duration histogram
sess = pd.read_json("~/.claude/memory/time-metrics/sessions.jsonl", lines=True)
sess[sess.kind == "session"]["duration_s"].hist(bins=30)

# Recipe 3: tasks median duration by name prefix
tasks = pd.read_json("~/.claude/memory/time-metrics/tasks.jsonl", lines=True)
tasks[tasks.kind == "task"].groupby(
    tasks["name"].str.split("-").str[0])["duration_s"].median()
```

---

## Section 5 — Reading with awk (streaming)

### Recipe 1: count events per event_class (no jq required)

```sh
cat ~/.claude/memory/traces/*.jsonl \
  | awk -F'"event_class":"' 'NF>1 {split($2,a,"\""); counts[a[1]]++}
    END {for (k in counts) print counts[k], k}' \
  | sort -rn | head -20
```

### Recipe 2: extract all Bash commands from traces

```sh
cat ~/.claude/memory/traces/*.jsonl \
  | awk -F'"command":"' 'NF>1 {
      split($2, a, "\"")
      gsub(/\\n/, "\n", a[1])
      print substr(a[1], 1, 120)
    }'
```
