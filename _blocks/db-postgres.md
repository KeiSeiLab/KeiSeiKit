# DB — PostgreSQL (current major — 17 as of 2026-04) patterns

Use when the project needs relational integrity, concurrent writes, or server-side indexing power that SQLite can't match. Default RDBMS for new multi-user services. [E4 — expert assessment]

**Version choice:** PostgreSQL 17 for new projects (current GA line, improved vacuum, JSON_TABLE, better parallel index builds). PostgreSQL 16 acceptable if hosting provider pins it. [UNVERIFIED: exact feature matrix — verify on postgresql.org/docs before committing to a minor-version-specific feature]

**Schema migrations:** every schema change ships as a numbered `.sql` file, never `ALTER TABLE` on prod. Use `kei-migrate` (this kit) or Atlas/goose/sqlx-cli — see `db-migration-hygiene.md`. One migration per logical change; no mega-migrations.

**Indexing:**
- B-tree default for equality + range. `CREATE INDEX CONCURRENTLY` on prod to avoid table lock.
- `GIN` for `jsonb` / array / full-text (`tsvector`).
- `BRIN` only for massive append-only time-series (orders of magnitude smaller than B-tree).
- Partial indexes (`WHERE active = true`) for sparse predicates.
- **Verify with `EXPLAIN (ANALYZE, BUFFERS)`** before declaring an index necessary. No blind indexing.

**Connection pooling:** app-side connection pool is NOT enough at scale. Use:
- **PgBouncer** (transaction mode) for most services — battle-tested, low overhead.
- **Supavisor** if already on Supabase — serverless-friendly, wire-compatible. [E4]
- Native server pooling (PG 17's improved but still not a substitute). [UNVERIFIED]

Sizing rule of thumb: `max_connections` on server × 1 pool layer. Don't stack pools (pool → PgBouncer → PG = deadlock risk).

**Backup:**
- Logical: `pg_dump` nightly for schema + data portability.
- Physical: `pg_basebackup` + WAL archiving (`archive_command`) for PITR.
- Managed service (RDS / Supabase / Neon) — verify backup retention in their UI, don't assume.

**Forbidden:** `SELECT *` in hot paths (N+1 + column drift); unindexed FK columns (join explosion); `SERIAL` on new tables — prefer `GENERATED ALWAYS AS IDENTITY` (SQL standard, PG 10+); plaintext passwords in `pg_hba.conf`; committing `.env` with DB URL.
