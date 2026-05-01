//! Security — `server --host 0.0.0.0` MUST be rejected.
//!
//! `mlx_lm.server` defaults to localhost, but a careless user could
//! type `--host 0.0.0.0` to bind on every interface. This primitive
//! refuses with `Error::SecurityRefused` so a public bind requires an
//! explicit operator-level decision (config + audit), not a CLI typo.
//!
//! On non-target builds the platform gate fires first with NotSupported;
//! the test branches accordingly so it stays green on every host.

use kei_llm_mlx::error::Error;
use kei_llm_mlx::server::{build_spec, is_localhost};
use kei_llm_mlx::{is_supported, server::DEFAULT_HOST};

#[test]
fn rejects_zero_zero_zero_zero() {
    let s = is_supported();
    let res = build_spec("mlx-community/x-4bit", "0.0.0.0", 8080);
    if s.supported {
        assert!(matches!(res, Err(Error::SecurityRefused(_))));
    } else {
        assert!(matches!(res, Err(Error::NotSupported(_))));
    }
    assert!(is_localhost(DEFAULT_HOST));
    assert!(!is_localhost("0.0.0.0"));
    assert!(!is_localhost("192.168.1.10"));
    assert!(is_localhost("localhost"));
    assert!(is_localhost("::1"));
}
