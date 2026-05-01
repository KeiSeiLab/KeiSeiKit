//! matcher unit tests — extracted to keep src/matcher.rs ≤200 LOC.

use kei_import_project::{match_module, ModuleSource, TraitKind};
use std::path::PathBuf;

fn src(content: &str) -> ModuleSource {
    ModuleSource::from_content("test", vec![(PathBuf::from("lib.rs"), content.to_owned())])
}

#[test]
fn memory_backend_methods_match() {
    let source = src(
        "impl MemoryBackend for S { fn backend_name(&self) {} fn store(&self) {} fn query(&self) {} fn compact(&self) {} fn mirror_to_remote(&self) {} }"
    );
    let matches = match_module(&source);
    let mem = matches.iter().find(|m| m.kind == TraitKind::MemoryBackend);
    assert!(mem.is_some(), "MemoryBackend should match");
    assert!(mem.unwrap().confidence >= 0.5);
}

#[test]
fn empty_source_returns_empty() {
    let source = ModuleSource::from_content("empty", vec![]);
    assert!(match_module(&source).is_empty());
}

#[test]
fn results_sorted_descending() {
    let source = src(
        "impl MemoryBackend for S { fn backend_name(&self) {} fn store(&self) {} fn query(&self) {} fn compact(&self) {} fn mirror_to_remote(&self) {} }"
    );
    let matches = match_module(&source);
    for w in matches.windows(2) {
        assert!(w[0].confidence >= w[1].confidence);
    }
}
