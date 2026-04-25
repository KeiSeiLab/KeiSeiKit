export interface PetManifest {
  schema: number;
  identity: {
    pet_name: string;
    user_name: string;
    addressing: string;
    languages: string[];
  };
  voice: {
    tone_primary: string;
    tone_secondary: string[];
    humor_style: string;
    humor_frequency: string;
  };
  edge: {
    profanity: string;
    profanity_languages: string[];
    directness: string;
    initiative: string;
  };
  forbidden: {
    topics: string[];
    tone_patterns: string[];
  };
  meta: Record<string, unknown>;
}

export interface LedgerRow {
  id: string;
  dna: string | null;
  status: string;
  started_ts: number;
}

export interface Summary {
  total_dnas: number;
  active_pets: string[];
  ledger_last_ts: number | null;
  recent_sessions: number;
}

export interface MemoryHit {
  id: number;
  role: string;
  text: string;
  ts: number;
}
