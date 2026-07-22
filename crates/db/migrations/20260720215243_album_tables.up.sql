CREATE TYPE album_type AS ENUM ('album', 'single', 'extended_play', 'compilation', 'other');

CREATE TABLE IF NOT EXISTS albums (
    id TEXT PRIMARY KEY,
    created_at TIMESTAMPTZ NOT NULL,
    created_by TEXT REFERENCES users(id),
    updated_at TIMESTAMPTZ,
    updated_by TEXT REFERENCES users(id),
    album_type album_type NOT NULL,
    title TEXT NOT NULL,
    release_date TIMESTAMPTZ NOT NULL
);
