//! Emit path — decide WHICH canonical KeiSeiKit shape to render the
//! imported skill into, then dispatch to the matching emitter.

pub mod as_atom;
pub mod as_primitive;
pub mod as_recipe;

use crate::canonical::{AtomCallKind, ImportedSkill};

/// Three target shapes the importer can emit into.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitPath {
    /// Standalone atom — `_primitives/_rust/<crate>/atoms/<verb>.md`.
    Atom,
    /// Recipe DAG — `recipes/<name>.toml`.
    Recipe,
    /// Proposed primitive — `_primitives/proposed/<name>.md`.
    Primitive,
}

impl EmitPath {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmitPath::Atom => "atom",
            EmitPath::Recipe => "recipe",
            EmitPath::Primitive => "primitive",
        }
    }
}

/// Decision logic — chooses an emit path based on detected atom-call
/// shape and body complexity. See `lib.rs::import` for the upstream
/// classification step that populates `phase.atom_calls`.
///
/// Rules (first match wins):
///  1. 0-1 atom_calls AND body < 2 KiB → `Atom`
///  2. ≥ 2 atom_calls AND ALL atom_ids resolved → `Recipe`
///  3. ≥ 2 atom_calls AND ANY unresolved (kei-primitive without registry hit) → `Primitive`
///  4. 0 atom_calls AND any bash code-fence detected → `Primitive` (bash-only wrapper)
///  5. Default fallback → `Primitive`
pub fn decide_emit_path(skill: &ImportedSkill) -> EmitPath {
    let total = skill.total_atom_calls();
    let resolved = skill.resolved_atom_calls();
    let body_bytes = skill.body_bytes();
    let has_bash = skill
        .phases
        .iter()
        .flat_map(|p| p.atom_calls.iter())
        .any(|c| c.kind == AtomCallKind::Bash);
    let has_kei_unresolved = skill
        .phases
        .iter()
        .flat_map(|p| p.atom_calls.iter())
        .any(|c| c.kind == AtomCallKind::KeiPrimitive && c.atom_id.is_none());

    if total <= 1 && body_bytes < 2048 {
        return EmitPath::Atom;
    }
    if total >= 2 && total == resolved && !has_kei_unresolved {
        return EmitPath::Recipe;
    }
    if total >= 2 && has_kei_unresolved {
        return EmitPath::Primitive;
    }
    if total == 0 && has_bash {
        return EmitPath::Primitive;
    }
    EmitPath::Primitive
}
