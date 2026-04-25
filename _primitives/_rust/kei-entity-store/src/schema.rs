//! EntitySchema — declarative description of one entity table.
//!
//! A sibling crate (e.g. kei-task) defines a `static EntitySchema` and
//! passes a reference into every verb call. The engine reads this
//! structure to know: table name, fields to INSERT/SELECT, FTS columns,
//! edge table (for link/rank), and which verbs are enabled.

pub use crate::field::FieldDef;

/// Field kinds the engine knows how to bind for INSERT / UPDATE and
/// how to read in SELECT. A field's `kind` also drives the CREATE TABLE
/// DDL produced by the engine's migration runner.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    /// INTEGER PRIMARY KEY — exactly one PK per schema. Name = "id".
    IntegerPk,
    /// TEXT PRIMARY KEY — caller supplies the PK value (e.g. UUID).
    /// Mutually exclusive with `IntegerPk` within a single schema.
    TextPk,
    /// INTEGER NOT NULL (with optional DEFAULT 0).
    IntegerNotNull,
    /// INTEGER, default 0.
    Integer,
    /// TEXT NOT NULL (no default).
    TextNotNull,
    /// TEXT with empty-string default.
    Text,
    /// TEXT NOT NULL with explicit default value (held in `default`).
    TextDefault,
    /// TEXT NOT NULL representing a soft-delete enum with named
    /// sentinel values (`active` / `archived`). When used as the
    /// schema's `archived_field`, the `archive` verb writes the
    /// `archived` sentinel instead of flipping an integer.
    /// Default at insert = `active` sentinel.
    TextArchiveEnum,
    /// REAL (f64) NOT NULL, default 0.0.
    Real,
    /// REAL (f64) NOT NULL with an explicit default (held in
    /// `real_default`).
    RealDefault,
    /// Unix-timestamp INTEGER auto-stamped on insert (created_at).
    TimestampCreated,
    /// Unix-timestamp INTEGER auto-stamped on insert + update (updated_at).
    TimestampUpdated,
}

/// Edge-key storage strategy for the schema's `edge_table`.
///
/// - `IntegerPair` (default) — legacy `(from_id INTEGER, to_id INTEGER,
///   edge_type TEXT)` — matches kei-task byte-for-byte.
/// - `TextPair` — `(src_path TEXT, dst_path TEXT, edge_type TEXT)` —
///   required by kei-sage (composite text keys, no integer ids).
/// - `TextPairWithMetadata` — same text key but with optional
///   `id`/`weight`/`created_at` columns plus caller-controlled key
///   column names (`from_col`/`to_col`) and arbitrary extra columns
///   (kei-chat-store cross-refs, kei-content-store citations,
///   kei-crossdomain typed edges with evidence/metadata).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EdgeKeyKind {
    #[default]
    IntegerPair,
    TextPair,
    /// Extended text-pair edge with optional metadata columns and
    /// caller-controlled column names. Existing `TextPair` stays
    /// backward-compat (uses fixed `src_path`/`dst_path`).
    TextPairWithMetadata {
        /// Name of the "from" TEXT key column. Defaults to `"src_path"`
        /// for continuity with `TextPair` — override to e.g. `"from_uri"`
        /// for kei-crossdomain.
        from_col: &'static str,
        /// Name of the "to" TEXT key column. Defaults to `"dst_path"`.
        to_col: &'static str,
        /// Emit `edge_id INTEGER PRIMARY KEY AUTOINCREMENT` column.
        has_id: bool,
        /// Emit `weight REAL NOT NULL DEFAULT 1.0` column.
        has_weight: bool,
        /// Emit `created_at INTEGER NOT NULL` column auto-stamped on
        /// insert.
        has_created_at: bool,
        /// Extra typed columns appended after the standard metadata.
        /// Each `(name, kind)` pair produces a column using the same
        /// DDL rules as entity fields (`Text` → `TEXT DEFAULT ''`,
        /// `TextDefault` is not supported here — use `Text` with a
        /// caller-side default migration if a non-empty default is
        /// needed). `link` verb accepts matching JSON keys and binds
        /// them; `rank` ignores them.
        extra_columns: &'static [(&'static str, FieldKind)],
    },
}

impl EdgeKeyKind {
    /// True if this edge variant uses TEXT keys (any text variant).
    pub fn is_text(&self) -> bool {
        matches!(
            self,
            EdgeKeyKind::TextPair | EdgeKeyKind::TextPairWithMetadata { .. }
        )
    }
}

/// Declarative schema for one entity.
#[derive(Debug, Clone, Copy)]
pub struct EntitySchema {
    /// Human-readable entity name — used in error messages.
    pub name: &'static str,
    /// SQL table name for the primary entity rows.
    pub table: &'static str,
    /// Column order — MUST start with the PK.
    pub fields: &'static [FieldDef],
    /// Verb whitelist — e.g. ["create","get","search","update","delete"].
    pub enabled_verbs: &'static [&'static str],
    /// If `Some`, engine creates an FTS5 virtual table `fts_<table>`
    /// with the listed non-id columns and keeps it in sync on create
    /// + update. `search` verb uses it.
    pub fts_columns: Option<&'static [&'static str]>,
    /// If `Some`, engine creates `<edge_table>` for the `link` verb.
    /// Column layout depends on `edge_key_kind`. `rank` verb runs
    /// PageRank over it.
    pub edge_table: Option<&'static str>,
    /// Edge-table key layout. Default `IntegerPair` preserves legacy
    /// `(from_id, to_id)` schema; `TextPair` switches to
    /// `(src_path, dst_path)` for path-keyed graphs (kei-sage).
    pub edge_key_kind: EdgeKeyKind,
    /// If `Some`, enables the `archive` verb. Names the column used as
    /// the soft-delete marker. If the column's kind is `TextArchiveEnum`
    /// the verb writes the `archived` sentinel; otherwise (integer
    /// column) it flips to 1. In both cases a sibling `<field>_at`
    /// INTEGER column is stamped with the current Unix timestamp if
    /// present in `fields`.
    pub archived_field: Option<&'static str>,
    /// Arbitrary DDL statements run after the primary table + FTS +
    /// edge table have been created. Used for secondary tables
    /// (milestones, task_deps) that piggy-back on the same DB but are
    /// task-specific (not generic-CRUD).
    pub custom_migrations: &'static [&'static str],
}

impl EntitySchema {
    /// Returns the PK column (integer or text). Panics if the schema
    /// has no PK — schema authors must declare exactly one.
    pub fn pk(&self) -> &FieldDef {
        self.fields
            .iter()
            .find(|f| f.is_pk())
            .expect("EntitySchema MUST have exactly one PK field (IntegerPk or TextPk)")
    }

    /// Returns true if `verb` appears in `enabled_verbs`.
    pub fn verb_enabled(&self, verb: &str) -> bool {
        self.enabled_verbs.contains(&verb)
    }

    /// Returns the list of non-PK field names, in order. Used by the
    /// `create` verb to build the INSERT column-list.
    pub fn writable_fields(&self) -> impl Iterator<Item = &FieldDef> {
        self.fields.iter().filter(|f| !f.is_pk())
    }

    /// Look up a field by name.
    pub fn field(&self, name: &str) -> Option<&FieldDef> {
        self.fields.iter().find(|f| f.name == name)
    }
}
