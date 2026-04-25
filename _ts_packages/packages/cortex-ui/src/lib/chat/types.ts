// Chat domain types shared across sse-consumer, history-store, and the
// ChatPanel UI. Discriminated unions keep the event pipeline type-safe
// from raw SSE line → onEvent dispatch → store reducer.

export type SentimentTag =
  | 'happy'
  | 'sad'
  | 'angry'
  | 'think'
  | 'surprised'
  | 'awkward'
  | 'question'
  | 'curious'
  | 'neutral';

export const SENTIMENT_TAGS: readonly SentimentTag[] = [
  'happy',
  'sad',
  'angry',
  'think',
  'surprised',
  'awkward',
  'question',
  'curious',
  'neutral',
] as const;

export interface ChatTokenEvent {
  type: 'token';
  text: string;
}

export interface ChatSentimentEvent {
  type: 'sentiment';
  tag: SentimentTag;
  confidence: number;
}

export interface ChatDoneEvent {
  type: 'done';
  conversation_id: string;
}

export interface ChatErrorEvent {
  type: 'error';
  message: string;
}

export type ChatEvent =
  | ChatTokenEvent
  | ChatSentimentEvent
  | ChatDoneEvent
  | ChatErrorEvent;

/** One chat bubble. `pending` means the pet is still streaming tokens. */
export interface ChatMsg {
  id: string;
  role: 'user' | 'pet';
  text: string;
  ts: number;
  sentiment?: SentimentTag;
  pending?: boolean;
}
