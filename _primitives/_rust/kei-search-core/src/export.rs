//! Export research → markdown / JSON.

use crate::store::ResearchStore;
use anyhow::{anyhow, Result};
use serde_json::json;

pub enum Format {
    Markdown,
    Json,
}

pub fn export(store: &ResearchStore, id: i64, fmt: Format) -> Result<String> {
    let r = store.get_research(id)?.ok_or_else(|| anyhow!("research {id} missing"))?;
    let claims = store.claims_for(id)?;
    match fmt {
        Format::Markdown => {
            let mut md = String::new();
            md.push_str(&format!("# Research {}\n\n", r.id));
            md.push_str(&format!("**Query:** {}\n\n", r.query_original));
            md.push_str(&format!("**Status:** {}\n", r.status));
            md.push_str(&format!("**Cost:** {} mc\n\n", r.total_cost_mc));
            md.push_str("## Claims\n\n");
            for c in claims {
                md.push_str(&format!("- [{}] {} (consensus={:.2})\n",
                    c.grade, c.claim_text, c.consensus));
            }
            Ok(md)
        }
        Format::Json => {
            let val = json!({
                "id": r.id,
                "query": r.query_original,
                "status": r.status,
                "cost_mc": r.total_cost_mc,
                "claims": claims,
            });
            Ok(serde_json::to_string_pretty(&val)?)
        }
    }
}
