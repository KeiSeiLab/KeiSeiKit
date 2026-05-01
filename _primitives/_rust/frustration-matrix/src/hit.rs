//! Normalised per-file parse product.
//!
//! Both markdown and jsonl parsers emit their own line types. The scan
//! loop only needs a common shape: `{file, line_no, text, timestamp?}`.
//! This cube is the only place that knows how to unify the two.
//!
//! `timestamp` is `Some` only for jsonl entries (runtime writes an
//! ISO 8601 `.timestamp` field). Markdown falls back to file mtime,
//! applied by the scan loop — keep this struct dumb.

use crate::jsonl::JsonlUserLine;
use crate::markdown::UserLine;

/// One candidate line for category matching.
pub struct Hit {
    pub file: String,
    pub line_no: usize,
    pub text: String,
    pub timestamp: Option<String>,
}

impl From<UserLine> for Hit {
    fn from(u: UserLine) -> Self {
        Hit {
            file: u.file,
            line_no: u.line_no,
            text: u.text,
            timestamp: None,
        }
    }
}

impl From<JsonlUserLine> for Hit {
    fn from(j: JsonlUserLine) -> Self {
        Hit {
            file: j.file.display().to_string(),
            line_no: j.line_no,
            text: j.text,
            timestamp: j.timestamp,
        }
    }
}
