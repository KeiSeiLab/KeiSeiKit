-- Migration v8 — skill_invocations table (HERMES-MIGRATION-PLAN P3.4, 2026-04-27).
--
-- Tracks per-invocation outcome of a skill load into agent context.
-- Phase D's nightly self-improvement queries this table for:
--   - usage_count(skill, lookback_days)
--   - success_rate(skill, lookback_days)
--   - last_used(skill)
--   - unused_skills(days)
--
-- Schema is mirrored verbatim by the inline migration entry in
-- `src/schema.rs::MIGRATIONS[7]`. This `.sql` file is the reviewer-facing
-- artefact; the runtime source-of-truth is the const string array.

CREATE TABLE skill_invocations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    skill_name TEXT NOT NULL,
    ts INTEGER NOT NULL,
    agent_id TEXT,
    success INTEGER NOT NULL CHECK(success IN (0, 1)),
    trajectory_id TEXT,
    duration_ms INTEGER
);

CREATE INDEX idx_skill_invocations_name_ts
    ON skill_invocations(skill_name, ts DESC);

CREATE INDEX idx_skill_invocations_success
    ON skill_invocations(skill_name, success);
