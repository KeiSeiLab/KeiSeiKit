/**
 * Shared types for the browser-side voice pipeline (STT / TTS / lip-sync).
 *
 * Kept intentionally small: every voice module either consumes an `RmsCallback`
 * or exposes a tiny async primitive. Richer domain types (audio format
 * negotiation, server error payloads) belong next to the server contract,
 * not here.
 */

/** Called by the lip-sync loop once per animation frame with an
 *  already-normalized + clamped amplitude in `[0, 1]`. `0` == mouth closed. */
export type RmsCallback = (value01: number) => void;

/** Small factory-returned handle used by `createRecorder()`. */
export interface RecorderHandle {
  /** Request microphone, open a MediaRecorder, start capturing. */
  start(): Promise<void>;
  /** Stop capture, return the concatenated audio blob. Always cleans up. */
  stop(): Promise<Blob>;
}

/** Standard recorder mime we try first; spec-mandated by the daemon STT. */
export const PREFERRED_MIME = 'audio/webm;codecs=opus';
