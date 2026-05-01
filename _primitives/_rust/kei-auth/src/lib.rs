//! kei-auth — multi-tenant token auth. Replaces LBM's single LBM_MCP_TOKEN.
//!
//! Cubes:
//!   - [`schema`] — SQLite tables for users + tokens
//!   - [`hmac`] — HMAC-SHA256 signing helpers
//!   - [`tokens`] — issue / verify / revoke / list
//!   - [`scopes`] — read / write / admin enum + checks

pub mod hmac;
pub mod schema;
pub mod scopes;
pub mod tokens;

pub use scopes::Scope;
pub use tokens::{issue, revoke, verify, VerifyOutcome};
