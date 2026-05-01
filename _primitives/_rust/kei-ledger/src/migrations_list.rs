//! Ordered migration DDL list — single source of truth for `agents`
//! table evolution + the v8 `skill_invocations` companion table.
//!
//! Constructor Pattern: extracted from `schema.rs` so the runner cube
//! stays under the 200-LOC ceiling. NEVER reorder; append only. Index =
//! schema version (1-based via `MIGRATIONS[i]`, target = i+1).

/// Ordered migrations. Index = schema version (1-based; target = i+1).
pub const MIGRATIONS: &[&str] = &[
    // v1 — initial schema (RULE 0.12, 2026-04-21)
    "CREATE TABLE IF NOT EXISTS agents (
        id TEXT PRIMARY KEY,
        branch TEXT NOT NULL,
        parent_branch TEXT,
        spec_sha TEXT NOT NULL,
        status TEXT NOT NULL CHECK (status IN ('running','done','failed','merged','rejected')),
        started_ts INTEGER NOT NULL,
        finished_ts INTEGER,
        summary TEXT,
        worktree_path TEXT
    );
    CREATE INDEX IF NOT EXISTS idx_parent ON agents(parent_branch);
    CREATE INDEX IF NOT EXISTS idx_status ON agents(status);",
    // v2 — Layer G DNA identity column + prefix index (2026-04-23)
    "ALTER TABLE agents ADD COLUMN dna TEXT;
    CREATE INDEX IF NOT EXISTS idx_agents_dna_prefix ON agents(substr(dna, 1, 30));",
    // v3 — length caps on branch/parent_branch (audit L1, 2026-04-23)
    // Triggers (not table CHECK) because CHECK can't be retro-added without
    // rebuilding the table. They refuse over-long inserts/updates.
    "CREATE TRIGGER IF NOT EXISTS trg_agents_branch_len_ins
     BEFORE INSERT ON agents
     BEGIN
        SELECT RAISE(ABORT, 'branch length exceeds 256')
            WHERE length(NEW.branch) > 256;
        SELECT RAISE(ABORT, 'parent_branch length exceeds 256')
            WHERE NEW.parent_branch IS NOT NULL AND length(NEW.parent_branch) > 256;
     END;
     CREATE TRIGGER IF NOT EXISTS trg_agents_branch_len_upd
     BEFORE UPDATE OF branch, parent_branch ON agents
     BEGIN
        SELECT RAISE(ABORT, 'branch length exceeds 256')
            WHERE length(NEW.branch) > 256;
        SELECT RAISE(ABORT, 'parent_branch length exceeds 256')
            WHERE NEW.parent_branch IS NOT NULL AND length(NEW.parent_branch) > 256;
     END;",
    // v4 — creator_id + fork_parent_id lineage columns (RULE 0.12 ext,
    // 2026-04-23). Both nullable for backward-compat with pre-v4 rows.
    "ALTER TABLE agents ADD COLUMN creator_id TEXT;
    ALTER TABLE agents ADD COLUMN fork_parent_id TEXT;
    CREATE INDEX IF NOT EXISTS idx_agents_creator ON agents(creator_id);
    CREATE INDEX IF NOT EXISTS idx_agents_fork_parent ON agents(fork_parent_id);",
    // v5 — UNIQUE on dna (2026-04-23). Pre-check in `migrate()` aborts with
    // `LedgerError::DnaMigrationBlocked` if existing rows already conflict.
    "CREATE UNIQUE INDEX IF NOT EXISTS idx_agents_dna_unique ON agents(dna);",
    // v6 — cost-tracking columns for /usage endpoint (Wave 40, 2026-04-24).
    // ALTER ADD COLUMN is idempotent only via the user_version gate — re-running
    // the literal DDL would error with "duplicate column name". The runner
    // skips this entry on a >=v6 ledger because `current >= 6` short-circuits.
    "ALTER TABLE agents ADD COLUMN cost_cents INTEGER DEFAULT 0;
    ALTER TABLE agents ADD COLUMN provider TEXT DEFAULT '';
    ALTER TABLE agents ADD COLUMN model TEXT DEFAULT '';",
    // v7 — micro-cents accumulator (Wave 44c, 2026-04-24).
    // 1 cent = 1_000_000 micro-cents. Pre-v7 rows backfill from existing
    // `cost_cents`. Idempotent under the user_version gate identical to v6.
    "ALTER TABLE agents ADD COLUMN cost_micro_cents INTEGER DEFAULT 0;
    UPDATE agents SET cost_micro_cents = COALESCE(cost_cents, 0) * 1000000
        WHERE cost_micro_cents IS NULL OR cost_micro_cents = 0;",
    // v8 — skill_invocations table for Phase D metrics
    // (HERMES-MIGRATION-PLAN P3.4, 2026-04-27). Mirrors
    // `src/migrations/0008_skill_invocations.sql` verbatim. Tracks per-call
    // outcome of a skill load (success 0/1, agent context, trajectory link,
    // wall-time). Phase D nightly job aggregates by `skill_name` to drive
    // archive / re-extract / `stability: validated` decisions.
    "CREATE TABLE IF NOT EXISTS skill_invocations (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        skill_name TEXT NOT NULL,
        ts INTEGER NOT NULL,
        agent_id TEXT,
        success INTEGER NOT NULL CHECK(success IN (0, 1)),
        trajectory_id TEXT,
        duration_ms INTEGER
    );
    CREATE INDEX IF NOT EXISTS idx_skill_invocations_name_ts
        ON skill_invocations(skill_name, ts DESC);
    CREATE INDEX IF NOT EXISTS idx_skill_invocations_success
        ON skill_invocations(skill_name, success);",
    // v9 — kei-model-router posterior columns (RULE 0.16 + future router
    // intelligence, 2026-04-30). All nullable / defaulted for backward
    // compat. `task_class_dna` is a VIRTUAL generated column that strips
    // the trailing `-<nonce8>` from `dna`, giving a stable per-task-class
    // identity for empirical posterior aggregation. Idempotent under
    // user_version gate.
    "ALTER TABLE agents ADD COLUMN tokens_in INTEGER;
    ALTER TABLE agents ADD COLUMN tokens_out INTEGER;
    ALTER TABLE agents ADD COLUMN stubs_count INTEGER DEFAULT 0;
    ALTER TABLE agents ADD COLUMN outcome TEXT
        CHECK (outcome IS NULL OR outcome IN ('functional','partial','scaffolding','fail'));
    ALTER TABLE agents ADD COLUMN escalation_depth INTEGER DEFAULT 0;
    ALTER TABLE agents ADD COLUMN task_class_dna TEXT
        GENERATED ALWAYS AS (
            CASE
                WHEN dna IS NULL OR dna = '' THEN NULL
                WHEN length(dna) > 9
                     AND substr(dna, length(dna) - 8, 1) = '-'
                     THEN substr(dna, 1, length(dna) - 9)
                ELSE dna
            END
        ) VIRTUAL;
    CREATE INDEX IF NOT EXISTS idx_agents_task_class
        ON agents(task_class_dna);",
];
