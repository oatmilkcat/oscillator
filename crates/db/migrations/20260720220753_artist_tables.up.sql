CREATE TABLE IF NOT EXISTS artists (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    created_by TEXT REFERENCES users(id),
    updated_at TIMESTAMPTZ,
    updated_by TEXT REFERENCES users(id),
    name TEXT NOT NULL
);
