//! fal.ai Flux 2 Pro client — stylize a portrait into an anime frame.
//!
//! Public surface: `stylize(source_png, style)`.
//!
//! Pipeline: (1) upload source image to fal storage, (2) POST generation
//! request to the queue endpoint, (3) poll status until `COMPLETED`, (4)
//! download the first result image.  60-second wall-clock budget; past that
//! the function returns `Error::Timeout` so the handler can surface HTTP 504.
//!
//! `FAL_KEY` is read from the environment on every call (env-rotation-friendly,
//! per RULE 0.8 — never hardcoded, never cached in daemon state).
//!
//! HTTP pipeline steps live in `fal_pipeline`; SSRF mitigation in `fal_ssrf`.

use std::time::Duration;

/// Style preset — drives the prompt and nothing else.
#[derive(Debug, Clone, Copy)]
pub enum Style {
    Cute,
    Cool,
    Studious,
}

impl Style {
    /// Parse the wire-level string. Unknown values fall back to `Cute`.
    pub fn from_wire(s: &str) -> Self {
        match s {
            "anime-cool" => Self::Cool,
            "anime-studious" => Self::Studious,
            _ => Self::Cute,
        }
    }

    pub(crate) fn prompt(self) -> &'static str {
        match self {
            Self::Cute => CUTE_PROMPT,
            Self::Cool => COOL_PROMPT,
            Self::Studious => STUDIOUS_PROMPT,
        }
    }
}

const CUTE_PROMPT: &str = "soft pastel anime character, round expressive eyes, gentle smile, kawaii aesthetic, warm pink and peach tones, clean line art, centered head-and-shoulders portrait, plain neutral background, facing forward";
const COOL_PROMPT: &str = "anime character with sharp angular features, cool blue and slate color palette, confident calm expression, stylish modern outfit, crisp line art, centered head-and-shoulders portrait, plain neutral background, facing forward";
const STUDIOUS_PROMPT: &str = "anime character wearing round glasses, neat hair, studious calm expression, academic sweater or blazer, soft even lighting, muted color palette, centered head-and-shoulders portrait, plain neutral background, facing forward";

/// Errors surfaced to the caller; handlers map them onto HTTP codes.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("FAL_KEY environment variable not set")]
    NoApiKey,
    #[error("fal http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("fal returned status {0}: {1}")]
    BadStatus(u16, String),
    #[error("fal response missing field {0}")]
    BadShape(&'static str),
    #[error("fal polling exceeded 60s budget")]
    Timeout,
    #[error("non-fal URL rejected (SSRF): {0}")]
    NonFalUrl(String),
}

pub(crate) const BUDGET: Duration = Duration::from_secs(60);
pub(crate) const POLL_INTERVAL: Duration = Duration::from_millis(800);
pub(crate) const BODY_PREVIEW_CAP: usize = 512;
pub(crate) const ERROR_BODY_CAP: usize = 16 * 1024;
pub(crate) const IMAGE_BODY_CAP: usize = 64 * 1024 * 1024;

/// Stylize `source_png` into an anime portrait. Returns raw PNG bytes.
pub async fn stylize(source_png: &[u8], style: Style) -> Result<Vec<u8>, Error> {
    let key = std::env::var("FAL_KEY").map_err(|_| Error::NoApiKey)?;
    let deadline = tokio::time::Instant::now() + BUDGET;
    let uploaded_url = crate::fal_pipeline::upload_image(&key, source_png).await?;
    let status_url = crate::fal_pipeline::enqueue(&key, &uploaded_url, style).await?;
    let result_url =
        crate::fal_pipeline::poll_until_done(&key, &status_url, deadline, POLL_INTERVAL)
            .await?;
    crate::fal_pipeline::download_image(&key, &result_url).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_from_wire_defaults_to_cute() {
        assert!(matches!(Style::from_wire("wat"), Style::Cute));
    }

    #[test]
    fn style_from_wire_cool() {
        assert!(matches!(Style::from_wire("anime-cool"), Style::Cool));
    }

    #[test]
    fn style_from_wire_studious() {
        assert!(matches!(Style::from_wire("anime-studious"), Style::Studious));
    }
}
