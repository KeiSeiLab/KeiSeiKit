//! skeleton — generate Rust impl-skeleton files from a TraitKind.
//!
//! Public entry-point: `render_skeleton`. Static trait metadata lives in
//! `skeleton_table` to keep this file ≤200 LOC.
//!
//! Constructor Pattern: one responsibility, ≤200 LOC, ≤30 LOC per fn.

use crate::skeleton_table::trait_meta;
use crate::trait_patterns::TraitKind;

/// Generate a Rust impl-skeleton for `module_name` implementing `target_trait`.
///
/// Output is a String containing valid-looking (but unimplemented) Rust source
/// with one `unimplemented!()` body per trait method and a TODO comment per
/// method describing expected behaviour.
pub fn render_skeleton(module_name: &str, target_trait: TraitKind) -> String {
    let meta = trait_meta(target_trait);
    let type_name = module_name_to_type(module_name);
    let mut out = String::with_capacity(2048);
    write_header(&mut out, module_name, target_trait, meta.trait_name);
    write_imports(&mut out, meta.use_imports);
    write_struct_stub(&mut out, &type_name);
    write_dna_impl(&mut out, &type_name);
    write_trait_impl(&mut out, meta, &type_name, module_name);
    out
}

fn write_header(out: &mut String, module_name: &str, kind: TraitKind, trait_name: &str) {
    out.push_str(&format!(
        "// AUTO-GENERATED skeleton for {module_name} \u{2192} {kind:?}\n\
         // TODO: replace `unimplemented!()` with real impl. Verify the trait\n\
         // signature matches your kei-runtime-core version.\n\n\
         // Trait: {trait_name}\n\n"
    ));
}

fn write_imports(out: &mut String, imports: &str) {
    out.push_str(imports);
    out.push('\n');
}

fn write_struct_stub(out: &mut String, type_name: &str) {
    out.push_str(&format!(
        "pub struct {type_name}; // TODO: rename + add fields\n\n"
    ));
}

fn write_dna_impl(out: &mut String, type_name: &str) {
    out.push_str(&format!("impl kei_runtime_core::dna::HasDna for {type_name} {{\n"));
    out.push_str(&format!("    fn dna(&self) -> &kei_runtime_core::dna::Dna {{\n"));
    out.push_str(&format!("        unimplemented!(\"HasDna::dna for {type_name}\")\n"));
    out.push_str("    }\n");
    out.push_str(&format!("    fn parent_dna(&self) -> Option<&kei_runtime_core::dna::Dna> {{\n"));
    out.push_str(&format!("        unimplemented!(\"HasDna::parent_dna for {type_name}\")\n"));
    out.push_str("    }\n");
    out.push_str("}\n\n");
}

fn write_trait_impl(
    out: &mut String,
    meta: &crate::skeleton_table::TraitMeta,
    type_name: &str,
    module_name: &str,
) {
    out.push_str(&format!(
        "#[async_trait::async_trait]\nimpl {} for {} {{\n",
        meta.trait_name, type_name
    ));
    for m in meta.methods {
        out.push_str(&format!(
            "    // TODO: {}\n    {}\n        unimplemented!(\"{}::{} for {}\")\n    }}\n\n",
            m.todo_hint, m.sig, meta.trait_name, m.name, module_name
        ));
    }
    out.push_str("}\n");
}

/// Convert kebab-case module name to PascalCase with `Foreign` prefix.
///
/// `kei-foreign-store` → `ForeignKeiForeignStore`
pub fn module_name_to_type(module_name: &str) -> String {
    let pascal: String = module_name
        .split('-')
        .map(capitalize_first)
        .collect();
    format!("Foreign{pascal}")
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_name_conversion_kebab() {
        assert_eq!(module_name_to_type("kei-foreign-store"), "ForeignKeiForeignStore");
        assert_eq!(module_name_to_type("my-backend"), "ForeignMyBackend");
        assert_eq!(module_name_to_type("foo"), "ForeignFoo");
    }

    #[test]
    fn render_contains_impl_keyword() {
        let out = render_skeleton("kei-backend-daytona", TraitKind::ComputeProvider);
        assert!(out.contains("impl"), "missing impl keyword");
    }

    #[test]
    fn render_contains_unimplemented() {
        let out = render_skeleton("kei-backend-daytona", TraitKind::ComputeProvider);
        assert!(out.contains("unimplemented!("), "missing unimplemented!()");
    }

    #[test]
    fn render_contains_async_fn() {
        let out = render_skeleton("kei-backend-daytona", TraitKind::ComputeProvider);
        assert!(out.contains("async fn"), "missing async fn");
    }

    #[test]
    fn all_kinds_render_without_panic() {
        for &kind in TraitKind::all() {
            let out = render_skeleton("test-module", kind);
            assert!(!out.is_empty(), "empty output for {kind:?}");
        }
    }
}
