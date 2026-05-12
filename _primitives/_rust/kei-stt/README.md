# kei-stt

Speech-to-text abstraction crate with 3 backends selected at runtime via
`KEI_STT_BACKEND`. Default backend is **whisper-local** (free, local, no API key).

## Backend matrix

| Backend          | Feature flag     | Cost           | Latency     | Quality   |
|------------------|------------------|----------------|-------------|-----------|
| `whisper-local`  | `whisper-local`  | Free           | 1–10× RT    | Very good |
| `deepgram`       | `deepgram`       | ~$0.0043/min   | 200–500 ms  | Excellent |
| `openai-whisper` | `openai-whisper` | ~$0.006/min    | 300–800 ms  | Excellent |

RT = real-time factor (depends on hardware / model size for whisper-local).

## Environment variables

| Variable                   | Backend         | Required | Description                              |
|----------------------------|-----------------|----------|------------------------------------------|
| `KEI_STT_BACKEND`          | all             | No       | `whisper-local` (default) / `deepgram` / `openai-whisper` |
| `KEI_STT_WHISPER_BINARY`   | whisper-local   | No       | Path to `whisper` CLI (default: PATH)    |
| `KEI_STT_WHISPER_MODEL`    | whisper-local   | No       | Model name (default: `base.en`)          |
| `DEEPGRAM_API_KEY`         | deepgram        | Yes      | Deepgram API key                         |
| `OPENAI_API_KEY`           | openai-whisper  | Yes      | OpenAI API key                           |

## Usage

```toml
[dependencies]
kei-stt = { path = "../kei-stt", features = ["whisper-local"] }
```

```rust
#[tokio::main]
async fn main() -> Result<(), kei_stt::SttError> {
    let backend = kei_stt::from_env()?;
    let audio = std::fs::read("speech.wav").unwrap();
    let req = kei_stt::SttRequest::new_wav(audio);
    let resp = backend.transcribe(&req).await?;
    println!("[{}] {}", backend.name(), resp.text);
    for seg in &resp.segments {
        println!("  {:>6}ms–{:>6}ms  {}", seg.start_ms, seg.end_ms, seg.text);
    }
    Ok(())
}
```

## Compile-time features

```toml
# All backends:
kei-stt = { features = ["all-backends"] }
# Cloud only, no local whisper:
kei-stt = { features = ["deepgram", "openai-whisper"], default-features = false }
```

## whisper-local prerequisites

Install the `openai-whisper` Python package:

```sh
pip install openai-whisper
```

This makes the `whisper` CLI available. Alternatively point `KEI_STT_WHISPER_BINARY`
at a compatible binary (`faster-whisper`, etc. with identical CLI interface).
