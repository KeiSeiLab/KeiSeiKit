//! Function-body extractor used by `loc_check`.
//!
//! Manual brace-depth scan. We avoid `proc_macro2::Span::start()`
//! (feature-gated) so the scanner stays portable on stable.

/// Scan a Rust source for `fn <name>(` declarations and return for
/// each: (name, start-line, body-line-count). Body line count is
/// inclusive of the lines containing `{` and `}`. Brace counting
/// ignores braces inside string and char literals and line comments.
pub fn scan_fn_bodies(text: &str) -> Vec<(String, usize, usize)> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0;
    let mut line = 1usize;
    while i < bytes.len() {
        if bytes[i] == b'\n' {
            line += 1;
            i += 1;
            continue;
        }
        if let Some((name, j)) = match_fn_decl(bytes, i) {
            if let Some((open_idx, body_start_line)) = find_open_brace(bytes, j, line) {
                if let Some(end_line) = match_brace_pair(bytes, open_idx, body_start_line) {
                    let body_lines = end_line.saturating_sub(body_start_line) + 1;
                    out.push((name, body_start_line, body_lines));
                }
            }
        }
        i += 1;
    }
    out
}

/// If `bytes[i..]` begins with `fn <ident>(`, return (ident, idx_after_ident).
fn match_fn_decl(bytes: &[u8], i: usize) -> Option<(String, usize)> {
    if i + 3 > bytes.len() || &bytes[i..i + 3] != b"fn " {
        return None;
    }
    if i > 0 && is_ident_byte(bytes[i - 1]) {
        return None;
    }
    let mut j = i + 3;
    while j < bytes.len() && bytes[j] == b' ' {
        j += 1;
    }
    let start = j;
    while j < bytes.len() && is_ident_byte(bytes[j]) {
        j += 1;
    }
    if j == start {
        return None;
    }
    let name = std::str::from_utf8(&bytes[start..j]).ok()?.to_string();
    Some((name, j))
}

fn find_open_brace(bytes: &[u8], from: usize, mut line: usize) -> Option<(usize, usize)> {
    let mut i = from;
    let mut depth_paren: i32 = 0;
    let mut depth_angle: i32 = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'\n' => line += 1,
            b'(' => depth_paren += 1,
            b')' => depth_paren -= 1,
            b'<' => depth_angle += 1,
            b'>' if depth_angle > 0 => depth_angle -= 1,
            b'{' if depth_paren == 0 => return Some((i, line)),
            b';' if depth_paren == 0 => return None,
            _ => {}
        }
        i += 1;
    }
    None
}

fn match_brace_pair(bytes: &[u8], open_idx: usize, mut line: usize) -> Option<usize> {
    let mut st = ScanState::default();
    let mut i = open_idx;
    while i < bytes.len() {
        let b = bytes[i];
        if b == b'\n' {
            line += 1;
            st.in_line_comment = false;
            i += 1;
            continue;
        }
        if st.in_line_comment {
            i += 1;
            continue;
        }
        if st.starts_line_comment(bytes, i) {
            st.in_line_comment = true;
            i += 2;
            continue;
        }
        if let Some(d) = st.consume_brace(bytes, i, b) {
            if d == 0 {
                return Some(line);
            }
        }
        i += 1;
    }
    None
}

#[derive(Default)]
struct ScanState {
    depth: i32,
    in_str: bool,
    in_char: bool,
    in_line_comment: bool,
}

impl ScanState {
    fn starts_line_comment(&self, bytes: &[u8], i: usize) -> bool {
        !self.in_str
            && !self.in_char
            && bytes[i] == b'/'
            && i + 1 < bytes.len()
            && bytes[i + 1] == b'/'
    }

    /// Returns Some(new_depth) when a `{` or `}` was consumed at top
    /// level (outside string/char). Returns None otherwise.
    fn consume_brace(&mut self, bytes: &[u8], i: usize, b: u8) -> Option<i32> {
        if !self.in_char && b == b'"' && (i == 0 || bytes[i - 1] != b'\\') {
            self.in_str = !self.in_str;
            return None;
        }
        if !self.in_str && b == b'\'' && (i == 0 || bytes[i - 1] != b'\\') {
            self.in_char = !self.in_char;
            return None;
        }
        if self.in_str || self.in_char {
            return None;
        }
        if b == b'{' {
            self.depth += 1;
            return Some(self.depth);
        }
        if b == b'}' {
            self.depth -= 1;
            return Some(self.depth);
        }
        None
    }
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
