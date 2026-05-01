# DB — SQLite (prod-suitable) patterns

Use when the workload is read-heavy, single-writer-acceptable, or needs zero-ops embedded storage. SQLite is prod-suitable — Fly.io, Turso, Cloudflare D1, and countless CLI/mobile apps run it in production. [E4 — expert assessment]

**When NOT to use:** high-concurrency write workload (> ~1 writer/sec sustained), multi-region strong consistency, horizontal write scaling. Use Postgres instead.

**WAL mode is mandatory for prod:**
```sql
PRAGMA journal_mode = WAL;         -- readers don't block writer, writer doesn't block readers
PRAGMA synchronous = NORMAL;       -- durable across app crash, NOT across power loss (use FULL if PSU-risk)
PRAGMA busy_timeout = 5000;        -- 5s wait for lock instead of instant SQLITE_BUSY
PRAGMA foreign_keys = ON;          -- default OFF in SQLite (!), always enable
PRAGMA temp_store = MEMORY;
```
Apply these on every connection open — they are per-connection, not per-database (except `journal_mode` which persists).

**Distributed patterns:**
- **Turso** (libSQL fork): edge-replicated read replicas with HTTP/WebSocket wire protocol. Primary single-writer, replicas read-only. [E4]
- **LiteFS** (Fly.io): file-system replication, leader-election via Consul. Primary+replicas. [E4]
- **Cloudflare D1**: managed SQLite on edge with their own replication. [UNVERIFIED: current throughput limits]
- **Litestream**: continuous replication to S3/R2 for backup + PITR; single node, not HA.

**Full-text search (FTS5):**
```sql
CREATE VIRTUAL TABLE docs_fts USING fts5(title, body, content=docs, content_rowid=id);
CREATE TRIGGER docs_ai AFTER INSERT ON docs BEGIN
  INSERT INTO docs_fts(rowid, title, body) VALUES (new.id, new.title, new.body);
END;
```
FTS5 outperforms bolt-on `LIKE '%x%'` by 100×+ on large text corpora. Native, no extension install.

**Backup:** `sqlite3 db '.backup /path/backup.db'` while app runs (safe with WAL). Or Litestream for continuous.

**Forbidden:** multiple writer processes without a coordination layer; opening the same DB over NFS (lock semantics broken); `DELETE FROM bigtable` without `VACUUM` after (doesn't shrink file); committing the `.db` / `.db-wal` / `.db-shm` files to git.
