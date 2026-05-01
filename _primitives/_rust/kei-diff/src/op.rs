//! Patch operation types + RFC 6902 JSON serialization.
//!
//! We emit only the minimal trio (`add`, `remove`, `replace`). Custom Serialize
//! keeps the wire format stable and self-documenting (no need for serde tag
//! gymnastics).

use serde::de::{self, MapAccess, Visitor};
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;

/// A single RFC 6902 patch operation (subset).
#[derive(Debug, Clone, PartialEq)]
pub enum Op {
    Add { path: String, value: Value },
    Remove { path: String },
    Replace { path: String, value: Value },
}

/// An ordered list of `Op`s. Serializes as a JSON array per RFC 6902.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Patch(pub Vec<Op>);

impl Patch {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn push(&mut self, op: Op) {
        self.0.push(op);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl Serialize for Op {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Op::Add { path, value } => {
                let mut m = s.serialize_map(Some(3))?;
                m.serialize_entry("op", "add")?;
                m.serialize_entry("path", path)?;
                m.serialize_entry("value", value)?;
                m.end()
            }
            Op::Remove { path } => {
                let mut m = s.serialize_map(Some(2))?;
                m.serialize_entry("op", "remove")?;
                m.serialize_entry("path", path)?;
                m.end()
            }
            Op::Replace { path, value } => {
                let mut m = s.serialize_map(Some(3))?;
                m.serialize_entry("op", "replace")?;
                m.serialize_entry("path", path)?;
                m.serialize_entry("value", value)?;
                m.end()
            }
        }
    }
}

impl Serialize for Patch {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut seq = s.serialize_seq(Some(self.0.len()))?;
        for op in &self.0 {
            seq.serialize_element(op)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for Op {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        d.deserialize_map(OpVisitor)
    }
}

impl<'de> Deserialize<'de> for Patch {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Vec::<Op>::deserialize(d).map(Patch)
    }
}

struct OpVisitor;

impl<'de> Visitor<'de> for OpVisitor {
    type Value = Op;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("an RFC 6902 JSON Patch operation object")
    }

    fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Op, M::Error> {
        let mut op: Option<String> = None;
        let mut path: Option<String> = None;
        let mut value: Option<Value> = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "op" => op = Some(map.next_value()?),
                "path" => path = Some(map.next_value()?),
                "value" => value = Some(map.next_value()?),
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }
        let op = op.ok_or_else(|| de::Error::missing_field("op"))?;
        let path = path.ok_or_else(|| de::Error::missing_field("path"))?;
        build_op::<M::Error>(&op, path, value)
    }
}

fn build_op<E: de::Error>(op: &str, path: String, value: Option<Value>) -> Result<Op, E> {
    match op {
        "add" => Ok(Op::Add {
            path,
            value: value.ok_or_else(|| E::missing_field("value"))?,
        }),
        "remove" => Ok(Op::Remove { path }),
        "replace" => Ok(Op::Replace {
            path,
            value: value.ok_or_else(|| E::missing_field("value"))?,
        }),
        other => Err(E::unknown_variant(other, &["add", "remove", "replace"])),
    }
}
