//! Round-trip tests for the formula API: register → load returns equal,
//! replacement is atomic, SHA-8 is deterministic + content-sensitive.

use kei_registry::formula::formula_sha;
use kei_registry::{
    load_formula, open_db, register, register_formula, BlockFormula, BlockType, EffectKind,
    FormulaSource, Predicate, SymbolKind, TypeAtom, TypeSignature,
};
use std::collections::BTreeSet;
use std::path::PathBuf;
use tempfile::tempdir;

fn unit_sig() -> TypeSignature {
    TypeSignature {
        inputs: vec![TypeAtom::Unit],
        output: TypeAtom::Unit,
        errors: vec![],
    }
}

fn fresh_db_block() -> (rusqlite::Connection, i64) {
    let tmp = tempdir().unwrap();
    let db_path = tmp.path().join("registry.sqlite");
    Box::leak(Box::new(tmp));
    let conn = open_db(&db_path).unwrap();
    let block = register(
        &conn,
        BlockType::Primitive,
        "demo",
        "_primitives/_rust/demo",
        b"body bytes",
        "demo:cap",
    )
    .unwrap();
    (conn, block.id)
}

fn empty_formula(block_id: i64) -> BlockFormula {
    BlockFormula {
        block_id,
        r#type: unit_sig(),
        invariants: vec![],
        effects: BTreeSet::new(),
        deps: BTreeSet::new(),
        source: FormulaSource::Declared,
    }
}

fn count_rows(conn: &rusqlite::Connection, table: &str, block_id: i64) -> i64 {
    let sql = format!("SELECT COUNT(*) FROM {} WHERE block_id = ?1", table);
    conn.query_row(&sql, rusqlite::params![block_id], |r| r.get(0))
        .unwrap()
}

#[test]
fn formula_roundtrip_minimal() {
    let (conn, id) = fresh_db_block();
    let formula = empty_formula(id);
    register_formula(&conn, &formula).unwrap();
    assert_eq!(load_formula(&conn, id).unwrap().unwrap(), formula);
}

#[test]
fn formula_load_returns_none_when_unset() {
    let (conn, id) = fresh_db_block();
    assert!(load_formula(&conn, id).unwrap().is_none());
}

#[test]
fn formula_roundtrip_with_predicates_and_deps() {
    let (conn, id) = fresh_db_block();
    let mut effects = BTreeSet::new();
    effects.insert(EffectKind::FsRead {
        glob: "src/**/*.rs".into(),
    });
    effects.insert(EffectKind::Stdout);
    effects.insert(EffectKind::Exec {
        binary: "cargo".into(),
    });
    let mut deps = BTreeSet::new();
    deps.insert("primitive::core::aaaaaaaa::bbbbbbbb-cccccccc".to_string());
    deps.insert("primitive::util::dddddddd::eeeeeeee-ffffffff".to_string());
    let invariants = vec![
        Predicate::FileExists {
            path: PathBuf::from("README.md"),
        },
        Predicate::ContentRegex {
            file: PathBuf::from("Cargo.toml"),
            pattern: r#"^name = "demo"$"#.into(),
            min: 1,
            max: Some(1),
        },
        Predicate::SymbolDeclared {
            file: PathBuf::from("src/lib.rs"),
            name: "run".into(),
            symbol_kind: SymbolKind::Fn,
        },
    ];
    let formula = BlockFormula {
        block_id: id,
        r#type: unit_sig(),
        invariants,
        effects,
        deps,
        source: FormulaSource::Inferred { confidence: 80 },
    };
    register_formula(&conn, &formula).unwrap();
    assert_eq!(load_formula(&conn, id).unwrap().unwrap(), formula);
}

#[test]
fn formula_replace_atomically() {
    let (conn, id) = fresh_db_block();
    let mut deps_a = BTreeSet::new();
    deps_a.insert("primitive::a::aaaaaaaa::bbbbbbbb-cccccccc".to_string());
    let formula_a = BlockFormula {
        invariants: vec![Predicate::FileExists {
            path: PathBuf::from("a.rs"),
        }],
        deps: deps_a,
        ..empty_formula(id)
    };
    register_formula(&conn, &formula_a).unwrap();

    let mut effects_b = BTreeSet::new();
    effects_b.insert(EffectKind::Stderr);
    let formula_b = BlockFormula {
        invariants: vec![Predicate::BodyShaEq {
            sha8: "deadbeef".into(),
        }],
        effects: effects_b,
        source: FormulaSource::Hybrid,
        ..empty_formula(id)
    };
    register_formula(&conn, &formula_b).unwrap();

    assert_eq!(load_formula(&conn, id).unwrap().unwrap(), formula_b);
    assert_eq!(count_rows(&conn, "block_predicates", id), 1);
    assert_eq!(count_rows(&conn, "block_deps", id), 0);
}

#[test]
fn formula_sha_deterministic() {
    let f1 = empty_formula(7);
    let f2 = BlockFormula { block_id: 99, ..f1.clone() };
    assert_eq!(formula_sha(&f1), formula_sha(&f2));

    let mut deps = BTreeSet::new();
    deps.insert("primitive::x::aaaaaaaa::bbbbbbbb-cccccccc".to_string());
    let f3 = BlockFormula { deps, ..f1.clone() };
    assert_ne!(formula_sha(&f1), formula_sha(&f3));
}

#[test]
fn formula_sha_persisted_matches_recomputed() {
    let (conn, id) = fresh_db_block();
    let formula = empty_formula(id);
    register_formula(&conn, &formula).unwrap();
    let stored: String = conn
        .query_row(
            "SELECT formula_sha FROM blocks WHERE id = ?1",
            rusqlite::params![id],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(stored, formula_sha(&formula));
}
