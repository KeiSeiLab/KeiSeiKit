//! FTS search over `people` — routes through `kei_entity_store::verbs::search`.

use crate::people::Person;
use crate::schema::SOCIAL_SCHEMA;
use crate::store::Store;
use anyhow::{anyhow, Result};
use kei_entity_store::verbs::search as v_search;
use serde_json::{json, Value};

pub fn search_people(store: &Store, q: &str, limit: i64) -> Result<Vec<Person>> {
    let input = json!({ "query": q, "limit": if limit <= 0 { 20 } else { limit } });
    let v = v_search::run(store.conn(), &SOCIAL_SCHEMA, input)
        .map_err(|e| anyhow!("{e}"))?;
    let results = v.get("results").and_then(|r| r.as_array())
        .ok_or_else(|| anyhow!("missing results array"))?;
    results.iter().map(person_from_value).collect()
}

fn person_from_value(v: &Value) -> Result<Person> {
    let obj = v.as_object().ok_or_else(|| anyhow!("expected object"))?;
    let s = |k: &str| obj.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let i = |k: &str| obj.get(k).and_then(|x| x.as_i64()).unwrap_or(0);
    Ok(Person {
        id: i("id"), name: s("name"), email: s("email"), handle: s("handle"),
        role: s("role"), organization: s("organization"), source: s("source"),
        bio: s("bio"), created_at: i("created_at"), updated_at: i("updated_at"),
    })
}
