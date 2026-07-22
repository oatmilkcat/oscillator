CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP,
    display_name TEXT,
    username TEXT NOT NULL,
    email TEXT NOT NULL,
    password_hash TEXT,
    password_reset_token TEXT
);

CREATE UNIQUE INDEX ix_users_username_unique ON users (username);
CREATE UNIQUE INDEX ix_users_email_unique ON users (email);
