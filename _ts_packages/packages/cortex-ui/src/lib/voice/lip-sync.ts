/**
 * Play a TTS blob through a hidden `<audio>` element while streaming
 * per-frame RMS amplitude into `onRms(value01)`. The consumer feeds that
 * value into Live2D's `ParamMouthOpenY` to drive lip-sync.
 *
 * Cleanup: every run revokes its object URL, closes its AudioContext, and
 * emits a final `onRms(0)` so the mouth returns to rest even if the caller
 * swaps audio mid-playback.
 *
 * Constructor Pattern: 1 file, 1 entry point (`playWithLipSync`). Helpers
 * are small private functions, not classes.
 */

import type { RmsCallback } from './types';

/** Amplify the raw RMS. Human speech in a 256-sample window rarely exceeds
 *  ~0.25 RMS; ×4 brings typical peaks to ~1.0 which visually matches
 *  anime-style "mouth wide open" without clipping on loud sibilants. */
const RMS_GAIN = 4;

interface CtxHandle {
  ctx: AudioContext;
  analyser: AnalyserNode;
  buf: Uint8Array;
}

function create_audio_ctx(audio: HTMLAudioElement): CtxHandle {
  type AudioCtxCtor = new () => AudioContext;
  const Ctor =
    (globalThis as { AudioContext?: AudioCtxCtor; webkitAudioContext?: AudioCtxCtor })
      .AudioContext ??
    (globalThis as { AudioContext?: AudioCtxCtor; webkitAudioContext?: AudioCtxCtor })
      .webkitAudioContext;
  if (!Ctor) throw new Error('AudioContext unavailable');
  const ctx = new Ctor();
  const source = ctx.createMediaElementSource(audio);
  const analyser = ctx.createAnalyser();
  analyser.fftSize = 256;
  source.connect(analyser);
  analyser.connect(ctx.destination);
  return { ctx, analyser, buf: new Uint8Array(analyser.fftSize) };
}

/** Compute the RMS of an analyser time-domain byte buffer (0..255, centre
 *  128), normalize to [-1, 1], gain, clamp. */
function rms_01(buf: Uint8Array): number {
  let sum = 0;
  for (let i = 0; i < buf.length; i++) {
    const v = (buf[i] - 128) / 128;
    sum += v * v;
  }
  const rms = Math.sqrt(sum / buf.length);
  const scaled = rms * RMS_GAIN;
  if (scaled < 0) return 0;
  if (scaled > 1) return 1;
  return scaled;
}

function close_ctx(h: CtxHandle | null): void {
  if (!h) return;
  try { void h.ctx.close(); } catch { /* noop */ }
}

interface Session {
  audio: HTMLAudioElement;
  url: string;
  handle: CtxHandle | null;
  raf: number;
}

function make_cleanup(s: Session, onRms: RmsCallback): () => void {
  return () => {
    if (s.raf) cancelAnimationFrame(s.raf);
    s.raf = 0;
    close_ctx(s.handle);
    s.handle = null;
    URL.revokeObjectURL(s.url);
    try { onRms(0); } catch { /* noop */ }
  };
}

function make_tick(s: Session, onRms: RmsCallback): () => void {
  const tick = (): void => {
    if (!s.handle) return;
    s.handle.analyser.getByteTimeDomainData(s.handle.buf);
    try { onRms(rms_01(s.handle.buf)); } catch { /* noop */ }
    s.raf = requestAnimationFrame(tick);
  };
  return tick;
}

/** Play `blob` and drive `onRms` every animation frame until the audio ends.
 *  Resolves when the `ended` event fires; always calls `onRms(0)` at exit. */
export async function playWithLipSync(
  blob: Blob,
  onRms: RmsCallback,
): Promise<void> {
  const url = URL.createObjectURL(blob);
  const audio = new Audio(url);
  // NOTE: no `audio.crossOrigin = 'anonymous'`. Object URLs are same-origin
  // by construction, and forcing CORS on them triggers a pointless opaque-
  // response path. If the call ever accepts a remote URL, add explicit CORS
  // at that site, not here.
  const session: Session = { audio, url, handle: null, raf: 0 };
  const cleanup = make_cleanup(session, onRms);
  const tick = make_tick(session, onRms);
  return new Promise<void>((resolve, reject) => {
    audio.addEventListener('ended', () => { cleanup(); resolve(); });
    audio.addEventListener('error', () => {
      cleanup();
      reject(new Error('audio playback failed'));
    });
    try { session.handle = create_audio_ctx(audio); }
    catch (err) {
      cleanup();
      reject(err instanceof Error ? err : new Error(String(err)));
      return;
    }
    audio.play()
      .then(() => { session.raf = requestAnimationFrame(tick); })
      .catch((err: unknown) => {
        cleanup();
        reject(err instanceof Error ? err : new Error(String(err)));
      });
  });
}
