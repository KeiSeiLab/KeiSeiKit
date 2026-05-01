// Match: type↔table for users
export type User = {
    id: number;
    email: string;
    age: string;       // INTENTIONAL drift: SQL is INTEGER
    phone: string;     // INTENTIONAL drift: orphan TS field
    bio: string | null;
};

// Match: type↔table for magic_tokens
export interface MagicToken {
    token: string;
    user_id: number;
    expires_at: string;
}
