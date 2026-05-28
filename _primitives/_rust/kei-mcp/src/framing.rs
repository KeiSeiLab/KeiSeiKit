//! Line framing for stdio JSON-RPC — bounded by `MAX_MESSAGE_BYTES`.
//!
//! MISS-8 hardening. A single JSON-RPC line is read with a hard 10 MB cap so
//! a malicious or runaway peer cannot OOM the server by sending one huge
//! line. The cap is enforced INCREMENTALLY — once `MAX_MESSAGE_BYTES`
//! payload bytes have been pulled into the buffer, we stop allocating and
//! only DRAIN bytes (without storing them) until the next newline. This
//! keeps the resident set bounded at ~10 MB per oversize event regardless
//! of how big the peer's line actually is.
//!
//! The reader must implement `AsyncBufRead` so we can use the buffered
//! `fill_buf`/`consume` interface to peek-and-drain without copying into
//! a growing `Vec<u8>` after the cap is hit.

use crate::error::Error;
use tokio::io::AsyncBufReadExt;

/// Hard cap on a single JSON-RPC line (10 MB). Anything larger is rejected
/// as a parse error so a malicious / runaway peer cannot OOM the server.
pub const MAX_MESSAGE_BYTES: u64 = 10 * 1024 * 1024;

/// One outcome of a single bounded read.
#[derive(Debug)]
pub enum ReadOutcome {
    /// Upstream is closed; caller should exit the loop.
    Eof,
    /// Line was empty / whitespace-only; caller should retry.
    Empty,
    /// Valid line within the cap.
    Line(String),
    /// Line exceeded `MAX_MESSAGE_BYTES`; the rest of the line has been
    /// drained without being stored. Reader is now on the next physical
    /// line (or at EOF).
    Oversize,
}

/// Read one line from `reader` with a hard `MAX_MESSAGE_BYTES` cap.
///
/// Behaviour:
/// - Up to `MAX_MESSAGE_BYTES` payload bytes are stored in the returned
///   buffer. The trailing `\n` (if any) is included.
/// - Once the cap is exceeded, every subsequent byte up to and including
///   the next `\n` is consumed but NOT stored. Memory stays bounded.
pub async fn read_capped_line<R: AsyncBufReadExt + Unpin>(
    reader: &mut R,
) -> Result<ReadOutcome, Error> {
    let mut buf: Vec<u8> = Vec::new();
    let mut over_cap = false;
    loop {
        let chunk = reader
            .fill_buf()
            .await
            .map_err(|e| Error::Stdin(format!("reading stdin: {e}")))?;
        if chunk.is_empty() {
            // EOF before any newline.
            break;
        }
        let nl_pos = chunk.iter().position(|b| *b == b'\n');
        let take_len = nl_pos.map(|p| p + 1).unwrap_or(chunk.len());
        if !over_cap {
            let remaining = (MAX_MESSAGE_BYTES as usize).saturating_sub(buf.len());
            let copy = take_len.min(remaining);
            buf.extend_from_slice(&chunk[..copy]);
            if take_len > remaining {
                over_cap = true;
            }
        }
        reader.consume(take_len);
        if nl_pos.is_some() {
            break;
        }
    }
    if buf.is_empty() && !over_cap {
        return Ok(ReadOutcome::Eof);
    }
    if over_cap {
        return Ok(ReadOutcome::Oversize);
    }
    let line = match String::from_utf8(buf) {
        Ok(s) => s,
        Err(e) => return Err(Error::InvalidUtf8(e.to_string())),
    };
    if line.trim().is_empty() {
        return Ok(ReadOutcome::Empty);
    }
    Ok(ReadOutcome::Line(line))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::io::{AsyncWriteExt, BufReader};

    #[tokio::test]
    async fn small_line_returns_line_outcome() {
        let (mut tx, rx) = tokio::io::duplex(64);
        let mut reader = BufReader::new(rx);
        tx.write_all(b"{\"hi\":1}\n").await.unwrap();
        drop(tx);
        let out = read_capped_line(&mut reader).await.unwrap();
        match out {
            ReadOutcome::Line(s) => assert_eq!(s.trim(), "{\"hi\":1}"),
            other => panic!("expected Line, got {other:?}"),
        }
    }

    #[tokio::test]
    async fn eof_outcome_on_closed_pipe() {
        let (tx, rx) = tokio::io::duplex(64);
        drop(tx);
        let mut reader = BufReader::new(rx);
        let out = read_capped_line(&mut reader).await.unwrap();
        assert!(matches!(out, ReadOutcome::Eof));
    }

    #[tokio::test]
    async fn whitespace_line_returns_empty_outcome() {
        let (mut tx, rx) = tokio::io::duplex(64);
        let mut reader = BufReader::new(rx);
        tx.write_all(b"   \n").await.unwrap();
        drop(tx);
        let out = read_capped_line(&mut reader).await.unwrap();
        assert!(matches!(out, ReadOutcome::Empty));
    }

    /// 11 MB line → cap hit → Oversize. Buffer never exceeds the cap.
    #[tokio::test]
    async fn oversize_line_resyncs_to_next_line() {
        let (mut tx, rx) = tokio::io::duplex(12 * 1024 * 1024);
        let mut reader = BufReader::new(rx);
        let huge: Vec<u8> = vec![b'x'; (MAX_MESSAGE_BYTES + 1024) as usize];
        let writer = tokio::spawn(async move {
            tx.write_all(&huge).await.unwrap();
            tx.write_all(b"\n").await.unwrap();
            tx.write_all(b"{\"ok\":1}\n").await.unwrap();
            tx.shutdown().await.unwrap();
        });
        let first = read_capped_line(&mut reader).await.unwrap();
        assert!(matches!(first, ReadOutcome::Oversize));
        let second = read_capped_line(&mut reader).await.unwrap();
        match second {
            ReadOutcome::Line(s) => assert_eq!(s.trim(), "{\"ok\":1}"),
            other => panic!("expected post-resync Line, got {other:?}"),
        }
        writer.await.unwrap();
    }

    /// Exactly-at-cap line is accepted (boundary).
    #[tokio::test]
    async fn at_cap_line_accepted() {
        let (mut tx, rx) = tokio::io::duplex(MAX_MESSAGE_BYTES as usize + 64);
        let mut reader = BufReader::new(rx);
        let mut payload: Vec<u8> = vec![b'a'; (MAX_MESSAGE_BYTES - 1) as usize];
        payload.push(b'\n');
        let writer = tokio::spawn(async move {
            tx.write_all(&payload).await.unwrap();
            tx.shutdown().await.unwrap();
        });
        let out = read_capped_line(&mut reader).await.unwrap();
        assert!(
            matches!(out, ReadOutcome::Line(_)),
            "exactly-at-cap line must be accepted (got {out:?})"
        );
        writer.await.unwrap();
    }
}
