/**
 * Microphone recorder factory. Wraps `getUserMedia` + `MediaRecorder` in a
 * push-to-talk friendly handle: one `start()`, one `stop()`, blob out, clean
 * up all MediaStream tracks so the browser mic indicator turns off.
 *
 * Constructor Pattern: 1 file, 1 factory, no class inheritance.
 */

import { PREFERRED_MIME, type RecorderHandle } from './types';

/** Pick `PREFERRED_MIME` when supported, else let the browser choose (''). */
function choose_mime(): string {
  const MR = (globalThis as { MediaRecorder?: typeof MediaRecorder })
    .MediaRecorder;
  if (MR && typeof MR.isTypeSupported === 'function') {
    if (MR.isTypeSupported(PREFERRED_MIME)) return PREFERRED_MIME;
  }
  return '';
}

/** Stop every track on the stream so the OS mic indicator goes off. */
function stop_tracks(stream: MediaStream | null): void {
  if (!stream) return;
  for (const t of stream.getTracks()) {
    try { t.stop(); } catch { /* noop */ }
  }
}

function build_blob(chunks: Blob[], mime: string): Blob {
  const type = mime || chunks[0]?.type || 'audio/webm';
  return new Blob(chunks, { type });
}

interface RecorderState {
  stream: MediaStream | null;
  recorder: MediaRecorder | null;
  chunks: Blob[];
  mime: string;
}

function reset(state: RecorderState): void {
  stop_tracks(state.stream);
  state.stream = null;
  state.recorder = null;
}

async function do_start(state: RecorderState): Promise<void> {
  if (state.recorder) throw new Error('recorder already started');
  state.mime = choose_mime();
  state.stream = await navigator.mediaDevices.getUserMedia({ audio: true });
  const opts: MediaRecorderOptions = state.mime ? { mimeType: state.mime } : {};
  const r = new MediaRecorder(state.stream, opts);
  r.ondataavailable = (ev: BlobEvent) => {
    if (ev.data && ev.data.size > 0) state.chunks.push(ev.data);
  };
  state.recorder = r;
  r.start();
}

function do_stop(state: RecorderState): Promise<Blob> {
  return new Promise<Blob>((resolve, reject) => {
    const r = state.recorder;
    if (!r) { reset(state); reject(new Error('recorder not started')); return; }
    r.onstop = () => { reset(state); resolve(build_blob(state.chunks, state.mime)); };
    r.onerror = (e: Event) => {
      reset(state);
      reject(e instanceof Error ? e : new Error('recorder error'));
    };
    try {
      if (r.state !== 'inactive') r.stop();
      else { reset(state); resolve(build_blob(state.chunks, state.mime)); }
    } catch (err) {
      reset(state);
      reject(err instanceof Error ? err : new Error(String(err)));
    }
  });
}

/** Create a fresh recorder handle. Each handle is single-shot: once `stop()`
 *  resolves, create a new one for the next PTT press. */
export function createRecorder(): RecorderHandle {
  const state: RecorderState = {
    stream: null, recorder: null, chunks: [], mime: '',
  };
  return {
    start: () => do_start(state),
    stop: () => do_stop(state),
  };
}
