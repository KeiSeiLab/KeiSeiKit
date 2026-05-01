use crate::schema::create_schema;
use crate::types::{Claim, Research, Source};
use anyhow::{Context, Result};
use chrono::Utc;
use rusqlite::{params, Connection};
use std::path::Path;

pub struct ResearchStore {
    conn: Connection,
}

impl ResearchStore {
    pub fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let conn = Connection::open(path).context("open sqlite")?;
        conn.pragma_update(None, "journal_mode", "WAL").ok();
        create_schema(&conn)?;
        Ok(Self { conn })
    }

    pub fn open_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        create_schema(&conn)?;
        Ok(Self { conn })
    }

    pub fn conn(&self) -> &Connection { &self.conn }

    pub fn create_research(&self, query: &str) -> Result<i64> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "INSERT INTO researches (query_original, status, created_at)
             VALUES (?1, 'running', ?2)",
            params![query, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn set_status(&self, id: i64, status: &str) -> Result<()> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "UPDATE researches SET status=?1, completed_at=?2 WHERE id=?3",
            params![status, now, id],
        )?;
        Ok(())
    }

    pub fn set_cost(&self, id: i64, mc: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE researches SET total_cost_mc=?1 WHERE id=?2",
            params![mc, id],
        )?;
        Ok(())
    }

    pub fn set_markdown(&self, id: i64, md: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE researches SET result_markdown=?1 WHERE id=?2",
            params![md, id],
        )?;
        Ok(())
    }

    pub fn get_research(&self, id: i64) -> Result<Option<Research>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, query_original, status, result_markdown, total_cost_mc,
                    created_at, completed_at FROM researches WHERE id=?1",
        )?;
        let mut rows = stmt.query(params![id])?;
        if let Some(r) = rows.next()? {
            return Ok(Some(Research {
                id: r.get(0)?, query_original: r.get(1)?, status: r.get(2)?,
                result_markdown: r.get(3)?, total_cost_mc: r.get(4)?,
                created_at: r.get(5)?, completed_at: r.get(6)?,
            }));
        }
        Ok(None)
    }

    pub fn add_source(&self, s: &Source) -> Result<i64> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "INSERT INTO sources (research_id, url, title, content, provider,
                                  domain, relevance_score, created_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
            params![s.research_id, s.url, s.title, s.content, s.provider,
                s.domain, s.relevance_score, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn add_claim(&self, c: &Claim) -> Result<i64> {
        let now = Utc::now().timestamp();
        self.conn.execute(
            "INSERT INTO claims (research_id, claim_text, support, contradict,
                                 consensus, grade, created_at)
             VALUES (?1,?2,?3,?4,?5,?6,?7)",
            params![c.research_id, c.claim_text, c.support, c.contradict,
                c.consensus, c.grade, now],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn claims_for(&self, research_id: i64) -> Result<Vec<Claim>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, research_id, claim_text, support, contradict,
                    consensus, grade, created_at FROM claims WHERE research_id=?1"
        )?;
        let rows = stmt.query_map(params![research_id], |r| {
            Ok(Claim {
                id: r.get(0)?, research_id: r.get(1)?, claim_text: r.get(2)?,
                support: r.get(3)?, contradict: r.get(4)?, consensus: r.get(5)?,
                grade: r.get(6)?, created_at: r.get(7)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows { out.push(r?); }
        Ok(out)
    }
}
