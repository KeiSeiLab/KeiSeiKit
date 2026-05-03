# LEDGER-SCHEMA — Portable Specification

> How to query `~/.claude/agents/ledger.sqlite` without the `kei-ledger` binary.
> SSoT: `_primitives/_rust/kei-ledger/src/migrations_list.rs` (schema v9, 2026-04-30).

---

## Section 1 — Schema DDL

### Table: `agents`

```sql
CREATE TABLE agents (
  -- v1 (2026-04-21, RULE 0.12)
  id           TEXT PRIMARY KEY,
  branch       TEXT NOT NULL,              -- git branch, max 256 chars
  parent_branch TEXT,                      -- NULL for root agents
  spec_sha     TEXT NOT NULL,             -- SHA of spec.md artefact
  status       TEXT NOT NULL
    CHECK (status IN ('running','done','failed','merged','rejected')),
  started_ts   INTEGER NOT NULL,          -- Unix epoch seconds (UTC)
  finished_ts  INTEGER,                   -- NULL while running
  summary      TEXT,                      -- free-text completion note
  worktree_path TEXT,                     -- absolute path to worktree

  -- v2 (2026-04-23)
  dna          TEXT UNIQUE,               -- DNA wire string (see DNA-FORMAT.md)

  -- v4 (2026-04-23)
  creator_id   TEXT,                      -- DNA/id of spawning agent
  fork_parent_id TEXT,                    -- DNA of forked-from agent

  -- v6 (2026-04-24)
  cost_cents   INTEGER DEFAULT 0,         -- whole cents (deprecated by v7)
  provider     TEXT DEFAULT '',           -- e.g. "anthropic"
  model        TEXT DEFAULT '',           -- e.g. "claude-opus-4-7"

  -- v7 (2026-04-24)
  cost_micro_cents INTEGER DEFAULT 0,    -- 1 cent = 1_000_000 micro-cents

  -- v9 (2026-04-30)
  tokens_in    INTEGER,                   -- input token count
  tokens_out   INTEGER,                   -- output token count
  stubs_count  INTEGER DEFAULT 0,        -- count of todo!()/unimplemented!()
  outcome      TEXT
    CHECK (outcome IS NULL OR
           outcome IN ('functional','partial','scaffolding','fail')),
  escalation_depth INTEGER DEFAULT 0,

  -- v9 virtual (computed, not stored)
  task_class_dna TEXT GENERATED ALWAYS AS (
    CASE WHEN dna IS NULL OR dna = '' THEN NULL
         WHEN length(dna) > 9 AND substr(dna, length(dna)-8, 1) = '-'
           THEN substr(dna, 1, length(dna)-9)
         ELSE dna END
  ) VIRTUAL
);
```

### Table: `skill_invocations` (v8, 2026-04-27)

```sql
CREATE TABLE skill_invocations (
  id           INTEGER PRIMARY KEY AUTOINCREMENT,
  skill_name   TEXT NOT NULL,
  ts           INTEGER NOT NULL,          -- Unix epoch seconds
  agent_id     TEXT,                      -- FK to agents.id (nullable)
  success      INTEGER NOT NULL
    CHECK (success IN (0, 1)),            -- 1 = loaded ok, 0 = failed
  trajectory_id TEXT,                     -- session/trace link
  duration_ms  INTEGER                    -- wall-clock load time
);
```

### Indices (both tables)

```sql
-- agents
CREATE INDEX idx_parent ON agents(parent_branch);
CREATE INDEX idx_status ON agents(status);
CREATE INDEX idx_agents_dna_prefix ON agents(substr(dna, 1, 30));
CREATE UNIQUE INDEX idx_agents_dna_unique ON agents(dna);
CREATE INDEX idx_agents_creator ON agents(creator_id);
CREATE INDEX idx_agents_fork_parent ON agents(fork_parent_id);
CREATE INDEX idx_agents_task_class ON agents(task_class_dna);
-- skill_invocations
CREATE INDEX idx_skill_invocations_name_ts ON skill_invocations(skill_name, ts DESC);
CREATE INDEX idx_skill_invocations_success ON skill_invocations(skill_name, success);
```

---

## Section 2 — Field semantics

