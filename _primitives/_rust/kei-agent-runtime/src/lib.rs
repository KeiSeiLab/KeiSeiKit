//! kei-agent-runtime — Agent substrate v1 runtime.
//!
//! Modules:
//!   - `capability` — Capability trait + context structs + result enums
//!   - `registry`   — static &str → &'static dyn Capability lookup for all 14 impls
//!   - `gates`      — 6 PreToolUse gate capabilities
//!   - `verifies`   — 8 on-return verify capabilities
//!   - `compose`    — task.toml + role + capabilities → prompt.md
//!   - `spawn`      — prepare tasks/<agent-id>/prompt.md + ledger row
//!   - `prepare`    — orchestrator-facing `AgentInvocation` bundle (ergonomics)
//!   - `verify`     — run all verify capabilities against agent's return
//!   - `simulated_merge` — orchestrator-side worktree → apply diff → verify
//!
//! Per `docs/AGENT-SUBSTRATE-SCHEMA.md` (LOCKED 2026-04-23).

pub mod capability;
pub mod compose;
pub mod dna;
pub mod gates;
pub mod prepare;
pub mod registry;
pub mod role;
pub mod simulated_merge;
pub mod spawn;
pub mod validate;
pub mod verifies;
pub mod verify;
