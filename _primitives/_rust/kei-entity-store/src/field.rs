//! `FieldDef` — one column in an `EntitySchema`. Split out of
//! `schema.rs` to keep both files under the Constructor-Pattern
//! 200-LOC cap.

use crate::schema::FieldKind;

/// One column in an EntitySchema.
#[derive(Debug, Clone, Copy)]
pub struct FieldDef {
    pub name: &'static str,
    pub kind: FieldKind,
    /// Default literal for TextDefault / IntegerNotNull (as SQL literal
    /// WITHOUT surrounding quotes — engine quotes TEXT automatically).
    pub default: Option<&'static str>,
    /// Emit a single-column index `idx_<table>_<name>`.
    pub indexed: bool,
    /// Default for `Real` / `RealDefault` columns. `None` means 0.0.
    pub real_default: Option<f64>,
    /// Sentinel pair for `TextArchiveEnum` — `(active, archived)`.
    /// Ignored for other kinds. `None` falls back to
    /// `("active", "archived")`.
    pub archive_enum: Option<(&'static str, &'static str)>,
}

impl FieldDef {
    pub const fn pk(name: &'static str) -> Self {
        Self::base(name, FieldKind::IntegerPk)
    }
    pub const fn text_pk(name: &'static str) -> Self {
        Self::base(name, FieldKind::TextPk)
    }
    pub const fn text(name: &'static str) -> Self {
        Self::base(name, FieldKind::Text)
    }
    pub const fn text_nn(name: &'static str) -> Self {
        Self::base(name, FieldKind::TextNotNull)
    }
    pub const fn text_default(name: &'static str, default: &'static str) -> Self {
        let mut f = Self::base(name, FieldKind::TextDefault);
        f.default = Some(default);
        f
    }
    pub const fn integer(name: &'static str) -> Self {
        Self::base(name, FieldKind::Integer)
    }
    pub const fn integer_nn(name: &'static str) -> Self {
        Self::base(name, FieldKind::IntegerNotNull)
    }
    pub const fn real(name: &'static str) -> Self {
        Self::base(name, FieldKind::Real)
    }
    pub const fn real_default(name: &'static str, default: f64) -> Self {
        let mut f = Self::base(name, FieldKind::RealDefault);
        f.real_default = Some(default);
        f
    }
    pub const fn text_archive_enum(
        name: &'static str,
        active: &'static str,
        archived: &'static str,
    ) -> Self {
        let mut f = Self::base(name, FieldKind::TextArchiveEnum);
        f.archive_enum = Some((active, archived));
        f
    }
    pub const fn created_at() -> Self {
        Self::base("created_at", FieldKind::TimestampCreated)
    }
    pub const fn updated_at() -> Self {
        Self::base("updated_at", FieldKind::TimestampUpdated)
    }
    pub const fn with_index(mut self) -> Self {
        self.indexed = true;
        self
    }

    /// Internal base constructor — zeroes optional fields so the
    /// per-kind builders above stay one-liners.
    const fn base(name: &'static str, kind: FieldKind) -> Self {
        Self {
            name,
            kind,
            default: None,
            indexed: false,
            real_default: None,
            archive_enum: None,
        }
    }

    /// True if this FieldDef is a primary key (either integer or text).
    pub fn is_pk(&self) -> bool {
        matches!(self.kind, FieldKind::IntegerPk | FieldKind::TextPk)
    }
}