| Column | Type | Notes |
|--------|------|-------|
| `id` | TEXT PK | UUID or slug; unique per row |
| `branch` | TEXT | Git branch; `agent/<slug>-<epoch>` convention |
| `parent_branch` | TEXT | NULL = root agent (spawned by user) |
| `spec_sha` | TEXT | SHA-256 of the `.claude/agents/<id>/spec.md` content |
| `status` | TEXT ENUM | `running` → `done`/`failed` → `merged`/`rejected` |
| `started_ts` | INTEGER | Epoch seconds UTC; set at `fork` call |
| `finished_ts` | INTEGER | Set at `done`/`fail`; NULL while running |
| `dna` | TEXT UNIQUE | See DNA-FORMAT.md; NULL for pre-v2 rows |
| `cost_micro_cents` | INTEGER | Preferred cost column since v7 |
| `outcome` | TEXT ENUM | From STATUS-TRUTH MARKER (RULE 0.16) |
| `task_class_dna` | TEXT VIRTUAL | DNA without trailing nonce; stable per task class |

---

## Section 3 — Sample queries (pure sqlite3 CLI)

Open: `sqlite3 ~/.claude/agents/ledger.sqlite`

### Query 1: all running agents
```sql
SELECT id, branch, datetime(started_ts,'unixepoch') AS started
FROM agents WHERE status = 'running' ORDER BY started_ts DESC;
```

### Query 2: agents completed in the last 24 hours

```sql
SELECT id, status, outcome, summary,
       datetime(finished_ts, 'unixepoch') AS finished
FROM agents
WHERE finished_ts > unixepoch('now') - 86400
ORDER BY finished_ts DESC;
```

### Query 3: recent failures with reason

```sql
SELECT id, branch, summary, datetime(finished_ts, 'unixepoch') AS when
FROM agents
WHERE status = 'failed'
ORDER BY finished_ts DESC
LIMIT 20;
```

### Query 4: costliest spawns by micro-cents

```sql
SELECT id, model, cost_micro_cents,
       ROUND(cost_micro_cents / 1e8, 4) AS cost_dollars,
       tokens_in, tokens_out
FROM agents
WHERE cost_micro_cents > 0
ORDER BY cost_micro_cents DESC
LIMIT 10;
```

### Query 5: status histogram

```sql
SELECT status, COUNT(*) AS n,
       ROUND(COUNT(*) * 100.0 / SUM(COUNT(*)) OVER (), 1) AS pct
FROM agents
GROUP BY status
ORDER BY n DESC;
```

### Query 6: skill success rate

```sql
SELECT skill_name,
       COUNT(*) AS invocations,
       ROUND(AVG(success) * 100, 1) AS success_pct,
       ROUND(AVG(duration_ms)) AS avg_ms
FROM skill_invocations
WHERE ts > unixepoch('now') - 7 * 86400
GROUP BY skill_name
ORDER BY invocations DESC;
```

---

## Section 4 — Migration versioning

Schema version is stored in `PRAGMA user_version` (integer, 1-based).

```sh
sqlite3 ~/.claude/agents/ledger.sqlite "PRAGMA user_version;"
# prints e.g. 9
```

Rules:
- Migrations are append-only: index `i` in `MIGRATIONS[]` → target version `i+1`.
- Never reorder or edit existing entries; add a new entry at the end.
- Each migration runs inside a `BEGIN IMMEDIATE … COMMIT` transaction.
- Partial failure rolls back the entire migration for that version; next startup retries.
- `ALTER TABLE … ADD COLUMN` is idempotent under the version gate: the runner skips versions already applied.
- v5 has a pre-check: if existing rows have duplicate non-NULL DNAs, `migrate()` returns `DnaMigrationBlocked` and refuses to proceed. Clean up duplicates before upgrading.

### Backward-compat policy

- All new columns use `DEFAULT 0` or `DEFAULT ''` or `NULL` to keep pre-migration rows valid.
- Old readers that do not know about new columns continue working; they simply ignore them.
- The `task_class_dna` VIRTUAL column requires SQLite ≥ 3.31.0 (2020-01-22). Earlier versions fail to open the database after v9 is applied.
