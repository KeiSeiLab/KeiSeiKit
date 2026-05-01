//! Stream — NDJSON chunk parser.
//!
//! Three token chunks + a terminal `done` marker should yield 4 chunks
//! and `concat_chunks` should reconstruct the full string.

use kei_llm_mlx::stream::{concat_chunks, parse_stream};

const STREAM_SAMPLE: &str = r#"{"delta": "Hello", "tokens_so_far": 1}
{"delta": ", ", "tokens_so_far": 2}
{"delta": "world!", "tokens_so_far": 3}
{"delta": "", "done": true, "tokens_so_far": 3}
"#;

#[test]
fn ndjson_yields_n_plus_one_chunks() {
    let chunks = parse_stream(STREAM_SAMPLE).expect("parse");
    assert_eq!(chunks.len(), 4);
    assert!(chunks.last().unwrap().done);
    assert_eq!(chunks[0].tokens_so_far, Some(1));
    assert_eq!(concat_chunks(&chunks), "Hello, world!");
}

#[test]
fn non_json_lines_are_skipped() {
    let mixed = "loading model...\n{\"delta\": \"X\", \"tokens_so_far\": 1}\nready.\n";
    let chunks = parse_stream(mixed).expect("parse");
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0].delta, "X");
}
