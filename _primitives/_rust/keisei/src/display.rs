//! Display sanitization — ANSI/control-char stripper for user-facing output.
//!
//! Constructor Pattern: single responsibility — convert untrusted strings
//! (brain names, paths from manifests) into display-safe text by replacing
//! every ASCII control character (`< 0x20` or `== 0x7F`) with `?`. Space
//! (0x20) is preserved. Characters outside ASCII (emoji / unicode) pass
//! through unchanged — their UTF-8 bytes are all `> 0x7F` and never
//! collide with the control-byte range.
//!
//! Closes L9 (v0.19.2 audit): a malicious manifest
//! `name = "evil\x1b[2J..."` would clear the terminal or inject escape
//! sequences when `status` prints it. Every branch that prints
//! manifest-sourced text MUST route through `sanitize_display` first.

/// Replace every ASCII control character (`< 0x20` or `== 0x7F`) with `?`.
/// Space is preserved. Non-ASCII characters pass through unchanged.
pub fn sanitize_display(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if (c as u32) < 0x20 || c == '\x7F' {
            out.push('?');
        } else {
            out.push(c);
        }
    }
    out
}
