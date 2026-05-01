use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scope {
    Read,
    Write,
    Admin,
}

impl Scope {
    pub fn as_str(&self) -> &'static str {
        match self { Scope::Read => "read", Scope::Write => "write", Scope::Admin => "admin" }
    }

    /// Admin ⊇ Write ⊇ Read.
    pub fn allows(&self, required: Scope) -> bool {
        use Scope::*;
        match (self, required) {
            (Admin, _) => true,
            (Write, Read) | (Write, Write) => true,
            (Read, Read) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Scope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Scope {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "read" => Ok(Scope::Read),
            "write" => Ok(Scope::Write),
            "admin" => Ok(Scope::Admin),
            _ => Err(format!("unknown scope: {s}")),
        }
    }
}
