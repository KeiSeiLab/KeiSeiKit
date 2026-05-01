CREATE TABLE users (
    id INTEGER NOT NULL,
    email TEXT NOT NULL,
    age INTEGER NOT NULL,
    bio TEXT
);

CREATE TABLE magic_tokens (
    token TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    expires_at TIMESTAMP NOT NULL
);
