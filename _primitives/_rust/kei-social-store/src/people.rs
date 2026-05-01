//! People + organizations.
//!
//! `add_person` / `get_person` delegate to `kei_entity_store::verbs::*`
//! under `SOCIAL_SCHEMA`. Organizations live in a `custom_migrations`
//! table (name-keyed upsert semantics, not generic CRUD) and keep their
//! bespoke SQL path.

use crate::schema::SOCIAL_SCHEMA;
use crate::store::Store;
use anyhow::{anyhow, Result};
use chrono::Utc;
use kei_entity_store::verbs::{create as v_create, get as v_get};
use rusqlite::params;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub handle: String,
    pub role: String,
    pub organization: String,
    pub source: String,
    pub bio: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub org_type: String,
    pub description: String,
    pub created_at: i64,
}

pub fn add_person(store: &Store, p: &Person) -> Result<i64> {
    let input = json!({
        "name": p.name,
        "email": p.email,
        "handle": p.handle,
        "role": p.role,
        "organization": p.organization,
        "source": p.source,
        "bio": p.bio,
    });
    let v = v_create::run(store.conn(), &SOCIAL_SCHEMA, input)
        .map_err(|e| anyhow!("{e}"))?;
    v["id"].as_i64().ok_or_else(|| anyhow!("missing id in create response"))
}

pub fn get_person(store: &Store, id: i64) -> Result<Option<Person>> {
    match v_get::run(store.conn(), &SOCIAL_SCHEMA, json!({ "id": id })) {
        Ok(v) => Ok(Some(person_from_json(v)?)),
        Err(e) if e.exit_code() == 2 => Ok(None),
        Err(e) => Err(anyhow!("{e}")),
    }
}

fn person_from_json(v: Value) -> Result<Person> {
    let obj = v.as_object().ok_or_else(|| anyhow!("expected object in get response"))?;
    let s = |k: &str| obj.get(k).and_then(|x| x.as_str()).unwrap_or("").to_string();
    let i = |k: &str| obj.get(k).and_then(|x| x.as_i64()).unwrap_or(0);
    Ok(Person {
        id: i("id"), name: s("name"), email: s("email"), handle: s("handle"),
        role: s("role"), organization: s("organization"), source: s("source"),
        bio: s("bio"), created_at: i("created_at"), updated_at: i("updated_at"),
    })
}

pub fn add_org(store: &Store, o: &Organization) -> Result<i64> {
    let now = Utc::now().timestamp();
    let ot = if o.org_type.is_empty() { "company" } else { &o.org_type };
    store.conn().execute(
        "INSERT OR IGNORE INTO organizations (name, org_type, description, created_at)
         VALUES (?1,?2,?3,?4)",
        params![o.name, ot, o.description, now],
    )?;
    let id: i64 = store.conn().query_row(
        "SELECT id FROM organizations WHERE name=?1",
        params![o.name], |r| r.get(0))?;
    Ok(id)
}
