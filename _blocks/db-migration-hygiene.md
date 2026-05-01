# DB — Migration hygiene (universal)

Applies to every migration tool — `kei-migrate`, Atlas, goose, sqlx-cli, drizzle-kit, Alembic, Prisma migrate, Ecto migrations. [E4 — expert assessment]

**Numbering:** timestamp prefix, not integer. `20260421_120000_add_users_email_index.sql` sorts correctly forever and doesn't collide on parallel branches. Integer sequences (`0001_`, `0002_`) collide on merge; reject them in review.

**Up + down pairs:** every migration has a reverse. If the reverse is destructive and unsafe (e.g. dropping a column with data), write a `-- IRREVERSIBLE` comment and stop the down-script there. NEVER auto-run destructive downs on prod without a human click.

**Idempotent where possible:**
```sql
CREATE TABLE IF NOT EXISTS users (...);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
ALTER TABLE users ADD COLUMN IF NOT EXISTS bio TEXT;    -- PG 9.6+, verify per-DB
```
Re-running a partially-applied migration should be safe. A migration that crashes mid-way and can't be re-run = 2AM incident waiting to happen.

**Zero-downtime pattern (add-then-drop):**
1. Deploy migration that ADDS new column / table (old code still works).
2. Deploy app code that writes BOTH old + new.
3. Backfill old → new.
4. Deploy app code that reads new, ignores old.
5. Deploy migration that DROPS old column.

Never `DROP` + `ADD RENAME` in one migration on a live table. That's a table lock + app-downtime event.

**Backfill patterns:**
- Small table (< 1M rows): `UPDATE ... SET new = f(old)` in a single migration.
- Large table: background job with batched `UPDATE ... WHERE id BETWEEN ? AND ?` + `LIMIT`. Commit per batch. Monitor lag.
- Very large (> 100M rows): use the DB's native tooling (PG `VACUUM FULL` not needed; `pg_repack` if column-add bloats). [UNVERIFIED: verify on current PG docs]

**Tracking table (`_kei_migrations` or equivalent):** stores (version, name, checksum, applied_at). Checksum prevents silent tampering with an already-applied file. If checksum mismatches on an applied migration → hard-fail, demand human intervention.

**Forbidden:** editing a migration file after it's been applied on any environment (checksum break); `DROP TABLE` without backup + 24h cooldown; mixing DDL + large DML in one transaction (long locks); running migrations automatically on app startup in multi-replica deploys without a leader-election guard (every replica tries to apply = race condition).
