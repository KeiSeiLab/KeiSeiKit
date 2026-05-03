//! Typed errors for the atom-invocation runtime.
//!
//! Constructor Pattern: extracted from `invoke.rs` so the runtime
//! parent file stays under 200 LOC after Wave 44d added bounded-read
//! capture + truncation handling.

#[derive(Debug)]
pub enum InvokeError {
    AtomNotFound(String),
    InputParse(String),
    InputInvalid(String),
    MissingInputSchema(String),
    /// `crate_name` in atom YAML failed the `kei-*` allowlist check.
    InvalidAtom(String),
    /// Crate binary is missing from both `KEI_RUNTIME_BIN_DIR` and `PATH`.
    BinaryNotFound { crate_name: String },
    /// Subprocess exited non-zero — propagate the atom's own exit code.
    AtomFailed { atom: String, code: i32, stderr: String },
    /// IO / spawn failure (not a non-zero exit from the child).
    SubprocessError(String),
    /// Atom's stdout was not parseable as JSON.
    OutputParse(String),
    /// Legacy escape — atom not yet migrated to `run-atom` protocol.
    NotImplemented { atom: String },
}

impl std::fmt::Display for InvokeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AtomNotFound(id) => write!(f, "no atom matching {id}"),
            Self::InputParse(e) => write!(f, "input rejected: {e}"),
            Self::InputInvalid(e) => write!(f, "input rejected: {e}"),
            Self::MissingInputSchema(id) => {
                write!(f, "atom `{id}` declares no input schema")
            }
            Self::InvalidAtom(msg) => write!(f, "invalid atom crate_name: {msg}"),
            Self::BinaryNotFound { crate_name } => write!(
                f,
                "binary `{crate_name}` not found on PATH or KEI_RUNTIME_BIN_DIR"
            ),
            Self::AtomFailed { atom, code, stderr } => {
                write!(f, "atom `{atom}` exited {code}: {stderr}")
            }
            Self::SubprocessError(e) => write!(f, "subprocess: {e}"),
            Self::OutputParse(e) => write!(f, "atom stdout not JSON: {e}"),
            Self::NotImplemented { atom } => write!(
                f,
                "invoke not yet wired for this atom ({atom}); use the underlying CLI directly"
            ),
        }
    }
}

impl std::error::Error for InvokeError {}
