-- outcome-only-schema.sql — minimal SQLite schema for the outcome-only
-- profile. Mirrors `_primitives/_rust/kei-ledger/src/migrations_list.rs`
-- but flattened: a single transaction that creates the v9-equivalent
-- shape of `agents` + `skill_invocations`, plus the v3 BEFORE-INSERT/
-- UPDATE triggers that enforce branch length ≤256.
--
-- PRAGMA user_version = 9 is set OUTSIDE the transaction (after COMMIT)
-- so it lands atomically and is portable across SQLite versions
-- (transaction-aware PRAGMA writes are documented-undefined on
-- pre-3.37 builds). The shell installer (`_outcome_install_ledger`
-- in `lib-profile-outcome-only.sh`) guards re-runs against an
-- already-upgraded DB by reading user_version BEFORE invoking this
-- file; that prevents silent downgrade if the user later runs full
-- kit which bumps schema past v9.
--
-- Two tables:
--   agents              → outcome rows (kei-model-router posterior)
--   skill_invocations   → per-skill load events (Phase D metrics)

BEGIN IMMEDIATE;

CREATE TABLE IF NOT EXISTS agents (
    id              TEXT PRIMARY KEY,
    branch          TEXT NOT NULL,
    parent_branch   TEXT,
    spec_sha        TEXT NOT NULL,
    status          TEXT NOT NULL CHECK (status IN ('running','done','failed','merged','rejected')),
    started_ts      INTEGER NOT NULL,
    finished_ts     INTEGER,
    summary         TEXT,
    worktree_path   TEXT,
    dna             TEXT,
    creator_id      TEXT,
    fork_parent_id  TEXT,
    cost_cents      INTEGER DEFAULT 0,
    provider        TEXT DEFAULT '',
    model           TEXT DEFAULT '',
    cost_micro_cents INTEGER DEFAULT 0,
    tokens_in       INTEGER,
    tokens_out      INTEGER,
    stubs_count     INTEGER DEFAULT 0,
    outcome         TEXT CHECK (outcome IS NULL OR outcome IN ('functional','partial','scaffolding','fail')),
    escalation_depth INTEGER DEFAULT 0,
    task_class_dna  TEXT GENERATED ALWAYS AS (
        CASE
            WHEN dna IS NULL OR dna = '' THEN NULL
            WHEN length(dna) > 9
                 AND substr(dna, length(dna) - 8, 1) = '-'
                 THEN substr(dna, 1, length(dna) - 9)
            ELSE dna
        END
    ) VIRTUAL
);
CREATE INDEX IF NOT EXISTS idx_parent ON agents(parent_branch);
CREATE INDEX IF NOT EXISTS idx_status ON agents(status);
CREATE INDEX IF NOT EXISTS idx_agents_dna_prefix ON agents(substr(dna, 1, 30));
CREATE UNIQUE INDEX IF NOT EXISTS idx_agents_dna_unique ON agents(dna);
CREATE INDEX IF NOT EXISTS idx_agents_creator ON agents(creator_id);
CREATE INDEX IF NOT EXISTS idx_agents_fork_parent ON agents(fork_parent_id);
CREATE INDEX IF NOT EXISTS idx_agents_task_class ON agents(task_class_dna);

CREATE TABLE IF NOT EXISTS skill_invocations (
    id            INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_name    TEXT NOT NULL,
    ts            INTEGER NOT NULL,
    agent_id      TEXT,
    success       INTEGER NOT NULL CHECK(success IN (0, 1)),
    trajectory_id TEXT,
    duration_ms   INTEGER
);
CREATE INDEX IF NOT EXISTS idx_skill_invocations_name_ts
    ON skill_invocations(skill_name, ts DESC);
CREATE INDEX IF NOT EXISTS idx_skill_invocations_success
    ON skill_invocations(skill_name, success);

-- v3 triggers — enforce branch length ≤256 chars (mirrors
-- `migrations_list.rs:30-44` v3 migration). Without these, the flat
-- schema would silently accept rows that the Rust kei-ledger flow
-- rejects, creating cross-version drift.
CREATE TRIGGER IF NOT EXISTS trg_agents_branch_len_ins
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
    END;

COMMIT;

-- PRAGMA user_version is set OUTSIDE the transaction so the write is
-- portable across SQLite versions. The shell installer guards against
-- silent downgrade by checking user_version BEFORE invoking this file.
PRAGMA user_version = 9;
