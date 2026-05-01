use rusqlite::{Connection, Result};

const DDL: &str = r#"
    CREATE TABLE IF NOT EXISTS researches (
        id              INTEGER PRIMARY KEY,
        query_original  TEXT NOT NULL,
        status          TEXT NOT NULL DEFAULT 'pending',
        result_markdown TEXT DEFAULT '',
        total_cost_mc   INTEGER DEFAULT 0,
        created_at      INTEGER NOT NULL,
        completed_at    INTEGER DEFAULT 0
    );
    CREATE INDEX IF NOT EXISTS idx_res_status ON researches(status);

    CREATE TABLE IF NOT EXISTS sources (
        id              INTEGER PRIMARY KEY,
        research_id     INTEGER NOT NULL REFERENCES researches(id),
        url             TEXT NOT NULL,
        title           TEXT DEFAULT '',
        content         TEXT DEFAULT '',
        provider        TEXT DEFAULT '',
        domain          TEXT DEFAULT '',
        relevance_score REAL DEFAULT 0.0,
        created_at      INTEGER NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_src_research ON sources(research_id);

    CREATE TABLE IF NOT EXISTS claims (
        id              INTEGER PRIMARY KEY,
        research_id     INTEGER NOT NULL REFERENCES researches(id),
        claim_text      TEXT NOT NULL,
        support         REAL DEFAULT 0.0,
        contradict      REAL DEFAULT 0.0,
        consensus       REAL DEFAULT 0.0,
        grade           TEXT DEFAULT 'E6',
        created_at      INTEGER NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_claim_research ON claims(research_id);
"#;

pub fn create_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(DDL)?;
    Ok(())
}
